//#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use embedded_time::rate::*;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, spi::Spi, delay::Delay};
use ssd1306::{prelude::{SPIInterfaceNoCS, DisplaySize128x64, DisplayRotation, Brightness}, Ssd1306, mode::DisplayConfig};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let mut delay = Delay::new(core.SYST, rcc.clocks);


    // Initialize RTT
    rtt_init_print!();


    // Initialize GPIO
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure SPI pins
    let sck = gpioa.pa5;
    let mut rst = gpioa.pa6.into_push_pull_output();
    let mosi = gpioa.pa7;

    // hold chip select low the entire time, we only have 1 chip
    let mut cs = gpiob.pb10.into_push_pull_output();
    cs.set_low().unwrap();

    let dc = gpioa.pa8.into_push_pull_output();
    // Initialize SPI
    let spi = Spi::spi1(
        dp.SPI1,
        (sck, stm32l0xx_hal::spi::NoMiso, mosi),
        stm32l0xx_hal::spi::MODE_0,
        1_000_000_u32.Hz(), // TODO: 1_u32.MHz() didn't work here, figure out why
        &mut rcc,
    );

    // Create display interface
    let interface = SPIInterfaceNoCS::new(spi, dc);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_terminal_mode();

    display
        .reset(&mut rst, &mut delay)
        .unwrap();

    let _ = display.init().unwrap();
    let _ = display.clear().unwrap();
    display.set_display_on(true).unwrap();
    display.set_brightness(Brightness::BRIGHTEST).unwrap();




    loop {
        cortex_m::asm::delay(1000_000);
        display.clear();
        cortex_m::asm::delay(10_000);
        display.write_str("cope").unwrap();
        cortex_m::asm::delay(1000_000);
        display.clear();
        cortex_m::asm::delay(10_000);
        display.write_str("seethe").unwrap();
        cortex_m::asm::delay(1000_000);
        display.clear();
        cortex_m::asm::delay(10_000);
        display.write_str("mald").unwrap();
        cortex_m::asm::delay(1000_000);
        display.clear();
        cortex_m::asm::delay(10_000);
        display.write_str("dilate").unwrap();
        rprintln!("aaaah");
    }
}

