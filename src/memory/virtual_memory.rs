//! Virtual memory mechanisms
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
use crate::memory::memory_layout::*;
use super::frame_alloc::{kmalloc};
use crate::config::constants::{PAGE_SIZE,
                               UART0,
                               M_BASE,
                               M_WIDTH,
                               M_HEIGHT,
                               RAM_SIZE};

/// Kernel's page table address. Should be modified
/// only when booting by CPU 0.
static mut KERNEL_PAGETABLE: u64 = 0;

/// Configure the PTEs for the page tables mapping virtual
/// addresses starting at va to physical addresses starting
/// at pa.
/// `va` and `size` must be page-aligned because the paging
/// scheme is based on [`crate::config::constants::PAGE_SIZE`]
/// bytes pages
/// # Arguments
/// - `pgt`: page table to search
/// - `va`: virtual address
/// - `pa`: physical address
/// - `size`: size of the mapping
/// # Return
/// `true` if the mapping is successful, `false` otherwise
fn map(pgt: PageTable, mut va: Addr, mut pa: Addr, size: usize, 
perm: u8) -> bool {
  let last: Addr;              // Last address to map
  let mut pte: PageTableEntry; // Leaf PTE
  let mut pgt_l: PageTable;    // Leaf page table
  let mut index: usize;        // Leaf PTE index inside the page table
  let mut opt: Option<(PageTable, usize)>; // Return from walk function
  
  if !size.is_multiple_of(PAGE_SIZE) {
    panic!("[virtual_memory]: size is not page aligned.");
  }
  
  if !va.as_integer().is_multiple_of(PAGE_SIZE as u64) {
    panic!("[virtual_memory]: virtual address is not page aligned.");
  }
  
  if size == 0 {
    panic!("[virtual_memory]: size of the mapping is zero.");
  }
  
  // Start at va, map to last
  last = va.clone() + size - PAGE_SIZE;
  while va <= last {
    // Walk the page table until the leaf PTE for the current
    // address is found 
    opt = walk(pgt.clone(), va.clone(), true);
    
    if opt.is_none() {
      return false;
    }
    
    // Get the leaf page table and index of the PTE
    (pgt_l, index) = opt.unwrap();
    
    // Get the PTE from the page table
    pte = pgt_l.read_pte(index);
    
    // If this PTE is already valid, it is a remap
    if pte.check_fields(PTE_V) {
      panic!("[virtual_memory]: remapping PTE.");
    }
    
    // Map the physical address in this PTE
    pte.to_pte(pa.clone());
    
    // Change permissions
    pte.write_fields(PTE_V | perm);
    
    // Write the PTE back to the page table
    pgt_l.write_pte(pte, index);
    
    // Next addresses to map
    va += PAGE_SIZE;
    pa += PAGE_SIZE;
  }
  
  return true;
}

/// Call the `map` function with an easier interface
fn kernel_map(pgt: PageTable, va: u64, pa: u64, size: usize, 
perm: u8) {
  if !map(pgt.clone(), Addr::new(va), Addr::new(pa), size, perm) {
    panic!("[virtual_memory]: unable to map kernel memory.");
  }
}

/// Build the kernel's virtual memory layout by
/// creating the kernel pagetable 
pub fn init_virtual_memory() {
  // Allocate a frame for the root
  let frame: Addr = kmalloc().expect("[virtual_memory]: unable to
                            allocate kernel page table.");
  
  // Create the page table in the frame
  let pgt: PageTable = PageTable::new(frame);
  
  // Set all bytes of the page to 0 to clear it
  pgt.pageset(0);
  
  // Map all memory
  kernel_map(pgt.clone(), UART0, UART0, RAM_SIZE, PTE_R|PTE_W);
  kernel_map(pgt.clone(), skernel_addr(), skernel_addr(), RAM_SIZE, PTE_R|PTE_X|PTE_W);

  unsafe { KERNEL_PAGETABLE = pgt.as_integer(); }
}

/// Use virtual memory in this CPU
pub fn use_virtual_memory(){
  unsafe {
    // Install the page table in the CPU
    install_page_table(KERNEL_PAGETABLE);
  }
}



/*/// Copy bytes from a user address into
/// a buffer
/// # Arguments
/// - `buf`: byte buffer
/// - `addr`: source address 
/// - `len`: number of bytes to copy
///
pub fn copy_in(buf: &mut, addr: Addr, len: usize) -> bool {
  
  

}*/
