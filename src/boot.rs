// boot.rs

//! Boot mechanism.
//!
//! The code in this module should only be executed in
//! the context of the booting, thus it is not public. 
//! The boot module contains the kernel entry point,
//! hardware configuration and startup of the higher
//! kernel functionality.

// Kernel entry point
mod entry;

// Configures hardware
mod hard_config;

// Starts the higher kernel functionality
mod start;
