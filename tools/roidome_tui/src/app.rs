use ratatui_image::{picker::Picker, protocol::StatefulProtocol};

pub struct App {
    pub temperature: f32,
    pub humidity: f32,
    pub gas_level: f32,
    pub motion: bool,
    pub device_id: String,
    pub message_count: u32,
    pub running: bool,
    pub latest_frame_path: Option<String>,
    pub connected: bool,
    pub frame_count: u32,
    pub picker: Picker,
    pub image_state: Option<StatefulProtocol>,
}

impl App {
    // Picker created in main() before tokio runtime — passed in here
    pub fn new_with_picker(picker: Picker) -> Self {
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
            picker,
            image_state: None,
        }
    }

    pub fn update_snapshot(&mut self, path: String) {
        eprintln!("Opening image at: {}", path);
        let result = image::ImageReader::open(&path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            .and_then(|r| r.decode().map_err(|e| Box::new(e) as Box<dyn std::error::Error>));
        if let Ok(dyn_img) = result {
            self.image_state = Some(self.picker.new_resize_protocol(dyn_img));
        }
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
