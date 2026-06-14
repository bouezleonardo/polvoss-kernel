// memory/kernel_layout.rs

//! Kernel's virtual memory layout.
//!
//! This module specifies and builds the
//! kernel's virtual memory layout by
//! constructing a page table that maps the
//! physical adresses to the same virtual
//! addresses except for the trampoline code
//! and the processes kernel stacks. This is done
//! to facilitate MMIO, since the virtual addresses 
//! are mostly the same as the physical ones. 
//! However the trampoline code must be in a known
//! position and the kernel stacks need guard pages
//! to control overflows. This is achieved more
//! easily with predefined virtual adresses.

// These simbols come from linker.ld
unsafe extern "C" {
  /// Address where the kernel starts (where entry is)
  static skernel: u8;
  /// Address where .text section ends
  static etext: u8;
  /// Address where the kernel memory ends
  static ekernel: u8;
}

pub fn skernel_addr() -> usize {
  unsafe { &skernel as *const u8 as usize }
}

pub fn etext_addr() -> usize {
  unsafe { &etext as *const u8 as usize }
}

pub fn ekernel_addr() -> usize {
  unsafe { &ekernel as *const u8 as usize }
}
