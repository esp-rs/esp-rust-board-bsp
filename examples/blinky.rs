#![no_std]
#![no_main]

use esp_rust_board::{
    esp32c3_hal::{
        clock::ClockControl, gpio::IO, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
    },
    esp_backtrace as _, println,
};

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Initialize the Delay peripheral and use it to toggle the LED in a loop.
    let mut delay = Delay::new(&clocks);

    // Set GPIO7 as an output, and set its state low initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio7.into_push_pull_output();
    led.set_low().unwrap();

    loop {
        println!("Toggle!");
        led.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}
