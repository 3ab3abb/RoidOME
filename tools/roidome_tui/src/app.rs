



pub struct App { 

    pub temperature: f32 , 
    pub humidity: f32,
    pub gas_level: f32,
    pub motion: bool, 
    pub device_id: String,
    pub message_count: u32,
    pub running: bool, 

}

impl App { 

    pub fn new() -> Self {

        App {

            temperature: 0.0,
            humidity: 0.0,
            gas_level: 0.0,
            motion: false,
            device_id: String::from("waiting..."),
            message_count: 0,
            running: true
        }
    }


    pub fn update_temperature(&mut self, temp: f32, humidity: f32, id:&str){

        self.temperature = temp ;
        self.humidity = humidity ; 
        self.device_id = id.to_string() ; 
        self.message_count +=1 ; 
    }

    pub fn update_motion (&mut self, motion: bool) {


        self.motion = motion ; 
        self.message_count += 1 ; 

    }

    pub fn update_gas(&mut self, gas_level: f32){

        self.gas_level = gas_level ; 
        self.message_count += 1 ; 
    }
}
