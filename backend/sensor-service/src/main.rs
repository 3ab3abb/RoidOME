// Dependencies -------

use serde::Deserialize ; 
use std::fmt ; 
use thiserror::Error ;
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use tokio::sync::mpsc;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use sqlx::PgPool ;

// ----------------------


//Defining Custom Sensor Errors enum


#[derive(Debug,Error)]
enum SensorError { 

    #[error("Failed to parse the payload : {0}")]
    ParseError(#[from] serde_json::Error) ,


    #[error("Unknown Topic : {0}")]
    UnknownTopic(String) ,

    #[error("Missing Field: {0}")]
    MissingField(String) , 

}

//------------------------------------




// Humidity & Temperature  ---
#[derive(Debug ,Deserialize)]
struct TempHumidity {

    id:String ,
    temperature:f32,
    humidity:f32,
    timestamp:i64 ,

}
impl fmt::Display for TempHumidity { 
    fn fmt(&self , f:&mut fmt::Formatter) -> fmt::Result { 
        write!(
            f,
            "device : {} | temperature : {}°C |humidty : {}% , |timestamp : {}",
            self.id , self.temperature , self.humidity , self.timestamp 
              )
    }

}
//________________________________________________
// Motion Sensor  ---

#[derive(Debug,Deserialize)]
struct MotionSensor { 

    id:String ,
    motion :bool , 
    timestamp:i64 , 

} 



impl fmt::Display for MotionSensor {
    fn fmt(&self , f:&mut fmt::Formatter) -> fmt::Result { 

        write!(
            f,
            "Motion sensor with id {} , detection status is {} at {}" ,
            self.id , self.motion , self.timestamp
        )
    }
}
//________________________________________________

// Gas Sensor  ---
#[derive(Debug,Deserialize)]

struct GasSensor { 

    id:String ,
    gas_level :f32 , 
    timestamp:i64 , 

} 


impl fmt::Display for GasSensor { 

    fn fmt (&self , f:&mut fmt::Formatter) -> fmt::Result {


        write!(
            f,
            "Gas sensor with id {} , reads gas level {} at {}",
            self.id , self.gas_level , self.timestamp 
        )
    }
        

    }
//________________________________________________


//Parsing JSON Payloads
fn parse<T: for<'de> serde::Deserialize<'de>>(raw: &str) -> Result<T, serde_json::Error> { 

    serde_json::from_str(raw)

}


enum DeviceEvent { 

    TemperatureHumidity(TempHumidity) ,
    Motion(MotionSensor) ,
    Gas(GasSensor) , 
}

impl fmt::Display for DeviceEvent  { 

    fn fmt (&self  , f:&mut fmt::Formatter ) -> fmt::Result {
        
        match self { 

            DeviceEvent::TemperatureHumidity(r) => write!(f, "{}" , r) , 
            DeviceEvent::Motion(r) => write!(f,"{}" ,r ) , 
            DeviceEvent::Gas(r) => write!(f,"{}",r) , 
            

        }


    }

} 

impl DeviceEvent { 


    fn device_id(&self) -> &str { 

        match self { 
            
            DeviceEvent::TemperatureHumidity(r) => &r.id , 
            DeviceEvent::Motion(r) => &r.id ,
            DeviceEvent::Gas(r) => &r.id ,
        }
    }
}


// Router IMPLEMENTAION 
fn router(topic:&str , payload :&str ) -> Result<DeviceEvent , SensorError> { 

    match topic { 
        "home/sensors/temperature" => {
            let reading  = parse::<TempHumidity>(payload)? ; 
            Ok(DeviceEvent::TemperatureHumidity(reading))
        }

        "home/sensors/motion" => {
            let reading = parse::<MotionSensor>(payload)? ; 
            Ok(DeviceEvent::Motion(reading)) 
        }        
            
        "home/sensors/gas" => {
            let reading = parse::<GasSensor>(payload)? ; 
            Ok (DeviceEvent::Gas(reading)) 
        }
        
        unknown_topic => { 
                     
                return Err(SensorError::UnknownTopic(unknown_topic.to_string())) ; 
                
                }

    }




}


// Insert sensor readings to Database 


async fn insert_reading(
    pool: &PgPool,
    device_id: &str,
    sensor_type: &str,
    payload: &str,
) -> Result<(), sqlx::Error> {
    let payload_json: serde_json::Value = serde_json::from_str(payload)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO sensor_readings (device_id, sensor_type, payload)
         VALUES ($1, $2, $3)",
        device_id,
        sensor_type,
        payload_json,
    )
    .execute(pool)
    .await?;

