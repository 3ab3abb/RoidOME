pub struct App {
    pub temperature: f32,
    pub humidity: f32,
    pub gas_level: f32,
    pub motion: bool,
    pub device_id: String,
    pub message_count: u32,
    pub running: bool,
    pub latest_frame_path: Option<String>,
    pub connected: bool,  // true once first MQTT message received
    pub frame_count: u32,
}

impl App {
    pub fn new() -> Self {
        App {
            temperature: 0.0,
            humidity: 0.0,
            gas_level: 0.0,
            motion: false,
            device_id: String::from("—"),
            message_count: 0,
            running: true,
            latest_frame_path: None,
            frame_count: 0,
            connected: false,
        }
    }


    pub fn update_snapshot(&mut self, path: String) {
        self.latest_frame_path = Some(path);
        self.frame_count += 1;
        self.connected = true;
    }


    pub fn update_temperature(&mut self, temp: f32, humidity: f32, id: &str) {
        self.temperature = temp;
        self.humidity = humidity;
        self.device_id = id.to_string();
        self.message_count += 1;
        self.connected = true;
    }

    pub fn update_motion(&mut self, motion: bool) {
        self.motion = motion;
        self.message_count += 1;
        self.connected = true;
    }

    pub fn update_gas(&mut self, gas_level: f32) {
        self.gas_level = gas_level;
        self.message_count += 1;
        self.connected = true;
    }
}
