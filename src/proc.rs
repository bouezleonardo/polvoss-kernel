//! Processing mechanisms.
//!
//! This module contains mechanisms for
//! syncronization, context switch and 
//! cpu control.

// FIXME temporary for testing
mod init_test;
mod graphics_test;

// Spin lock Mutex
pub mod spin;

// Syncronization
pub mod sync;

// Control data structures
pub mod control_types;

// Processing control
pub mod processing;

// Process scheduler
pub mod scheduler;

// Round-robin scheduling algorithm
mod round_robin;

// Program loader
pub mod loader;
