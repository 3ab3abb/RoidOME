#pragma once 

#include <PubSubClient.h>

void initTempHumidity() ; 
void readAndPublishTempHumidity(PubSubClient& client) ; 
