#include <Arduino.h>


constexpr int MAX_PRESET = 6;

constexpr int UP_RELAY = 2;
constexpr int RUN_RELAY = 4;
constexpr int BUTTON_TIMING_MS = 600;
constexpr int DISPENSE_TIME = 5000 - BUTTON_TIMING_MS;

int current_preset = 1;

void setup() {
    Serial.begin(9600);
    Serial.println("Starting Up!");

	pinMode(UP_RELAY, OUTPUT);
	pinMode(RUN_RELAY, OUTPUT);
}

void loop() {
	digitalWrite(UP_RELAY, LOW);
	digitalWrite(RUN_RELAY, LOW);


	int x = 0;
    if (Serial.available() > 0) {
		x = Serial.parseInt();
    }
	if (x == 0 || x > MAX_PRESET) {
		return;
	}
	Serial.println(x);

	// // Always sit on the first preset, so we can always go back to it
	// // So we cycle through every option, and then just run on the right one
	// for (int i = 1; i <= MAX_PRESET; i++) {
	// 	Serial.print("Preset A: ");
	// 	Serial.println(i);

	// 	digitalWrite(UP_RELAY, HIGH);
	// 	delay(BUTTON_TIMING_MS);
	// 	digitalWrite(UP_RELAY, LOW);
	// 	delay(BUTTON_TIMING_MS);

	// 	if (i == x) {
	// 		digitalWrite(RUN_RELAY, HIGH);
	// 		delay(BUTTON_TIMING_MS);
	// 		digitalWrite(RUN_RELAY, LOW);
	// 		delay(BUTTON_TIMING_MS);
	// 	}

	// }

	// Actually, use the saved current preset
	while (current_preset != x) {
		Serial.print("Preset B: ");
		Serial.println(current_preset);

		digitalWrite(UP_RELAY, HIGH);
		delay(BUTTON_TIMING_MS);
		digitalWrite(UP_RELAY, LOW);
		delay(BUTTON_TIMING_MS);

		current_preset++;
		if (current_preset > MAX_PRESET) {
			current_preset = 1;
		}
	}

	digitalWrite(RUN_RELAY, HIGH);
	delay(BUTTON_TIMING_MS);
	digitalWrite(RUN_RELAY, LOW);
	delay(BUTTON_TIMING_MS);

	delay(DISPENSE_TIME);
}