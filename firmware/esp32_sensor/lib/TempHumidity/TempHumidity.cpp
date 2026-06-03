#include "TempHumidity.h"
#include "config.h"
#include <DHT.h>
#include <ArduinoJson.h>


static DHT dht(DHT_PIN ,DHT_TYPE) ; 

void initTempHumidity() {

	dht.begin() ; 
}



void readAndPublishTempHumidity(PubSubClient& client){


	float temp = dht.readTemperature() ; 

	float hum = dht.readHumidity() ; 




	if (isnan(temp) || isnan(hum)) { 

		Serial.println("[TempHumidity] Read failed - skipping publish !" ; )

		return  ; 

	}



	StaticJsonDocument<128> doc ;

	doc["id"] = MQTT_CLIENT_ID ; 
	doc["temperature"] = temp ; 
	doc["humidity"] = hum ; 
	//replace this with NTP time later 
	doc["timestamp"] = millis() ; 


	char buf[128] ; 
	serializejson(doc,buf) ; 


	client.publish(TOPIC_TEMPERATURE , buf) ; 
	Serial.printf("[TempHumidity] Published: %s\n",buf) ; 



}
