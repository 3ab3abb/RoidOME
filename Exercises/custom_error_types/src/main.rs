









use serde::Deserialize ; 
use std::fmt ; 

use thiserror::Error ; 


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


fn main() {





// Payloads example 
    let  temphum_raw = r#"
        {
            "id": "esp32_01",
            "temperature": 24.5,
            "humidity": 61.2,
            "timestamp": 1234567890
        }
    "#;


  let   motion_raw = r#"
        {
            "id": "esp32_01",
            "motion": true,
            "timestamp": 1234567890
        }
    "#;

    let  gas_raw = r#"
        {
            "id": "esp32_02",
            "gas_level": 12.34,
            "timestamp": 1234567890
        }
    "#;




//________________________________________________





match router("home/sensors/temperature" , temphum_raw) { 

        Ok(DeviceEvent::TemperatureHumidity(r)) => println!("{}",r),
        Ok(_) => println!("Unexpected event type ! ") , 
        Err(SensorError::ParseError(e)) => println!("Parse Failed : {}" ,e) ,       Err(SensorError::UnknownTopic(e)) => println!("Unkown Topic :  {}",e),       Err(SensorError::MissingField(f)) => println!("Missing Field : {}",f) ,       


    }
        





    
        
}

