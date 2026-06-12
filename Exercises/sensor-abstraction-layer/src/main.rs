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



fn main() {


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


  