    Ok(())
}
// Base64 Decoding from MQTT payload 
async fn ingest_from_mqtt(payload: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    
    let cleaned = payload.replace('\n', "").replace('\r', "");
    let bytes =  STANDARD.decode(&payload)?;
    Ok(bytes)


}
// Storing Frame on Disk and Base64 on Database , Sending to roidome-tui the file path 
async fn store_frame(
    pool : &PgPool,
    client: &AsyncClient,
    device_id : &str,
    payload: &str,
    source_type : &str,
    bytes :Vec<u8>,
)-> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let frames_dir = std::env::var("FRAMES_DIR").unwrap_or_else(|_| {
        std::env::current_dir()
            .unwrap()
            .join("frames")
            .to_string_lossy()
            .to_string()
    });

    let dir = format!("{}/{}/{}" ,frames_dir,source_type,device_id ) ; 
    let file_path = format!("{}/{}.jpg",dir,timestamp,) ; 
    tokio::fs::create_dir_all(&dir).await? ;  
    tokio::fs::write(&file_path,bytes).await?;         
     sqlx::query!(
        "INSERT INTO camera_frames (device_id, source_type,file_path,image_data)
         VALUES ($1, $2, $3, $4)",
        device_id,
        source_type,
        file_path,
        payload, 
    )
    .execute(pool)
    .await?;


   client
        .publish(
            "home/camera/snapshot",
            rumqttc::QoS::AtMostOnce,
            false,
            file_path.as_bytes(),
        )
        .await?;    

    Ok(())
}   





#[tokio::main]
async fn main() {

   //Database Pool Setup-----
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to PostgreSQL") ; 

   
    tracing_subscriber::fmt::init();

    let mqtt_host = std::env::var("MQTT_BROKER").unwrap_or_else(|_| "localhost".to_string());
   

    let mut mqttoptions = MqttOptions::new("roidome-backend", &mqtt_host, 1883);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let (tx, mut rx) = mpsc::channel(100);

        client.subscribe("home/#", QoS::AtMostOnce).await.unwrap();
   
    tokio::spawn(async move  { 
        
            loop { 


               
                match eventloop.poll().await { 
        
                    Ok(Event::Incoming(Packet::Publish(msg))) =>{
    
                        let topic = msg.topic.clone() ; 
                        let payload =String::from_utf8(msg.payload.to_vec()).unwrap() ;  
                        tx.send((topic, payload)).await.unwrap();
 

                    }
                    Err(e) => { 
                        
                        tracing::error!("MQTT ERROR : {}",e);
                        break ; 


                    }
                     _ => {}
            }
        } 

    }) ; 

        while let Some((topic, payload)) = rx.recv().await {
            
            if topic == "home/camera/frame" { 

                let pool = pool.clone() ; 
                let client = client.clone();
                let payload = payload.clone() ; 

                tokio::spawn (async move{
                    match ingest_from_mqtt(&payload).await {
                        Ok(bytes) => {

                            if let Err(e)  = store_frame(
                                &pool,
                                &client,
                                "esp-cam-01",
                                &payload,
                                "esp32cam",
                                bytes,
                                ).await { 

                                tracing::error!("Failed to store frame: {}",e) ; 
                            }
                                    
                        }
                        Err(e) => tracing::error!("Failed to frame: {}",e) , 
                    }
 
                }); 
                continue ;
            }


             if topic == "home/camera/snapshot" {
            continue;
            }

            match router(&topic, &payload) {
                Ok(event) => {
                    tracing::info!("Received: {}", event);
                    let sensor_type = topic.split('/').last().unwrap_or("unknown");
                    let result = insert_reading(&pool , &event.device_id(),&sensor_type,&payload ).await ; 
                    if let Err(e) = result { 
                        tracing::error!("Failed to insert reading: {}",e) ; 
                    }
                }
                Err(e) => tracing::error!("Error: {}", e),
                    
            } 
        }
}

