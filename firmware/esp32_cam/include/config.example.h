#pragma once

// WiFi
#define WIFI_SSID      "your_network"
#define WIFI_PASSWORD  "your_password"

// MQTT
#define MQTT_BROKER    "192.168.x.x"
#define MQTT_PORT      1883
#define MQTT_CLIENT_ID "espcam-01"

// Topics
#define TOPIC_FRAME   "home/camera/frame"
#define TOPIC_MOTION  "home/sensors/motion"

// Pins
#define BUTTON_PIN    0   // GPIO0 — user button on AI-Thinker
#define FLASH_PIN     4   // onboard LED flash
