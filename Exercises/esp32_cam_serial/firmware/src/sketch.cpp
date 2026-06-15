#include <Arduino.h>
#include "esp_camera.h"

#include "esp_camera.h"
#include "mbedtls/base64.h"





// AI-Thinker pin definition — DO NOT CHANGE
#define BUTTON_PIN 0
#define PWDN_GPIO_NUM     32
#define RESET_GPIO_NUM    -1
#define XCLK_GPIO_NUM      0
#define SIOD_GPIO_NUM     26
#define SIOC_GPIO_NUM     27
#define Y9_GPIO_NUM       35
#define Y8_GPIO_NUM       34
#define Y7_GPIO_NUM       39
#define Y6_GPIO_NUM       36
#define Y5_GPIO_NUM       21
#define Y4_GPIO_NUM       19
#define Y3_GPIO_NUM       18
#define Y2_GPIO_NUM        5
#define VSYNC_GPIO_NUM    25
#define HREF_GPIO_NUM     23
#define PCLK_GPIO_NUM     22

void initCamera() {
    camera_config_t config;
    config.ledc_channel = LEDC_CHANNEL_0;
    config.ledc_timer   = LEDC_TIMER_0;
    config.pin_d0       = Y2_GPIO_NUM;
    config.pin_d1       = Y3_GPIO_NUM;
    config.pin_d2       = Y4_GPIO_NUM;
    config.pin_d3       = Y5_GPIO_NUM;
    config.pin_d4       = Y6_GPIO_NUM;
    config.pin_d5       = Y7_GPIO_NUM;
    config.pin_d6       = Y8_GPIO_NUM;
    config.pin_d7       = Y9_GPIO_NUM;
    config.pin_xclk     = XCLK_GPIO_NUM;
    config.pin_pclk     = PCLK_GPIO_NUM;
    config.pin_vsync    = VSYNC_GPIO_NUM;
    config.pin_href     = HREF_GPIO_NUM;
    config.pin_sccb_sda = SIOD_GPIO_NUM;
    config.pin_sccb_scl = SIOC_GPIO_NUM;
    config.pin_pwdn     = PWDN_GPIO_NUM;
    config.pin_reset    = RESET_GPIO_NUM;
    config.xclk_freq_hz = 20000000;
    config.pixel_format = PIXFORMAT_JPEG;
    config.frame_size   = FRAMESIZE_QVGA;  // 320x240 — small for serial transfer
    config.jpeg_quality = 12;              // 0-63, lower = better quality
    config.fb_count     = 1;

    esp_err_t err = esp_camera_init(&config);
    if (err != ESP_OK) {
        Serial.printf("Camera init failed: 0x%x\n", err);
        return;
    }
    Serial.println("Camera ready");
}



String encodeBase64(uint8_t* data, size_t len) {
    size_t encoded_len = 0;
    // calculate output size
    mbedtls_base64_encode(nullptr, 0, &encoded_len, data, len);
    
    uint8_t* encoded = (uint8_t*)malloc(encoded_len);
    mbedtls_base64_encode(encoded, encoded_len, &encoded_len, data, len);
    
    String result = String((char*)encoded);
    free(encoded);
    return result;
}

void captureAndSend() { 

  camera_fb_t* fb  = esp_camera_fb_get() ; 

  if (!fb) { 
    Serial.println("Capture Failed !") ;
    return ;
  }

  Serial.printf("Captured %d bytes\n", fb->len);
  String encoded = encodeBase64(fb->buf, fb->len);
  Serial.println(encoded);

  esp_camera_fb_return(fb);
  

  Serial.println("Frame Sent") ; 


}

void setup() {
    Serial.begin(115200);
    pinMode(BUTTON_PIN, INPUT_PULLUP);
    initCamera();
}


void loop() {
    if (digitalRead(BUTTON_PIN) == LOW) {
        Serial.println("Button pressed — capturing...");
        captureAndSend();
        delay(2000);  // debounce
    }
}
