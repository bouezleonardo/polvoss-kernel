// config/constants.rs

//! Define configuration constants.

/// Page size in bytes
pub const PAGE_SIZE: usize = 4096;

/// Number of CPUs
pub const NUM_CPU: usize = 1;

/********************|MMIO|**********************/

/// Base address of the memory mapped monitor
pub const M_BASE: usize = 0x10000000;

/// Monitor character width
pub const M_WIDTH: usize = 80;

/// Monitor character height
pub const M_HEIGHT: usize = 25;
