use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TempHumidity {
    id: String,
    temperature: f32,
    humidity: f32,
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct MotionSensor {
    id: String,
    motion: bool,
    timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct GasSensor {
    id: String,
    gas_level: f32,
    timestamp: i64,
}


trait Sensor { 

    fn device_id(&self) -> &str ; 
    fn sensor_type(&self) -> &str ; 
    fn to_payload(&self) ->serde_json::Value;  
    

}

impl Sensor for MotionSensor { 

    fn device_id(&self)-> &str { 
        &self.id 
    }


    fn sensor_type(&self) -> &str { 
       "Motion Sensor - PIR"

    }

    fn to_payload(&self) -> serde_json::Value { 


        serde_json::json! ({

            "id" :self.id , 
            "motion" : self.motion,
            "timestamp" : self.timestamp , 
        })
    }
}


impl Sensor for GasSensor { 

    fn device_id(&self)->&str {

        &self.id 
    }


    fn sensor_type(&self)->&str {



        "Gas Sensor"


    }


    fn to_payload(&self)->serde_json::Value{


        serde_json::json!({

            "id" : self.id , 
            "gas_level" : self.gas_level , 
            "timestamp" : self.timestamp,
        })

    }
}



impl Sensor for TempHumidity { 

    fn device_id(&self)->&str {

        &self.id 

    }

    fn sensor_type(&self)->&str {
        "Temperature & Humidity Sensor"         
    }


    fn to_payload(&self) -> serde_json::Value {



            serde_json::json!({
                "id" : self.id ,
                "temperature" :self.temperature,
                "humidity" : self.humidity , 
                "timestamp" : self.timestamp,
            })


           } 
    
}




fn log_sensor(sensor: &dyn Sensor) {
    println!("{},{},{}",sensor.sensor_type(),sensor.device_id(),sensor.to_payload()) ; 
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



    let messages: Vec<(&str, &str)> = vec![
    ("home/sensors/temperature", r#"{"id":"esp32_01","temperature":24.5,"humidity":61.2,"timestamp":1234}"#),
    ("home/sensors/motion",      r#"{"id":"esp32_01","motion":true,"timestamp":1234}"#),
    ("home/sensors/unknown",     r#"{"id":"esp32_01"}"#),
    ("home/sensors/gas",         r#"{"id":"esp32_02","gas_level":412.0,"timestamp":1234}"#),
    ("home/sensors/temperature", r#"{"id":"esp32_01","temperature":22.1,"humidity":58.0,"timestamp":1235}"#),
];


    let events: Vec<DeviceEvent> = messages
    .iter()
    .filter(|(topic,_)| match *topic   {

    "home/sensors/temperature" => true,  
    "home/sensors/gas"  =>true, 
    "home/sensors/motion" => true , 
    _ => false ,
    } ) ; 



let example = TempHumidity { 


    id : String::from("esp-01"),

    temperature: 24.5 ,

    humidity : 65.0,

    timestamp : 345678987 , 

} ; 




let example_1 = MotionSensor { 


    id : String::from("esp-02") , 
    motion : true , 
    timestamp : 345678954 , 

}; 





log_sensor(&example) ; 
log_sensor(&example_1) ;







}


  
