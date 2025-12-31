# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Flash Commands

```bash
# Build the project (release mode recommended for embedded)
cargo build --release

# Build and flash to device (uses wchisp)
cargo run --release
```

The project targets `riscv32imac-unknown-none-elf` and uses `wchisp flash` as the runner for flashing.

## Project Structure

The main application code is in `src/`. The `ch58x/` and `ch58x-hal/` directories are local copies of upstream crates included only to patch dependency versions - do not modify them unless necessary.

## Architecture

This is a firmware project for CH58x RISC-V BLE microcontrollers (CH581/CH582/CH583) from WCH.

### Key Dependencies

- **Embassy**: Async runtime with `arch-riscv32` executor
- **ch58x-hal**: HAL providing GPIO, UART, SPI, I2C, ADC, RTC, BLE, and Embassy time driver
- **qingke/qingke-rt**: RISC-V runtime for WCH chips

### Runtime Requirements

- Uses `#![no_std]` and `#![no_main]`
- Requires nightly Rust for `type_alias_impl_trait` and `impl_trait_in_assoc_type`
- Entry point via `#[embassy_executor::main(entry = "qingke_rt::entry")]`
