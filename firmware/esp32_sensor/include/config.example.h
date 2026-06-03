#pragma once

// ----- WiFi -----
#define WIFI_SSID     "your_network_name"
#define WIFI_PASSWORD "your_password"

// ----- MQTT Broker -----
#define MQTT_BROKER    "192.168.x.x"
#define MQTT_PORT      1883
#define MQTT_CLIENT_ID "esp32_01"

// ----- Topics -----
#define TOPIC_TEMPERATURE "home/sensors/temperature"
#define TOPIC_MOTION      "home/sensors/motion"
#define TOPIC_GAS         "home/sensors/gas"

// ----- Pins -----
#define DHT_PIN   4
#define DHT_TYPE  DHT22
#define MOTION_PIN 34
#define GAS_PIN    35

// ----- Timing -----
#define PUBLISH_INTERVAL_MS  5000
#define MQTT_RETRY_DELAY_MS  2000
