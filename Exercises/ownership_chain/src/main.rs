#[derive(Debug)]
struct SensorReading { 

    id:String , 
    temperature : i32 , 
    timestamp : i64
}  



fn log_reading(reading:&SensorReading) { 



    println!("{:#?}",reading)  ; 


}


fn validate_reading(reading:&SensorReading)->bool{

    if reading.temperature<100 && reading.temperature>-50 { 
            println!("Valid Temperature") ; 
           return  true  ; 
        
    }
    
    return false ; 


}
fn archive_reading(reading:SensorReading) {

    println!("Reading : {:#?}  --  ARCHIVED !",reading) ; 


}




fn main() { 


    let reading = SensorReading { 
         id : String::from("ESP-01"), 

         temperature : 45 , 
         timestamp : 124342134 , 
    } ;
    
    let _ = log_reading(&reading) ;
    let _ = validate_reading(&reading);
    let _ = archive_reading(reading) ;



}
