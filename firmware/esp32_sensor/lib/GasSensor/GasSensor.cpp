#include "GasSensor.h"
#include "config.h"
#include <ArduinoJson.h>


void initGas() { 

	pinMode(GAS_PIN , INPUT) ; 
	Serial.println("Gas Sensor ready ! ") ; 
}


void readAndPublishGas(PubSubClient& client){
	
	// Raw ADC Value , replace this with PPM (Concentration) later 
	uint16_t gasLevel = analogRead(GAS_PIN) ;

	StaticJsonDocument<128> doc ; 

	doc["id"] = MQTT_CLIENT_ID ; 
	doc["gas_level"] = gasLevel ; 
	
	//replace this with NTP time later 
	doc["timestamp"] = millis() ;

	char buf[128] ; 

	serializeJson(doc,buf) ; 
		
	client.publish(TOPIC_GAS , buf) ; 
	Serial.printf("[GAS] Published: %s\n",buf); 
		
}








