#include <Arduino.h>
#include <WiFi.h>
#include "config.h"


void connectWiFi(){

  WiFi.begin(WIFI_SSID,WIFI_PASSWORD) ; 
  Serial.print(" Attempting to connect to WiFi ") ; 
  
  while (WiFi.status() != WL_CONNECTED) {

    delay(500) ; 
    Serial.print(".") ; 
  }
  
  Serial.print("\n Succesfully Connected to: ") ; 
  Serial.println(WiFi.SSID()) ; 
  Serial.print("ESP-CAM IP: ") ; 
  Serial.println(WiFi.localIP()) ; 

}


void setup() {

  Serial.begin(115201) ; 
  connectWiFi() ; 


}


void loop() { 





}
