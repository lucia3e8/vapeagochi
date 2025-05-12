//#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use embedded_time::rate::*;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, spi::Spi, delay::Delay, adc::{Adc, SampleTime}, timer::Timer};
use ssd1306::{prelude::{SPIInterfaceNoCS, DisplaySize128x64, DisplayRotation, Brightness}, Ssd1306, mode::DisplayConfig};
use core::fmt::Write;
use heapless::{String, Vec};   // 32‑byte static buffer
use bitvec::prelude::*;

// Button state buffer size
const BUTTON_HISTORY_SIZE: usize = 10;

// Button states
struct ButtonState {
    left: BitArray<[u8; 2], Lsb0>,  // 2 bytes = 16 bits, more than enough for 10 samples
    right: BitArray<[u8; 2], Lsb0>,
    middle: BitArray<[u8; 2], Lsb0>,
    pos: usize,
}

impl ButtonState {
    fn new() -> Self {
        ButtonState {
            left: BitArray::ZERO,
            right: BitArray::ZERO,
            middle: BitArray::ZERO,
            pos: 0,
        }
    }

    fn update(&mut self, left: bool, right: bool, middle: bool) {
        self.left.set(self.pos, left);
        self.right.set(self.pos, right);
        self.middle.set(self.pos, middle);
        self.pos = (self.pos + 1) % BUTTON_HISTORY_SIZE;
    }

    fn is_debounced(&self, threshold: usize) -> (bool, bool, bool) {
        let left_count = self.left.iter().by_vals().take(BUTTON_HISTORY_SIZE).filter(|&x| x).count();
        let right_count = self.right.iter().by_vals().take(BUTTON_HISTORY_SIZE).filter(|&x| x).count();
        let middle_count = self.middle.iter().by_vals().take(BUTTON_HISTORY_SIZE).filter(|&x| x).count();

        (
            left_count >= threshold,
            right_count >= threshold,
            middle_count >= threshold
        )
    }
}


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let mut delay = Delay::new(core.SYST, rcc.clocks);

    // Initialize RTT
    rtt_init_print!();

    // Setup timer for 10ms intervals (100Hz)
    let mut timer = Timer::tim3(dp.TIM3, 1_u32.Hz(), &mut rcc);

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
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
        .into_terminal_mode();

    display
        .reset(&mut rst, &mut delay)
        .unwrap();
    let _ = display.init().unwrap();
    let _ = display.clear().unwrap();

    // ADC config
    let mut adc = Adc::new(dp.ADC, &mut rcc);
    let mut an_in = gpioa.pa4.into_analog(); // pin PA4 is HEAT_IN

    // coil enable/disable
    let mut coil_en = gpiob.pb11.into_push_pull_output();
    coil_en.set_high().unwrap(); // enables the heating coil

    // buttons!
    let btn_left = gpiob.pb3.into_floating_input();
    let btn_right = gpiob.pb2.into_floating_input();
    let btn_middle = gpioa.pa0.into_floating_input();

    let mut button_state = ButtonState::new();
    let mut buf: String<32> = String::new();




    loop {
        buf.clear();

        let left_pressed = btn_left.is_high().unwrap();
        let right_pressed = btn_right.is_high().unwrap();
        let middle_pressed = btn_middle.is_high().unwrap();
        button_state.update(left_pressed, right_pressed, middle_pressed);
        let (left_debounced, right_debounced, middle_debounced) = button_state.is_debounced(7); // 7 out of 10 samples

        use core::fmt::Write;
        let val:u16 = adc.read(&mut an_in).unwrap();
        write!(buf, "coil_in={}\n[{} {} {}] ",
            val,
            if left_debounced { "1" } else { "0" },
            if middle_debounced { "1" } else { "0" },
            if right_debounced { "1" } else { "0" }
        ).unwrap();

        // Wait for timer interrupt
        while timer.wait().is_ok() {
            display.clear();
            display.write_str(&buf);
            timer.start(60_u32.Hz());
        }

        // BUZZER DOES NOT WORK
        // probably because I wired it DC not AC on accident
        // TODO: test with a differential signal and another buzzer
        // bcs it might also be bad soldering
        // Toggle buzzer every 1ms
        //if buzzer_timer.wait().is_ok() {
        //    if buzz.is_set_high().unwrap() {
        //        buzz.set_low().unwrap();
        //    } else {
        //        buzz.set_high().unwrap();
        //    }
        //    buzzer_timer.start(500_u32.Hz());
        //}

    }
}

