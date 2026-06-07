CREATE TABLE IF NOT EXISTS camera_frames (
    id          SERIAL PRIMARY KEY,
    device_id   VARCHAR(64)  NOT NULL,
    source_type VARCHAR(32)  NOT NULL, -- "esp32cam", "rtsp", "usb" 
    file_path   VARCHAR(256) NOT NULL,
    received_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
