#include "MotionSensor.h"
#include "config.h"
#include <ArduinoJson.h>



void initMotion() {

	pinMode(MOTION_PIN,INPUT) ; 	
	Serial.println("PIR calibrating — wait 20s...");  	
	delay(20000); 
  	Serial.println("Sensor Ready!");
}



void readAndPublishMotion(PubSubClient& client) {

	bool motionState = digitalRead(MOTION_PIN);
	
	StaticJsonDocument<128> doc ; 

	doc["id"] = MQTT_CLIENT_ID ; 
	doc["motion"] = motionState ;

	// replace this with NTP time later
	doc["timestamp"] = millis() ; 
	
	char buf[128] ;
	serializeJson(doc,buf) ; 
	client.publish(TOPIC_MOTION ,buf) ; 
	Serial.printf("[Motion] Published:%s\n",buf) ; 


}


	

















