This is an ESP32-C3 firmware project (Rust, esp-hal). The flake includes espflash as the flashing tool. Here's how to
flash:

1. Build the firmware
cargo build --release

2. Flash with espflash
espflash flash --chip esp32c3 target/riscv32imc-unknown-none-elf/release/xteink-x4

Connect the device via USB-C. If espflash can't auto-detect the port, specify it explicitly:
espflash flash --chip esp32c3 --port /dev/ttyACM0 target/riscv32imc-unknown-none-elf/release/xteink-x4

3. Optional: monitor serial output after flashing
espflash flash --chip esp32c3 --monitor target/riscv32imc-unknown-none-elf/release/xteink-x4

To find your device's port: ls /dev/tty{USB,ACM}* before and after plugging in.

