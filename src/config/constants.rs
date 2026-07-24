//! Define configuration constants.

/*******************|SYSTEM|*********************/

/// Page size in bytes
pub const PAGE_SIZE: usize = 4096;

/// Maximum number of processes
pub const NUM_PROC: usize = 64;

/******************|HARDWARE|********************/

/// Number of CPUs
pub const NUM_CPU: usize = 1;

/// Size of main memory in bytes
pub const RAM_SIZE: usize = 128 * 1024 * 1024;

/********************|MMIO|**********************/

/// Base address for Plataform-Level Interrupt
/// Controller. The PLIC is used to discover which
/// external device interrupted
pub const PLIC: u64 = 0x0C000000;

/// Base address of the UART devices. Used to read
/// input from the keyboard
pub const UART0: u64 = 0x10000000;

/// Base address of the memory mapped monitor
pub const M_BASE: u64 = 0x10000000;

/// Monitor character width
pub const M_WIDTH: usize = 80;

/// Monitor character height
pub const M_HEIGHT: usize = 24;
