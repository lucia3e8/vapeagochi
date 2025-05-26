//#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use bitvec::prelude::*;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_time::rate::*;
use heapless::{String, Vec}; // 32‑byte static buffer
use rtt_target::{rprintln, rtt_init_print};
use ssd1306::{
    mode::DisplayConfig,
    prelude::{DisplayRotation, DisplaySize128x64, SPIInterfaceNoCS},
    Ssd1306,
};
use stm32l0xx_hal::pac::FLASH;
use stm32l0xx_hal::{
    adc::Adc,
    delay::Delay,
    pac,
    prelude::*,
    rcc::Config,
    spi::Spi,
    timer::Timer,
};

// Button state buffer size
const BUTTON_HISTORY_SIZE: usize = 10;
const VOLTAGE_HISTORY_SIZE: usize = 10;

const HIT_HIGH: u16 = 140;
const HIT_LOW: u16 = 120;

// how many consecutive samples above/below to trigger/reset
const OVER_DEBOUNCE: usize = 5;
const UNDER_DEBOUNCE: usize = 3;

// Add EEPROM support for persistent storage
// STM32L072 has 6KB data EEPROM starting at 0x08080000
const EEPROM_BASE: u32 = 0x0808_0000;
const HITTIME_OFFSET: u32 = 0x0002; // bytes

// Button states
struct ButtonState {
    left: BitArray<[u8; 2], Lsb0>, // 2 bytes = 16 bits, more than enough for 10 samples
    right: BitArray<[u8; 2], Lsb0>,
    middle: BitArray<[u8; 2], Lsb0>,
    pos: usize,
}

// Voltage state using circular buffer approach
struct VoltageState {
    samples: Vec<u16, VOLTAGE_HISTORY_SIZE>,
    pos: usize,
    hit_active: bool,
    hit_start_time: Option<u32>,
    total_hit_duration_ms: u32,
    current_hit_duration_ms: u32,
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
        let left_count = self
            .left
            .iter()
            .by_vals()
            .take(BUTTON_HISTORY_SIZE)
            .filter(|&x| x)
            .count();
        let right_count = self
            .right
            .iter()
            .by_vals()
            .take(BUTTON_HISTORY_SIZE)
            .filter(|&x| x)
            .count();
        let middle_count = self
            .middle
            .iter()
            .by_vals()
            .take(BUTTON_HISTORY_SIZE)
            .filter(|&x| x)
            .count();

        (
            left_count >= threshold,
            right_count >= threshold,
            middle_count >= threshold,
        )
    }
}

impl VoltageState {
    fn new() -> Self {
        VoltageState {
            samples: Vec::new(),
            pos: 0,
            hit_active: false,
            hit_start_time: None,
            total_hit_duration_ms: 0,
            current_hit_duration_ms: 0,
        }
    }

    fn update(&mut self, sample: u16) {
        if self.samples.len() < VOLTAGE_HISTORY_SIZE {
            self.samples.push(sample).unwrap();
        } else {
            self.samples[self.pos] = sample;
        }
        self.pos = (self.pos + 1) % VOLTAGE_HISTORY_SIZE;
    }

    fn check_hit(&mut self, current_time_ms: u32) -> bool {
        if self.samples.len() < VOLTAGE_HISTORY_SIZE {
            return false; // Not enough samples yet
        }

        let over_threshold_count = self.samples.iter().filter(|&&val| val > HIT_HIGH).count();

        let under_threshold_count = self.samples.iter().filter(|&&val| val < HIT_LOW).count();

        let mut hit_just_ended = false;

        // Check if hit is starting
        if over_threshold_count >= OVER_DEBOUNCE && !self.hit_active {
            self.hit_active = true;
            self.hit_start_time = Some(current_time_ms);
            self.current_hit_duration_ms = 0;
        }
        // Check if hit is ending
        else if under_threshold_count >= UNDER_DEBOUNCE && self.hit_active {
            self.hit_active = false;
            if let Some(start) = self.hit_start_time {
                let duration = current_time_ms.wrapping_sub(start);
                self.total_hit_duration_ms = self.total_hit_duration_ms.wrapping_add(duration);
                self.current_hit_duration_ms = 0;
                hit_just_ended = true;
            }
            self.hit_start_time = None;
        }
        // Update current hit duration if active
        else if self.hit_active {
            if let Some(start) = self.hit_start_time {
                self.current_hit_duration_ms = current_time_ms.wrapping_sub(start);
            }
        }

        hit_just_ended
    }

    fn get_total_duration_seconds(&self) -> f32 {
        self.total_hit_duration_ms as f32 / 1000.0
    }

    fn get_current_duration_ms(&self) -> u32 {
        self.current_hit_duration_ms
    }

    fn is_hitting(&self) -> bool {
        self.hit_active
    }
}

// unlock the EEPROM for programming
fn unlock_eeprom(flash: &FLASH) {
    // write the two PEKEYR keys
    flash.pekeyr.write(|w| unsafe { w.bits(0x89ABCDEF) });
    flash.pekeyr.write(|w| unsafe { w.bits(0x02030405) });
}

// lock the EEPROM against further writes
fn lock_eeprom(flash: &FLASH) {
    flash.pecr.modify(|_, w| w.pelock().set_bit());
}

