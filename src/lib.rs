//! A board support package for the ESP Rust Board.
//!
//! <https://github.com/esp-rs/esp-rust-board/>
//!
//! Re-exports all dependencies necessary to interact with the various devices
//! present on this development kit.

#![no_std]

pub use esp32c3_hal;
pub use esp_backtrace;
pub use esp_println::{print, println};
#[cfg(feature = "sensors")]
pub use icm42670;
pub use riscv_rt;
#[cfg(feature = "sensors")]
pub use shared_bus;
#[cfg(feature = "sensors")]
pub use shtcx;
#[cfg(feature = "smartled")]
pub use smart_leds;
