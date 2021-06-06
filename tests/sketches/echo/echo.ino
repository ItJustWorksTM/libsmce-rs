void setup() {
    Serial.begin(9600);
}

void loop() {
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

