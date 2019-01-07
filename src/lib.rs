//! [Hardware Abstraction Layer](https://crates.io/crates/embedded-hal) (HAL)
//! for NXP LPC11UXX family of µ-controllers.
#![no_std]

pub use lpc11uxx;

pub mod delay;
pub mod serial;
