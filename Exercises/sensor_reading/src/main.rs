use serde::Deserialize ; 
use std::fmt ; 

//1. Data modeling 
#[derive(Debug ,Deserialize)]
struct SensorReading {

    id:String ,
    temperature:f32,
    humidity:f32,
    timestamp:i64 ,

}
//2. Implement Data Display

impl fmt::Display for SensorReading { 
    
    fn fmt(&self , f:&mut fmt::Formatter) -> fmt::Result { 

        write!(
            f,
            "device : {} | temperature : {}°C |humidty : {}% , |timestamp : {}",
            self.id , self.temperature , self.humidity , self.timestamp 
              )
               }

}
//3. Parsing JSON
fn parse_reading(raw:&str) -> Result<SensorReading ,serde_json::Error>{


    serde_json::from_str(raw) 


}




fn main() {


let valid_raw = r#"
        {
            "id": "esp32_01",
            "temperature": 24.5,
            "humidity": 61.2,
            "timestamp": 1234567890
        }
    "#;



 let bad_raw = r#"
        {
            "id": "esp32_01",
            "temperature": "not_a_number"
        }
    "#;


// 4. Matching depedning on the Parse 
//
    match parse_reading(valid_raw) {

        Ok(reading) => println!(" VALID READING ----- {}" ,reading) ,
        Err(e)=>println!("Failed to parse ! {}",e) , 


    }

    match parse_reading(bad_raw) { 

        Ok(reading) =>println!(" VALID READING ----- {}" ,reading) ,
        Err(e) =>  println!("Failed to parse ! {}",e) , 
    
        

    }





} 




