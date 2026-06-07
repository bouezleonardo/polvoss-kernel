// riscv.rs

//! Define the submodules in src/riscv.
//!
//! The riscv module contains useful assembly code
//! to control the hardware in a lower level when
//! needed.

// Machine mode inline assembly functions
pub mod machine_mode;

// Supervisor mode inline assembly functions
pub mod supervisor_mode;

// Inline assembly utility functions
pub mod utils;
