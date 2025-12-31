# badge

Rust firmware for the FOSSASIA Badge Magic LED name tag (11x44 LED matrix, CH582 microcontroller).

## Hardware

- MCU: WCH CH582 (RISC-V with BLE)
- Display: 11x44 LED matrix

Official C firmware: [fossasia/badgemagic-firmware](https://github.com/fossasia/badgemagic-firmware)

## Building

Requires nightly Rust and the RISC-V target:

```bash
rustup target add riscv32imac-unknown-none-elf
```

Build and flash with [wchisp](https://github.com/ch32-rs/wchisp):

```bash
cargo run --release
```
