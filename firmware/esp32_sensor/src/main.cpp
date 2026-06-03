
#include "config.h"
#include "TempHumidity.h"
#include <PubSubClient.h>
#include <WiFi.h>



WifiClient espClient; 
PubSubClient client(espClient) ;  


void connectWiFi() {

	WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
   	Serial.print("Connecting to WiFi");

	while (WiFi.status() != WL_CONNECTED) { 
		
		delay(500); 
		Serial.print(".") ; 

		}

	
	Serial.print("\n WiFi connected - IP: ") ; 
	Serial.print(WiFi.localIP()) ; 
	
}



void connectMQTT() {

	client.setServer(MQTT_BROKER,MQTT_PORT) ; 
	
	while (!client.connected()) {
		Serial.print("Connecting to MQTT...");
		if (client.connect(MQTT_CLIENT_ID)){
			Serial.println("Connected ! ") ; 

		}else { 
		
			Serial.printf("failed , rc=%d - retrying\n",client.state()) ; 	

		}	
	}

}



void setup ()  { 

	Serial.begin(115200) ; 
	connectWiFI() ;  
	connectMQTT() ; 
	initTempHumidity() ; 

	
} 



unsigned long lastPublish = 0 ; 

void loop() { 

	//keeps mqtt connection alive to be able to process incoming messages 
	client.loop() ; 

	if (millis() - lastPublish >= PUBLISH_INTERVAL_MS) { 
	

		lastPublish = millis() ; 
		readAndPublishTempHumidity(client) ; 

	}

} 


