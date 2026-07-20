# Xteink X4 Hardware Specifications

## Overview

The Xteink X4 is a pocket-sized open-hardware e-ink reader powered by an ESP32-C3.

| Field | Value |
|---|---|
| Dimensions | 114 × 69 × 5.9 mm |
| Weight | ~74–77 g |
| USB | USB-C (charging + data) |
| Battery | 650 mAh Li-Po (~14 days per charge) |

---

## SoC — ESP32-C3

| Field | Value |
|---|---|
| Chip | ESP32-C3 (Espressif) |
| Architecture | RISC-V 32-bit, single-core |
| CPU Speed | Up to 160 MHz |
| On-chip SRAM | 400 KB |
| Flash | 16 MB (external SPI) |
| WiFi | 802.11 b/g/n 2.4 GHz |
| Bluetooth | BLE 5.0 |
| GPIO count | 22 usable pins |

---

## Display

| Field | Value |
|---|---|
| Size | 4.26 inch |
| Resolution | 800 × 480 px |
| PPI | 220 |
| Type | Monochrome e-ink (e-paper) |
| Panel model | GDEQ0426T82 |
| Controller | SSD1677 |
| Interface | SPI |

---

## Storage

| Field | Value |
|---|---|
| MicroSD | Slot present, up to 512 GB supported |
| Bundled card | 16 GB (some models 32 GB) |
| Interface | SPI (shared bus with display) |

---

## GPIO Pin Map

### E-ink Display (SSD1677 via SPI)

| Signal | GPIO |
|---|---|
| SCK (clock) | GPIO8 |
| MOSI (data out) | GPIO10 |
| CS (chip select) | GPIO21 |
| DC (data/command) | GPIO4 |
| RST (reset) | GPIO5 |
| BUSY | GPIO6 |

### MicroSD (shared SPI bus)

| Signal | GPIO |
|---|---|
| CS | GPIO12 |
| MISO | GPIO7 |
| SCK | GPIO8 (shared) |
| MOSI | GPIO10 (shared) |

### Buttons

| Button | GPIO / Method |
|---|---|
| Power button | GPIO3 (direct) |
| Up / Down / Left / Right / Confirm / Back | ADC resistor ladder |
| Button ADC line 1 | GPIO1 |
| Button ADC line 2 | GPIO2 |

### Power & Charging

| Signal | GPIO |
|---|---|
| Battery voltage monitor | GPIO0 (voltage divider) |
| USB charge detect | GPIO20 |

---

## Rust Target

| Field | Value |
|---|---|
| Rust target triple | `riscv32imc-unknown-none-elf` |
| Recommended HAL | [`esp-hal`](https://github.com/esp-rs/esp-hal) |
| Chip feature flag | `esp32c3` |

---

## Sources

- [Xteink X4 product page](https://www.xteink.com/products/xteink-x4)
- [Adafruit CircuitPython pinout guide](https://learn.adafruit.com/circuitpython-on-the-xteink-x4-ereader/pinouts)
- [CircuitPython board definition](https://circuitpython.org/board/xteink_x4/)
- [esp-hal repository](https://github.com/esp-rs/esp-hal)
