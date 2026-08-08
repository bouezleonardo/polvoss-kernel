//! Trap (interrupts and exceptions) mechanisms.
//!
//! This module contains mechanisms for
//! handling traps.

// Type definitions for trap handling
pub mod trap_types;

// Kernel trap vector (assembly)
mod kernelvec;

// Rust trap handlers
mod trap_handlers;

// Trap codes for interrupts and exceptions
mod trap_codes;
