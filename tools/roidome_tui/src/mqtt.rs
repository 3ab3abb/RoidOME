use crate::app::App;
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use std::sync::{Arc, Mutex};
use serde::Deserialize;




#[derive(Debug ,Deserialize)]
struct TempHumidity {

    id:String ,
    temperature:f32,
    humidity:f32,
    timestamp:i64 ,

}

#[derive(Debug,Deserialize)]
struct MotionSensor { 

    id:String ,
    motion :bool , 
    timestamp:i64 , 

} 

#[derive(Debug,Deserialize)]

struct GasSensor { 

    id:String ,
    gas_level :f32 , 
    timestamp:i64 , 

} 




fn parse<T: for<'de> serde::Deserialize<'de>>(raw: &str) -> Result<T, serde_json::Error> { 

    serde_json::from_str(raw)

}





fn handle_message(app: &mut App,topic:&str , payload :&str ) { 
    
   
    match topic { 
        "home/sensors/temperature" => {
            if let Ok(reading) = parse::<TempHumidity>(payload){
                app.update_temperature(reading.temperature,reading.humidity,&reading.id) ; 
            }
        }

        "home/sensors/motion" => {
            if let Ok(reading) = parse::<MotionSensor>(payload){
                app.update_motion(reading.motion) ; 
            }
        }        
            
        "home/sensors/gas" => {

            if let Ok(reading) = parse::<GasSensor>(payload){
                app.update_gas(reading.gas_level) ; 
            }
        }


        "home/camera/snapshot" => {
                
             eprintln!("Received snapshot path: {}", payload);
             app.update_snapshot(payload.to_string());
        }
        
        _ => {}
    }
}






pub async fn start_mqtt(app: Arc<Mutex<App>>){

    let mut mqttoptions = MqttOptions::new("roidome-tui", "localhost", 1883) ; 
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5)) ; 

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("home/#", QoS::AtMostOnce).await.unwrap();


    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(msg))) => {
                let topic = msg.topic.clone();
                let payload = String::from_utf8(msg.payload.to_vec()).unwrap();

                let mut locked_app = app.lock().unwrap();
                handle_message(&mut locked_app, &topic, &payload);
            }
            Err(e) => {
                eprintln!("MQTT error: {}", e);
                break;
            }
            _ => {eprintln!("Unhandled topic: '{}'", topic);}
        }
    }
}






