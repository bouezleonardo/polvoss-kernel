// mmio.rs

//! Memory mapped IO mechanisms.
//!
//! The mmio module contains the memory mapped IO
//! functionality to interact with IO devices that
//! are directly mapped into memory. Such devices
//! include the keyboard and monitor.

// I\O console
pub mod console;

// Print functions for the kernel
pub mod print;

// Monitor driver
mod monitor;

// UART driver
pub mod uart;
