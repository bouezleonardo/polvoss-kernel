// boot.rs

//! Define the submodules in src/boot.
//!
//! The code in boot should only be executed in the
//! context of the booting, thus it is not public. 
//! The boot module contains the kernel entry point,
//! hardware configuration and startup of the kernel
//! functionality.

// Kernel entry point
mod entry;

// Configures hardware
mod hard_config;

// Starts the higher kernel functionality
mod start;
