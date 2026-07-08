//! Virtual memory layout mechanisms
//!
//! This module builds the
//! kernel's virtual memory layout by
//! constructing a page table that maps the
//! physical adresses to the same virtual
//! addresses except for the trampoline code
//! and the processes kernel stacks. This is done
//! to facilitate the construction of the kernel's
//! memory region, since the trampoline code must be
//! in a known position and the kernel stacks need
//! guard pages to control overflows. This is achieved
//! more easily with predefined virtual addresses.

use crate::riscv::memory_types::*;

use super::frame_alloc::{kmalloc};

use crate::config::constants::{UART0,
                               M_BASE,
                               M_WIDTH,
                               M_HEIGHT};

/// Address of the kernel's page table. Should be 
/// modified only when booting by CPU 0
static mut KERNEL_PAGETABLE: PageTable = PageTable::new(Addr::new(0));

/// Create the PTEs for the page tables mapping virtual
/// addresses starting at va to physical addresses starting
/// at pa.
/// `va` and `size` must be page-aligned because the paging
/// scheme is based on [`crate::config::constants::PAGE_SIZE`]
/// bytes pages.
/*fn mappages(pgt: PageTable, va: Addr, pa: Addr, size: usize, 
perm: u8) -> bool {
  
  
}*/

/// Build the kernel's virtual memory layout by
/// creating the kernel pagetable 
pub fn init_virtual_memory(){
  // Allocate a frame for the root
  let frame: Addr = kmalloc().expect("[virtual_memory]: Unable to
                            allocate kernel page table.");
  
  // Create the page table in the frame
  let pgt: PageTable = PageTable::new(frame);
  
  // Set all bytes of the page to 0 to clear it
  pgt.pageset(0);
  
  unsafe { KERNEL_PAGETABLE = pgt; }
}
