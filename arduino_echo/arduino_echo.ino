#include <LiquidCrystal.h>

// setup lcd screen
LiquidCrystal lcd(8, 9, 4, 5, 6, 7);

void setup() {
  Serial.begin(9600); // open serial port
  lcd.begin(16, 2);
  lcd.print("hello");
}

String lastmsg;
void loop() {
	// reset cursor
	lcd.setCursor(0,0);
  
	// only read when data available
	if (Serial.available() > 0) {
		
		String msg = Serial.readString(); 
		
		// only update if new msg otherwhise the msg can be invisible
		if (msg != lastmsg) {
		lastmsg = msg;
		lcd.clear();
		
		String line2;
		// when line too long continue on next line
		if (msg.length() > 15) {
			line2 = msg.substring(16);
		}
      
		lcd.print(msg);
		lcd.setCursor(0, 1);
		lcd.print(line2);
		
		// echo msg back
		Serial.print(lastmsg); 
		}
	}
}