void setup() { Serial.begin(9600); Serial.println("Setup!"); }
void loop() { delay(1000); throw 123; }
