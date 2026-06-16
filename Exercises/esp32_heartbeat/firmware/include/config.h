#pragma once

// WiFi
#define WIFI_SSID      "Treehouse4G"
#define WIFI_PASSWORD  "Treehouse2026"

// MQTT
#define MQTT_BROKER    "192.168.1.177"
#define MQTT_PORT      1883
#define MQTT_CLIENT_ID "esp32_01"

// Heartbeat
#define HEARTBEAT_TOPIC    "home/device/esp32_01/heartbeat"
#define HEARTBEAT_INTERVAL 30000   // 30 seconds in ms
