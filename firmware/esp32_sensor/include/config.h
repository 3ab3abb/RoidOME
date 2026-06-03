#pragma once 

// ----- Wifi ----- 

#define WIFI_SSID ""  //     going to be replaced by Wifi Manager after  to avoid
#define WIFI_PASSWORD "" //    hardcoding the creds . 

// ----- MQTT BROKER ----- 

 // My mac local IP
#define MQTT_BROKER "" 

#define MQTT_PORT	1883 
#define MQTT_CLIENT_ID "esp32_01"

// ----- MQTT TOPICS -----

#define TOPIC_TEMPERATURE "home/sensors/temperature"
#define TOPIC_MOTION "home/sensors/motion"
#define TOPIC_GAS "home/sensors/gas"


// ----- PINS -----

#define DHT_PIN 4 
#define DHT_TYPE DHT22

// digital input
#define MOTION_PIN 34 

//analog input 
#define GAS_PIN 35

// ----- TIMING ----- 

// Basically how often sensors publish
#define  PUBLISH_INTERVALS_MS 5000 

 // Delay between reconnection
#define MQTT_RETRY_DELAY_MS 2000 




