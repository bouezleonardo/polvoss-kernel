#![no_std]   // No standard crate
#![no_main]  // No main function

// main.rs

//! Root to define the modules and panic function.
//!
//! Define the kernel modules and the panic function. This
//! is not the entry point of the kernel, look for the
//! entry.rs file in the boot module.

use core::panic::PanicInfo;

// Kernel module definitions
mod config; // Configuration
mod boot;   // Kernel boot and initialization
mod riscv;  // Inline assembly code functions
mod mmio;   // Memory mapped IO functionality

// Panic function
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
