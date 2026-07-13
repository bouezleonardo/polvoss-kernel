// mmio.rs

//! Memory mapped IO mechanisms.
//!
//! The mmio module contains the memory mapped IO
//! functionality to interact with IO devices that
//! are directly mapped into memory. Such devices
//! include the keyboard and monitor.

// Monitor driver
pub mod monitor;

// Print functions
pub mod print;

// UART driver
pub mod uart;
