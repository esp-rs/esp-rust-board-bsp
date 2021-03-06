//! A board support package for the ESP Rust Board.
//!
//! <https://github.com/esp-rs/esp-rust-board/>
//!
//! Re-exports all dependencies necessary to interact with the various devices
//! present on this development kit. Additionally provides some utility macros
//! and functions to improve quality of life when developing for this board.

#![no_std]

pub use esp32c3_hal;
pub use esp_backtrace;
#[cfg(feature = "sensors")]
pub use icm42670;
pub use riscv_rt;
pub use shared_bus;
#[cfg(feature = "sensors")]
pub use shtcx;
#[cfg(feature = "smartled")]
pub use smart_leds;

/// Prints to the USB Serial JTAG peripheral, with a newline.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!($crate::esp32c3_hal::UsbSerialJtag, $($arg)*).ok();
        }
    };
}

/// Prints to the USB Serial JTAG peripheral.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!($crate::esp32c3_hal::UsbSerialJtag, $($arg)*).ok();
        }
    };
}
