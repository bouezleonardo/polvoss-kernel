//! Processing mechanisms.
//!
//! This module contains mechanisms for
//! syncronization, context switch and 
//! cpu control

// Spin lock Mutex
pub mod spin;

// Control data structures
mod control_types;

pub mod proc;
