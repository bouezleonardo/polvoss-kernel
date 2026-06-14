#![no_std]   // No standard crate
#![no_main]  // No main function

// main.rs

//! POLVOSS kernel.
//!
//! The Privilegeless Open Learnable
//! Virtualized Operating System Simulator
//! (POLVOSS) is an open, easy to understand and
//! modify operating system simulator for learning
//! purposes. Therefore, the POLVOSS kernel is
//! designed to be monolithic, but with highly modular
//! code organization.

// This file defines the kernel modules and the panic  
// function. This is not the entry point of the kernel, 
// look for the entry.rs file in the boot module.

use core::panic::PanicInfo;
use crate::mmio::monitor::monitor_putc_at;

// Kernel module definitions
mod config; // Configuration
mod boot;   // Kernel boot and initialization
mod riscv;  // Inline assembly code functions
mod mmio;   // Memory mapped IO functionality
mod memory; // Memory allocation and layout
mod proc;

/// Panic function
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
  monitor_putc_at(b' ', 0, 0);
  monitor_putc_at(b'p', 0, 0);
  monitor_putc_at(b'a', 0, 0);
  monitor_putc_at(b'n', 0, 0);
  monitor_putc_at(b'i', 0, 0);
  monitor_putc_at(b'c', 0, 0);
  monitor_putc_at(b'!', 0, 0);
  monitor_putc_at(b' ', 0, 0);
  loop {}
}
