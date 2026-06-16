#include <Arduino.h>
#include <PubSubClient.h>
#include <ArduinoJson.h>
#include<WiFi.h>
#include "config.h"




WiFiClient espClient ; 
PubSubClient client(espClient) ; 

void connectWiFi(){

  WiFi.begin(WIFI_SSID,WIFI_PASSWORD) ;
  Serial.print("Attempting to connect to: ") ; 
  Serial.println(WIFI_SSID) ;

  while (WiFi.status()!=WL_CONNECTED) {

    delay(500) ; 
    Serial.print(".") ; 

  }

  Serial.print("Successfully conneted to: ") ; 
  Serial.println(WIFI_SSID) ; 

  Serial.print(" Connected with IP:");
  Serial.println(WiFi.localIP()) ; 

}

void connectMQTT(){

  client.setServer(MQTT_BROKER, MQTT_PORT) ; 
  while(!client.connected()) {
    Serial.print("Connecting to MQTT");
    Serial.print("."); 
    if (client.connect(MQTT_CLIENT_ID)) {
      Serial.println(" Connected! "); 
    }else{
	    Serial.printf("failed , rc=%d - retrying\n",client.state()) ; 	
			delay(10000) ; 

    }
  }
}

void publishHeartBeat() {

  StaticJsonDocument<128>  doc; 
  
  doc["id"] = "esp-01" ; 
  doc["uptime"] = millis() ; 
  doc["free_heap"]  = ESP.getFreeHeap();
  doc["rssi"] = WiFi.RSSI() ; 

  char buf[128] ; 
  serializeJson(doc, buf) ; 

  client.publish(HEARTBEAT_TOPIC, buf); 
  Serial.printf("[HEARTBEAT] Published: %s\n", buf) ; 

}



void setup(){

  Serial.begin(115200); 
  connectWiFi(); 
  connectMQTT();


}

unsigned long lastHeartbeat = 0;


void loop(){


  client.loop() ;
  if(millis() - lastHeartbeat >= HEARTBEAT_INTERVAL){
    lastHeartbeat = millis() ; 
    publishHeartBeat() ; 
  }
  




}
