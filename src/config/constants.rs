//! Define configuration constants.

/******************|DEVICES|*********************/

/// Console device number
pub const CONSOLE: u16 = 1;

/*******************|SYSTEM|*********************/

/// Maximum number of processes
pub const NUM_PROC: usize = 64;

/// Maximum number of open files
pub const NUM_FILE: usize = 64;

// Number of pages for a process' kernel stack
pub const KSTACK_SIZE: usize = 2;

// Number of pages for a process' user stack
pub const USTACK_SIZE: usize = 3;

/// Time for the timer interrupts (ms)
pub const TICK_TIME: u64 = 10;

/// Size of disk block
pub const SECTOR_SIZE: usize = 512; 

/******************|HARDWARE|********************/

/// Page size in bytes
pub const PAGE_SIZE: usize = 4096;

/// Number of CPUs
pub const NUM_CPU: usize = 1;

/// Size of main memory in bytes
pub const RAM_SIZE: usize = 128 * 1024 * 1024;

/// Amount of clock increments that correspond to 1 ms
pub const MILISECOND: u64 = 10000;

/********************|MMIO|**********************/

/// Base address for Plataform-Level Interrupt
/// Controller. The PLIC is used to discover which
/// external device interrupted
pub const PLIC: u64 = 0x0C000000;

/// Base address of the UART devices. Used to read
/// input from the keyboard
pub const UART0: u64 = 0x10000000;

/// UART0 Interrupt Request (IRQ). Used as an ID
pub const UART0_IRQ: u32 = 10;

/// Base address of the memory mapped monitor
pub const M_BASE: u64 = 0x10000000;

/// Monitor character width
pub const M_WIDTH: usize = 80;

/// Monitor character height
pub const M_HEIGHT: usize = 24;