// program one 16-bit half-word at (EEPROM_BASE + offset)
fn write_halfword(flash: &FLASH, offset: u32, data: u16) {
    let addr = (EEPROM_BASE + offset) as *mut u16;
    // wait until not busy
    while flash.sr.read().bsy().bit_is_set() {}
    unlock_eeprom(flash);
    // Write data directly - STM32L0 EEPROM doesn't need PROG bit
    unsafe { core::ptr::write_volatile(addr, data) };
    // wait until done
    while flash.sr.read().bsy().bit_is_set() {}
    lock_eeprom(flash);
}

// read one 16-bit half-word from EEPROM
fn read_halfword(offset: u32) -> u16 {
    let addr = (EEPROM_BASE + offset) as *const u16;
    unsafe { core::ptr::read_volatile(addr) }
}

// write a full 32-bit word as two half-words
fn write_word(flash: &FLASH, offset: u32, data: u32) {
    let lo = (data & 0xFFFF) as u16;
    let hi = (data >> 16) as u16;
    write_halfword(flash, offset, lo);
    write_halfword(flash, offset + 2, hi);
}

// read back a 32-bit word
fn read_word(offset: u32) -> u32 {
    let lo = read_halfword(offset) as u32;
    let hi = read_halfword(offset + 2) as u32;
    (hi << 16) | lo
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
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180).into_terminal_mode();

    display.reset(&mut rst, &mut delay).unwrap();
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
    let mut voltage_state = VoltageState::new();
    let mut buf: String<64> = String::new();
    let mut elapsed_ms: u32 = 0;

    // Load saved duration from EEPROM
    let saved_duration = read_word(HITTIME_OFFSET);
    // Check if EEPROM has valid data (not 0xFFFFFFFF which is erased state)
    if saved_duration != 0xFFFF_FFFF && saved_duration != 0 {
        voltage_state.total_hit_duration_ms = saved_duration;
        rprintln!(
            "Loaded hit duration from EEPROM: {} ms ({:.1} s)",
            saved_duration,
            saved_duration as f32 / 1000.0
        );
    } else {
        rprintln!("No saved hit duration found in EEPROM, starting fresh");
    }

    let mut last_save_time = elapsed_ms;
    let mut last_saved_duration = voltage_state.total_hit_duration_ms;

    loop {
        buf.clear();

        let left_pressed = btn_left.is_high().unwrap();
        let right_pressed = btn_right.is_high().unwrap();
        let middle_pressed = btn_middle.is_high().unwrap();
        button_state.update(left_pressed, right_pressed, middle_pressed);
        let (left_debounced, right_debounced, middle_debounced) = button_state.is_debounced(7); // 7 out of 10 samples

        use core::fmt::Write;
        let val: u16 = adc.read(&mut an_in).unwrap();

        // Update voltage state with new sample
        voltage_state.update(val);
        let hit_just_ended = voltage_state.check_hit(elapsed_ms);

        // Save immediately when a hit ends
        if hit_just_ended {
            write_word(
                &dp.FLASH,
                HITTIME_OFFSET,
                voltage_state.total_hit_duration_ms,
            );
            last_saved_duration = voltage_state.total_hit_duration_ms;
            rprintln!(
                "Hit ended - saved duration to EEPROM: {} ms ({:.1} s)",
                voltage_state.total_hit_duration_ms,
                voltage_state.get_total_duration_seconds()
            );
        }

        // Format display to avoid wrapping
        write!(
            buf,
            "V:{:3} T:{:.1}s\n[{}{}{}] ",
            val,                                        // voltage (3 digits)
            voltage_state.get_total_duration_seconds(), // total time
            if left_debounced { "L" } else { "-" },
            if middle_debounced { "M" } else { "-" },
            if right_debounced { "R" } else { "-" },
        )
        .unwrap();

        // Add current hit indicator
        if voltage_state.is_hitting() {
            write!(
                buf,
                "HIT:{:.1}s",
                voltage_state.get_current_duration_ms() as f32 / 1000.0
            )
            .unwrap();
        }

        // Wait for timer interrupt
        while timer.wait().is_ok() {
            display.set_position(0, 0).unwrap();
            display.write_str(&buf).unwrap(); // overwrites previous chars
                                              // display.clear_buffer();      // no SPI traffic
                                              // display.write_str(&buf);
                                              // display.flush().unwrap();    // single SPI write
            timer.start(60_u32.Hz());
            elapsed_ms = elapsed_ms.wrapping_add(16); // ~60Hz = ~16ms per frame

            // Save to EEPROM every 10 seconds if value changed
            if elapsed_ms.wrapping_sub(last_save_time) > 10000 {
                if voltage_state.total_hit_duration_ms != last_saved_duration {
                    write_word(
                        &dp.FLASH,
                        HITTIME_OFFSET,
                        voltage_state.total_hit_duration_ms,
                    );
                    last_saved_duration = voltage_state.total_hit_duration_ms;
                    rprintln!(
                        "Saved hit duration to EEPROM: {} ms ({:.1} s)",
                        voltage_state.total_hit_duration_ms,
                        voltage_state.get_total_duration_seconds()
                    );
                }
                last_save_time = elapsed_ms;
            }

            rprintln!(
                "val={} total_s={:.1} hitting={}",
                val,
                voltage_state.get_total_duration_seconds(),
                voltage_state.is_hitting()
            );
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
