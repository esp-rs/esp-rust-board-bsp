#![no_std]
#![no_main]

use esp_rust_board::{
    esp32c3_hal::{
        clock::ClockControl,
        pac::Peripherals,
        prelude::*,
        pulse_control::ClockSource,
        timer::TimerGroup,
        utils::{smartLedAdapter, SmartLedsAdapter},
        Delay, PulseControl, Rtc, IO,
    },
    esp_backtrace as _,
    smart_leds::{
        brightness, gamma,
        hsv::{hsv2rgb, Hsv},
        SmartLedsWrite,
    },
};

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDT.
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Configure RMT peripheral globally
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();

    // We use one of the RMT channels to instantiate a `SmartLedsAdapter` which can
    // be used directly with all `smart_led` implementations
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = <smartLedAdapter!(1)>::new(pulse.channel0, io.pins.gpio2);

    // Initialize the Delay peripheral and use it to toggle the LED in a loop.
    let mut delay = Delay::new(&clocks);

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data;

    loop {
        // Iterate over the rainbow!
        for hue in 0..=255 {
            color.hue = hue;
            // Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED
            data = [hsv2rgb(color)];
            // When sending to the LED, we do a gamma correction first (see smart_leds
            // documentation for details) and then limit the brightness to 10 out of 255 so
            // that the output is not too bright.
            led.write(brightness(gamma(data.iter().cloned()), 10))
                .unwrap();

            delay.delay_ms(20u8);
        }
    }
}
