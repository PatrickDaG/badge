#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![allow(rust_2024_compatibility)]

use ch58x_hal as hal;
use ch58x_hal::gpio::Flex;
use ch58x_hal::pac::{SYSTICK, systick};
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Instant, Timer};
use hal::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
use hal::peripherals;
use hal::prelude::*;
use hal::rtc::Rtc;
use hal::uart::UartTx;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let pa9 = unsafe { peripherals::PA9::steal() };
    let uart1 = unsafe { peripherals::UART1::steal() };
    let mut serial = UartTx::new(uart1, pa9, Default::default()).unwrap();

    let _ = writeln!(&mut serial, "\n\n\n{}", info);

    loop {}
}

#[embassy_executor::task]
async fn schlink() {
    Timer::after(Duration::from_millis(10000)).await;
    unsafe {
        hal::reset();
    }
}

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::_5mA);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(150)).await;
        led.set_low();
        Timer::after(Duration::from_millis(150)).await;
    }
}

struct LedIndex(u16);

// const MATRIX: [[LedIndex; 44]; 11] = [
//     // row![CA BA AB CB AC BC AD BD AE BE AF BF AG BG AH BH AI BI AJ BJ AK BK AL BL AM BM AN BN AO BO AP BP AQ BQ AR BR AS BS AT BT AU BU AV BV],
//     // row![DA EA DB EB DC EC DD ED DE EE],
//     row![CBACABABABABABABABABABABABABABABABABABABABAB],
//     row![DEDEDECECDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCD],
// ];

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_60mhz();
    // config.enable_dcdc = true;

    let p = hal::init(config);
    hal::embassy::init();

    let _rtc = Rtc::new(p.RTC);
    // spawner.spawn(schlink()).unwrap();

    let mut battery_input = Flex::new(p.PA5);
    battery_input.set_as_input(Pull::None);

    let mut leds = [
        Flex::new(p.PA15).degrade(),
        Flex::new(p.PB18).degrade(),
        Flex::new(p.PB0).degrade(),
        Flex::new(p.PB7).degrade(),
        Flex::new(p.PA12).degrade(),
        Flex::new(p.PA10).degrade(),
        Flex::new(p.PA11).degrade(),
        Flex::new(p.PB9).degrade(),
        Flex::new(p.PB8).degrade(),
        Flex::new(p.PB15).degrade(),
        Flex::new(p.PB14).degrade(),
        Flex::new(p.PB13).degrade(),
        Flex::new(p.PB12).degrade(),
        Flex::new(p.PB5).degrade(),
        Flex::new(p.PA4).degrade(),
        Flex::new(p.PB3).degrade(),
        Flex::new(p.PB4).degrade(),
        Flex::new(p.PB2).degrade(),
        Flex::new(p.PB1).degrade(),
        Flex::new(p.PB6).degrade(),
        Flex::new(p.PB21).degrade(),
        Flex::new(p.PB20).degrade(),
        Flex::new(p.PB19).degrade(),
    ];
    for i in leds.iter_mut() {
        i.set_as_input(Pull::None);
    }
    leds[0].set_high();
    leds[0].set_as_output(OutputDrive::_20mA);
    leds[1].set_low();
    leds[2].set_low();

    let mut states = [false; 23];
    loop {
        let rb = unsafe { &*SYSTICK::PTR };
        let ticks = rb.cnt().read().bits() >> 8;

        for i in 1..17 {
            let k = i;
            states[k] = ((ticks >> (i - 1)) & 1) == 1;
            if states[k] {
                leds[k].set_as_output(OutputDrive::_5mA);
            } else {
                leds[k].set_as_input(Pull::None);
            }
        }

        hal::delay_ms(1000);

        // for i in 1..leds.len() {
        //     let rb = unsafe { &*SYSTICK::PTR };
        //     let ticks = rb.cnt().read().bits();
        //     states[i] = !states[i];
        //     if states[i] {
        //         leds[i].set_as_output(OutputDrive::_5mA);
        //     } else {
        //         leds[i].set_as_input(Pull::None);
        //     }
        //     hal::delay_ms(10);
        // }
    }
}
