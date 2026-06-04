# RoidOME
### Distributed Smart Home Operating System

> A modular IoT platform integrating ESP32 firmware, a Rust async backend, real-time MQTT messaging, and PostgreSQL persistence.

---

## Architecture

```
ESP32 Firmware (C++ / PlatformIO)
    │  MQTT publish
    ▼
Mosquitto Broker
    │  subscribe home/#
    ▼
Rust Backend (Tokio + rumqttc)
    ├── MQTT listener task  →  mpsc channel
    └── Consumer task
            │  sqlx INSERT
            ▼
        PostgreSQL
```

**Stack decision:**
Firmware in C++/Arduino for fast iteration, backend in Rust for systems correctness. Migration to Embassy (Rust firmware) planned once the backend is solid.

---

## Technology Stack

| Layer | Technology | Purpose |
|---|---|---|
| Firmware | PlatformIO + Arduino/C++ | ESP32 sensor firmware |
| Async Runtime | Tokio | Rust async task scheduler |
| MQTT Client | rumqttc | Async MQTT for Rust |
| MQTT Broker | Mosquitto | Message broker |
| Database | PostgreSQL | Persistent sensor storage |
| ORM | sqlx | Async, compile-time SQL checking |
| Serialization | serde + serde_json | JSON parsing |
| Error Handling | thiserror | Custom error types |
| Logging | tracing | Structured async logging |
| Firmware JSON | ArduinoJson | JSON serialization on ESP32 |

---

## Repository Structure

```
RoidOME/
├── firmware/
│   └── esp32_sensor/
│       ├── platformio.ini
│       ├── src/main.cpp
│       ├── include/
│       │   ├── config.h              # gitignored — copy from config.example.h
│       │   └── config.example.h      # template with placeholder values
│       └── lib/
│           ├── TempHumidity/
│           ├── MotionSensor/
│           └── GasSensor/
├── backend/
│   └── sensor-service/
│       ├── src/main.rs
│       └── Cargo.toml
├── infra/
│   └── postgres/
│       └── migrations/
│           └── 0001_create_sensor_readings.sql
├── Exercises/
└── README.md
```

---

## Sensors & MQTT Topics

| Topic | Sensor | Payload Fields |
|---|---|---|
| `home/sensors/temperature` | DHT22 | `id`, `temperature` (°C), `humidity` (%), `timestamp` |
| `home/sensors/motion` | PIR | `id`, `motion` (bool), `timestamp` |
| `home/sensors/gas` | MQ-series | `id`, `gas_level` (ADC 0–4095), `timestamp` |

---

## Database Schema

```sql
CREATE TABLE sensor_readings (
    id          SERIAL PRIMARY KEY,
    device_id   VARCHAR(64)  NOT NULL,
    sensor_type VARCHAR(32)  NOT NULL,
    payload     JSONB        NOT NULL,
    received_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
```

`payload` is stored as `JSONB` — fully queryable without schema changes as sensor types evolve.

---

## Getting Started

### Prerequisites

- Rust (stable) — [rustup.rs](https://rustup.rs)
- PostgreSQL — `brew install postgresql`
- Mosquitto — `brew install mosquitto`
- PlatformIO CLI — `pip3 install platformio`

### 1. Clone the repo

```bash
git clone git@github.com:3ab3abb/RoidOME.git
cd RoidOME
```

### 2. Set up the database

```bash
psql postgres -c "CREATE DATABASE roidome;"
psql roidome -f infra/postgres/migrations/0001_create_sensor_readings.sql
```

### 3. Set environment variable

```fish
# fish
set -Ux DATABASE_URL "postgres://your_user@localhost/roidome"

# bash/zsh
export DATABASE_URL="postgres://your_user@localhost/roidome"
```

### 4. Configure firmware credentials

```bash
cp firmware/esp32_sensor/include/config.example.h \
   firmware/esp32_sensor/include/config.h
# edit config.h — WiFi SSID, password, broker IP, pin assignments
```

### 5. Run the backend

```bash
# Terminal 1 — start Mosquitto
brew services start mosquitto

# Terminal 2 — start Rust backend
cd backend/sensor-service && cargo run
```

### 6. Flash the firmware

```bash
cd firmware/esp32_sensor
pio run --target upload
pio device monitor --baud 115200
```

### 7. Verify end-to-end (without ESP32)

```bash
# Publish a test message
mosquitto_pub -h localhost -t "home/sensors/temperature" \
  -m '{"id":"esp32_01","temperature":24.5,"humidity":61.2,"timestamp":1234567890}'

# Check the database
psql roidome -c "SELECT * FROM sensor_readings;"
```

---

## Hardware

| Component | Details |
|---|---|
| Microcontroller | ESP32 (Espressif) |
| Temperature / Humidity | DHT22 — GPIO 4 |
| Motion | PIR sensor — GPIO 34 |
| Gas | MQ-series — GPIO 35 (analog) |
| Dev machine | MacBook Air M1 |

---

## Conventional Commits

```
feat:      adding something new
fix:       fixing a bug
chore:     maintenance, setup, tooling
docs:      documentation only
refactor:  restructuring, no behavior change
ci:        CI/CD pipeline changes
test:      adding or fixing tests
```

---

## License

MIT
