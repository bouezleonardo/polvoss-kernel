//! Trap (interrupts and exceptions) mechanisms.
//!
//! This module contains mechanisms for
//! handling traps.

// Type definitions for trap handling
pub mod trap_types;

// Kernel trap vector (assembly)
mod kernelvec;

// User trap vector (assembly)
mod uservec;

// Rust trap handlers
pub mod trap_handlers;

// Trap codes for interrupts and exceptions
mod trap_codes;

// Platform Level Interrupt Controller (PLIC)
// mechanisms
pub mod plic;

// System call handling
mod syscall_handler;

// Process related system calls
pub mod syscall_proc;

// File system related system calls
mod syscall_file;

// Clock mechanisms
mod clock;

// External devices mechanisms
mod device;
