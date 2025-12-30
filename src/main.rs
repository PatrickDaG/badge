#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![allow(rust_2024_compatibility)]

use ch58x_hal as hal;
use ch58x_hal::gpio::Flex;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Instant, Timer};
use hal::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
use hal::peripherals;
use hal::prelude::*;
use hal::rtc::Rtc;
use hal::uart::UartTx;
use qingke::riscv::asm;

static mut SERIAL: Option<UartTx<peripherals::UART1>> = None;

macro_rules! println {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            use core::writeln;

            if let Some(uart) = SERIAL.as_mut() {
                writeln!(uart, $($arg)*).unwrap();
            }
        }
    }
}

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
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::_5mA);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(150)).await;
        led.set_low();
        Timer::after(Duration::from_millis(150)).await;
    }
}

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_60mhz();
    // config.enable_dcdc = true;

    let p = hal::init(config);
    hal::embassy::init();

    let _rtc = Rtc::new(p.RTC);

    let uart = UartTx::new(p.UART1, p.PA9, Default::default()).unwrap();
    unsafe {
        SERIAL.replace(uart);
    }

    let mut a = Output::new(p.PA15, Level::Low, OutputDrive::_5mA);
    let mut b = Output::new(p.PB18, Level::High, OutputDrive::_5mA);
    for i in 0..100000 {
        a.set_low();
        b.set_high();
    }
    for i in 0..100000 {
        b.set_low();
        a.set_high();
    }
    loop {}

    // GPIO
    // spawner.spawn(blink(p.PA8.degrade())).unwrap();

    //     println!("\n\nHello World from ch58x-hal!");
    //     println!(
    //         r#"
    //     ______          __
    //    / ____/___ ___  / /_  ____ _____________  __
    //   / __/ / __ `__ \/ __ \/ __ `/ ___/ ___/ / / /
    //  / /___/ / / / / / /_/ / /_/ (__  |__  ) /_/ /
    // /_____/_/ /_/ /_/_.___/\__,_/____/____/\__, /
    //                                       /____/   on CH582F"#
    //     );
    //     println!("System Clocks: {}", hal::sysctl::clocks().hclk);
    //     println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
    //     println!("RTC datetime: {}", rtc.now());

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

    // GPIO
    leds[0].set_low();
    leds[0].set_as_output(OutputDrive::_5mA);

    leds[1].set_as_input(Pull::None);
    leds[1].set_high();
    leds[2].set_as_input(Pull::None);
    leds[2].set_high();
    // let boot_btn = Input::new(p.PB22, Pull::Up);
    // let rst_btn = Input::new(p.PB23, Pull::Up);
    //
    // let uart = UartTx::new(p.UART1, p.PA9, Default::default()).unwrap();
    // unsafe {
    //     SERIAL.replace(uart);
    // }
    //
    // let rtc = Rtc::new(p.RTC);

    // println!("\n\nHello World!");
    // println!("System Clocks: {}", hal::sysctl::clocks().hclk);
    // println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
    // println!("RTC datetime: {}", rtc.now());

    let mut i = 0;
    loop {
        if i % 100000 == 0 {
            leds[1].set_as_output(OutputDrive::_5mA);
            leds[2].set_as_input(Pull::None);
        } else {
            leds[2].set_as_output(OutputDrive::_5mA);
            leds[1].set_as_input(Pull::None);
        }
        // for k in 0..100000 {
        //     unsafe {
        //         asm::nop();
        //     }
        // }
        i += 1;
        if i == 100000 {
            i = 0;
        }
        // println!("inst => {:?}, {}", Instant::now(), i);
        // Delay.delay_ms(1000_u32); // blocking delay
        // Timer::after(Duration::from_millis(1000)).await;
    }
}
