#include <MQTT.h>
#include <WiFi.h>

MQTTClient mqtt;

void setup() {
    Serial.begin(9600);
    mqtt.begin("localhost", 1883, WiFi);
    // Will connect to localhost port 1883 be default
    if (mqtt.connect("arduino", "public", "public")) {
        mqtt.subscribe("/", 2);
        mqtt.onMessage(+[](String& topic, String& message) {
            // handle the message
            Serial.println("Topic: " + topic + " Message: " + message);
        });
    }
    
    Serial.println("Setup!");
}

void loop() {
    if (mqtt.connected())
        mqtt.loop();
    if(Serial.available()) {
        auto str = Serial.readString();
        if (str.startsWith("q")) {
            throw 123;
        }
        Serial.println(str);
    }
        

#ifdef __SMCE__
    delay(1); // Avoid overwhelming the CPU
#endif
}

