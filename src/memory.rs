// memory.rs

//! Memory allocation mechanisms and layouts.
//!
//! The memory module contains the memory allocation
//! and the virtual memory layout for the kernel.

// Physical memory allocation
pub mod frame_alloc;

// Type definitions for memory management
pub mod types;

// Kernel virtual memory layout
mod kernel_layout;
