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
mod syscall;
