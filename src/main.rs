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
use crate::mmio::monitor::clear_screen;

// Kernel module definitions
mod config; // Configuration
mod boot;   // Kernel boot and initialization
mod riscv;  // Inline assembly code functions
mod memory; // Memory allocation and layout
mod mmio;   // Memory mapped IO functionality
mod proc;

/// Panic function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  clear_screen();
  print!("PANIC!\r\n{}", info.message());
  loop {}
}
