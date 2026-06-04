CREATE TABLE IF NOT EXISTS sensor_readings (
    id          SERIAL PRIMARY KEY,
    device_id   VARCHAR(64)  NOT NULL,
    sensor_type VARCHAR(32)  NOT NULL,
    payload     JSONB        NOT NULL,
    received_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
