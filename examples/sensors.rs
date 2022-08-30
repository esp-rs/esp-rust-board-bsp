#![no_std]
#![no_main]

use esp_rust_board::{
    esp32c3_hal::{
        clock::ClockControl, i2c::I2C, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
        IO,
    },
    esp_backtrace as _,
    icm42670::{accelerometer::Accelerometer, Address, Icm42670},
    print, println,
    shared_bus::BusManagerSimple,
    shtcx::{shtc3, LowPower, PowerMode},
};

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
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

    // Initialize the I2C bus using GPIO10 for SDA and GPIO8 for SCL, running at
    // 400kHz.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        400u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    )
    .unwrap();

    // Create a bus manager so that we can share the I2C bus between sensor drivers
    // while avoiding ownership issues.
    let bus = BusManagerSimple::new(i2c);
    let mut icm = Icm42670::new(bus.acquire_i2c(), Address::Primary).unwrap();
    let mut sht = shtc3(bus.acquire_i2c());

    // The SHTC3 temperature/humidity sensor must be woken up prior to reading.
    sht.wakeup(&mut delay).unwrap();

    loop {
        // Read and display normalized accelerometer and gyroscope values.
        let accel_norm = icm.accel_norm().unwrap();
        let gyro_norm = icm.gyro_norm().unwrap();

        print!(
            "ACCEL = X: {:+.04} Y: {:+.04} Z: {:+.04}\t",
            accel_norm.x, accel_norm.y, accel_norm.z
        );
        print!(
            "GYRO  = X: {:+.04} Y: {:+.04} Z: {:+.04}\t",
            gyro_norm.x, gyro_norm.y, gyro_norm.z
        );

        // Read and display temperature and relative humidity values.
        let measurement = sht.measure(PowerMode::NormalMode, &mut delay).unwrap();

        print!(
            "TEMP  = {:+.2} °C\t",
            measurement.temperature.as_degrees_celsius()
        );
        println!("RH   = {:+.2} %RH", measurement.humidity.as_percent());

        delay.delay_ms(250u32);
    }
}
