# All-in-one console (PN532 on I2C + NAU7802 load cell)

The console detects this wiring at boot, the same image runs the regular SPI wiring.
Same wiring as [SpoolmanScale](https://github.com/Niko11111/SpoolmanScale) and espoolbuddy's console-scale.

| Signal | WT32-SC01 Plus |
|---|---|
| I2C SDA (PN532 and NAU7802) | GPIO10 |
| I2C SCL (PN532 and NAU7802) | GPIO11 |
| PN532 RSTPDN | GPIO14 |
| PN532 VCC and NAU7802 VIN | 5V |

- PN532 DIP switches to I2C: switch 1 ON, switch 2 OFF
- Enable "Local Scale Available" in the scale configuration, restart, then calibrate
