//! Physical memory layout.
//!
//! This module contains data and
//! mechanisms about the physical memory
//! layout such as the kernel's start and
//! end addresses.

use crate::config::constants::{RAM_SIZE,
                               PAGE_SIZE};

// These simbols come from linker.ld
unsafe extern "C" {
  static skernel: u8;
  static etext: u8;
  static ekernel: u8;
}

/// Address where the kernel starts (where entry is)
pub fn skernel_addr() -> u64 {
  unsafe { &skernel as *const u8 as u64 }
}
/// Address where .text section ends
pub fn etext_addr() -> u64 {
  unsafe { &etext as *const u8 as u64 }
}
/// Address where the kernel memory ends
pub fn ekernel_addr() -> u64 {
  unsafe { &ekernel as *const u8 as u64 }
}

/// Last frame address
pub fn last_addr() -> u64 {
  // Last physical address in RAM
  skernel_addr() + RAM_SIZE as u64
}

/// First frame address
pub fn first_addr() -> u64 {
  let page_size: u64 = PAGE_SIZE as u64;
  
  // First multiple of PAGE_SIZE after ekernel
  ekernel_addr().div_ceil(page_size) * page_size
}
