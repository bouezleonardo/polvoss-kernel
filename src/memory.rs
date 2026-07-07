//! Memory allocation mechanisms and layouts.
//!
//! The memory module contains the memory allocation
//! and the virtual memory mechanisms.

// Physical memory allocation
pub mod frame_alloc;

// Physical memory layout
mod memory_layout;

// Virtual memory mechanisms
mod virtual_memory;
