#![no_std]
#![no_main]

use esp_rust_board::{
    esp32c3_hal::{
        clock::ClockControl,
        gpio::IO,
        pac::Peripherals,
        prelude::*,
        Delay,
        RtcCntl,
        Timer,
    },
    panic_halt as _,
    println,
    riscv_rt::entry,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);
    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut timer0 = Timer::new(peripherals.TIMG0);

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDT.
    rtc_cntl.set_super_wdt_enable(false);
    rtc_cntl.set_wdt_enable(false);
    timer0.disable();

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
