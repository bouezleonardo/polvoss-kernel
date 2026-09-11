//! Physical memory layout.
//!
//! This module contains data and
//! mechanisms about the physical memory
//! layout such as the kernel's start and
//! end addresses.

use crate::config::constants::{RAM_SIZE, PAGE_SIZE};
use crate::riscv::memory_types::{MAX_VIRT_ADDR};
use crate::trap::uservec::*;

// These simbols come from linker.ld
unsafe extern "C" {
  /// First kernel address
  static skernel: u8;
  /// Last address of kernel's text section
  static etext: u8;
  /// Last address of kernel memory
  static ekernel: u8;
}

/// Virtual address of uservec. The uservec is the user
/// trap vector that saves the user registers and
/// changes from the process page table to the kernel page table.
/// It is mapped in the same location in the kernel and users 
/// address spaces
pub const USERVEC: usize = MAX_VIRT_ADDR - PAGE_SIZE;

/// Virtual address of trapframe. The trapframe is the region
/// of memory where the user data is stored when performing a trap.
pub const TRAPFRAME: usize = USERVEC - PAGE_SIZE;

/// Virtual address of a process' stack.
pub const PSTACK: usize = TRAPFRAME - PAGE_SIZE;

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

/// Address of the uservec code
pub fn uservec_addr() -> u64 {
  unsafe { uservec as *const() as u64 }
}

/// Return the virtual address of userret
pub fn userret_addr() -> usize {
  const VEC: *const() = uservec as *const();
  const RET: *const() = userret as *const();
  unsafe {
    USERVEC + (RET.offset_from(VEC)) as usize
  }
}
