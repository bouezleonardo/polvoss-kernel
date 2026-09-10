// riscv/memory_types.rs

//! Type definitions for memory management.
//!
//! The types defined in this module are intended to
//! make the use of pointers and other memory
//! manipulation mechanisms easier.

use core::ops::{Add, AddAssign, Sub};
use crate::config::constants::{PAGE_SIZE};
use crate::memory::frame_alloc::{kfree, kmalloc};
use super::supervisor_mode::{write_satp, sfence_vma, SATP_SV32};

// These types are intended for use on the Sv32 virtual
// memory scheme. Read section 12.3.1. of RISC-V privileged 
// doc.

/********************|TYPES AND CONSTANTS|**********************/

/// Max virtual address on Sv32 paging scheme subtracted
/// to be page aligned
pub const MAX_VIRT_ADDR: usize = usize::MAX - PAGE_SIZE + 1;

/// Wrapper for an u64 that represents a memory address.
// The physical addresses in Sv32 are 34 bits, while
// the virtual ones are 32 bits, thus an u64 is suficient
// to represent both.
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Addr (u64);
impl Addr {
  /// Create an address
  pub const fn new(addr: u64) -> Self {
    Self(addr)
  }
  /// Get the address of a reference
  pub fn to_addr<T>(var: &T) -> Self {
    Self(var as *const T as u64)
  }
  /// Get the address as an u64
  pub fn as_integer(&self) -> u64 {
    self.0
  }
  /// Get the address as a pointer
  pub fn as_ptr<T>(&self) -> *const T {
    self.0 as *const T
  }
  /// Dereference raw pointer and write to address
  pub fn write<T>(&self, value: T) {
    unsafe { (self.0 as *mut T).write(value) }
  }
  /// Dereference raw pointer and read from address
  pub fn read<T>(&self) -> T {
    unsafe { (self.0 as *const T).read() }
  }
  /// Write `value` to `len` bytes
  pub fn memset(&self, value: u8, len: usize) {
    unsafe { (self.0 as *mut u8).write_bytes(value, len) }
  }
  /// Copy `len` bytes from `src`
  pub fn copy<T>(&self, src: Addr, len: usize) {
    let ptr: *const T = src.as_ptr::<T>();
    unsafe { (self.0 as *mut T).copy_from(ptr, len) }
  }
}

// Define addition between an Addr and usize
impl Add<usize> for Addr {
  // The output is another Addr
  type Output = Self;
  
  fn add(self, num: usize) -> Self {
    Self(self.0 + num as u64)
  }
}

// Define addition and assign for an usize
impl AddAssign<usize> for Addr {
  fn add_assign(&mut self, num: usize) {
    self.0 = self.0 + num as u64;
  }
}

// Define subtraction between an Addr and usize
impl Sub<usize> for Addr {
  type Output = Self;
  
  fn sub(self, num: usize) -> Self {
    Self(self.0 - num as u64)
  }
}

/// Valid PTE field
pub const PTE_V: u8 = 1 << 0;
/// Read PTE field
pub const PTE_R: u8 = 1 << 1;
/// Write PTE field
pub const PTE_W: u8 = 1 << 2;
/// Execute PTE field
pub const PTE_X: u8 = 1 << 3;
/// User PTE field
pub const PTE_U: u8 = 1 << 4;
/// PTE size (bytes)
pub const PTE_SIZE: usize = 4;

/// Wrapper for an usize that represents a PTE.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct PageTableEntry (usize);
impl PageTableEntry {
  /// Create a pte
  pub const fn new(pte: usize) -> Self {
    Self(pte)
  }
  /// Set the physical address in the PTE
  pub fn set_addr(&mut self, addr: Addr) {
    // Clear address field
    self.0 &= !(1 << 10);
    // Save address 
    self.0 |= ((addr.as_integer() >> 12) << 10) as usize;
  }
  /// Get the physical address from the PTE
  pub fn get_addr(&self) -> Addr {
    Addr::new(((self.0 as u64) >> 10) << 12)
  }
  /// Check if a PTE field (UXWRV) is set
  pub fn check_fields(&self, field: u8) -> bool {
    if self.0 & field as usize != 0 {
      return true;
    }
    false
  }
  /// Get PTE fields 
  pub fn read_fields(&self) -> u8 {
    self.0 as u8
  }
  /// Set PTE fields 
  pub fn write_fields(&mut self, fields: u8) {
    self.0 |= fields as usize;
  }
}

/// Wrapper for an Addr that represents a page table.
#[derive(Clone)] // Allows explicit cloning
pub struct PageTable (Addr);
impl PageTable {
  /// Create a page table
  pub const fn new(addr: Addr) -> Self {
    Self(addr)
  }
  /// Get a clone of the page table's address
  pub fn as_addr(&self) -> Addr {
    self.0.clone()
  }
  /// Get the address as an u64
  pub fn as_integer(&self) -> u64 {
    self.0.as_integer()
  }
  /// Read the PTE in the specified `index`
  pub fn read_pte(&self, index: usize) -> PageTableEntry {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     // Each index corresponds to a PTE
     let addr: Addr = self.as_addr()+(index * PTE_SIZE);
     
     addr.read::<PageTableEntry>()
  }
  /// Write to the PTE in the specified `index`
  pub fn write_pte(&self, pte: PageTableEntry, index: usize) {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     // Each index corresponds to a PTE
     let addr: Addr = self.as_addr()+(index * PTE_SIZE);
     
     addr.write::<PageTableEntry>(pte);
  }
  /// Get the PTE address in the specified `index`
  pub fn pte_addr(&self, index: usize) -> Addr {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     // Each index corresponds to a PTE
     self.as_addr()+(index * PTE_SIZE)
  }
  /// Set the page table to `value`
  pub fn pageset(&self, value: u8) {
    self.0.memset(value, PAGE_SIZE);
  }
}

/********************|AUXILIARY FUNCTONS|***********************/

/// Get PTE index from `va` in the level 
/// # Arguments
/// - `level`: page table level
/// - `va`: virtual address
/// # Return
/// The PTE's index in the page table
pub fn find_index(level: usize, va: Addr) -> usize {
  // Remove the offset
  let mut aux: u64 = va.as_integer() >> 12;

  // Put the index for this level in the first 10 bits
  aux = aux >> (level*10);

  // Get the 10 first bits, zero out the rest
  aux &= 0x3FF;

  aux as usize
}

/// Walk all the levels of the page table until the leaf
/// PTE for virtual address `va` is found.
/// # Arguments
/// - `pgt`: page table to search
/// - `va`: virtual address
/// - `alloc`: allocate new page if PTE is not valid
/// # Return
/// The page table and index for the PTE
pub fn walk(mut pgt: PageTable, va: Addr, alloc: bool)
-> Option<(PageTable, usize)> {
  let mut pte: PageTableEntry; // PTE
  let mut index: usize;        // Page table index
  let frame: Option<Addr>; // Frame for a new page table
 
  // Index of the PTE in level 1 for va
  index = find_index(1, va.clone());

  // Read the PTE in this index
  pte = pgt.read_pte(index);
  
  // Check if PTE is valid
  if pte.check_fields(PTE_V) {
   // Next level page if it valid
   pgt = PageTable::new(pte.get_addr());
  } else if alloc {
   // Allocate a frame for the next level page
   // if it is not valid
   frame = kmalloc();
 
   // Check if kmalloc was successful
   if frame.is_none() {
     return None;
   }

   // Create the PTE that stores the address for the
   // leaf page table
   pte.set_addr(frame.unwrap());
   pte.write_fields(PTE_V);
 
   // Write the update the PTE in the page table
   pgt.write_pte(pte, index);
 
   // Next level page table (leaf page table)
   pgt = PageTable::new(pte.get_addr());
   // Clear the page
   pgt.pageset(0);
  } else {
    return None;
  }
  // Index of the leaf PTE (level 0) for va
  index = find_index(0, va.clone());
  
  Some((pgt, index))
}

/// Walk the levels of the pagetable to
/// determine which virtual addresses are mapped
/// and free their physical memory 
/// # Arguments
/// - `pgt1`: pagetable to be freed
pub fn free_addr_space(pgt1: PageTable) {
  // Number of PTEs in a page
  const NUM_PTE: usize = PAGE_SIZE/PTE_SIZE;
  
  let mut pgt0: PageTable;
  let mut pte1: PageTableEntry;
  let mut pte0: PageTableEntry;
  
  // Walk through level 1 page
  for i in 0..NUM_PTE {
    pte1 = pgt1.read_pte(i);
    
    // Check if the PTE is empty
    if pte1 == PageTableEntry(0) {
      continue;
    }
    // Walk through level 0 page
    pgt0 = PageTable::new(pte1.get_addr());
    for j in 0..NUM_PTE {
      pte0 = pgt0.read_pte(j);
      
      // Check if the PTE is empty
      if pte0 == PageTableEntry(0) {
        continue;
      }
      // Free contents
      kfree(pte0.get_addr());
    }
    // Free level 0 page
    kfree(pte1.get_addr());
  }
  // Free level 1 page
  kfree(pgt1.as_addr());
}

/// Walk the levels of the pagetable to
/// determine which virtual addresses are mapped
/// and allocate new physical ones to them 
/// # Arguments
/// - `pgt1`: pagetable to be copied
/// # Return
/// Option containing the new pagetable, None if
/// the copy was unsuccessful
pub fn 
copy_addr_space(pgt1: PageTable) 
-> Option<PageTable> {
  // Number of PTEs in a page
  const NUM_PTE: usize = PAGE_SIZE/PTE_SIZE;
  
  // Allocate a frame for the new pgt
  let mut addr_opt: Option<Addr> = kmalloc();
  if addr_opt.is_none() {
    return None;
  }
  // Indicate if there was something to copy
  let mut empty: bool = true;
  // Level 1 new page table
  let mut new_pgt1: PageTable;
  // Level 0 new page table
  let mut new_pgt0: PageTable;
  // Level 0 target page table
  let mut pgt0: PageTable;
  let mut addr: Addr = addr_opt.unwrap();
  let mut pte: PageTableEntry;
  
  new_pgt1 = PageTable::new(addr);
  
  // Walk through level 1
  for i in 0..NUM_PTE {
    pte = pgt1.read_pte(i);
    
    // Check if the PTE is empty
    if pte == PageTableEntry(0) {
      continue;
    }
    
    // Allocate a new physical addr for the
    // next pagetable in new_pgt1
    addr_opt = kmalloc();
    if addr_opt.is_none() {
      free_addr_space(new_pgt1);
      return None;
    }
    addr = addr_opt.unwrap();
    
    // Level 0 page tables
    pgt0 = PageTable::new(pte.get_addr());
    new_pgt0 = PageTable::new(addr.clone()); 
    
    // Save new address in the PTE
    pte.set_addr(addr);
    // Save PTE in the page table
    new_pgt1.write_pte(pte, i);
    
    // Walk through level 0
    for j in 0..NUM_PTE {
      pte = pgt0.read_pte(j);
    
      if pte == PageTableEntry(0) {
        continue;
      }
      
      addr_opt = kmalloc();
      if addr_opt.is_none() {
        free_addr_space(new_pgt1);
        return None;
      }
      
      empty = false;
      addr = addr_opt.unwrap();
      
      // Copy contents
      addr.copy::<u8>(pte.get_addr(), PAGE_SIZE);
      
      // Save pte
      pte.set_addr(addr);
      new_pgt0.write_pte(pte, j);
    }
  }
  
  if empty {
    free_addr_space(new_pgt1);
    return None;
  }
  
  Some(new_pgt1)
}

/// Format address to satp register
pub fn satp_format(addr: u64) -> usize {
  // Use Sv32 and remove offset from address
  SATP_SV32 | (addr >> 12) as usize 
}

/// Install pagetable in the satp register
pub fn install_page_table(addr: u64) {
  // Use Sv32 and remove offset from address
  let satp: usize = satp_format(addr); 
  
  // Wait for writes to the page table memory to finish
  sfence_vma();
  
  write_satp(satp);
  
  // Flush TLB
  sfence_vma();
}

// Previous address multiple of page or 0 based on addr
pub fn prev_page(addr: Addr) -> Addr {
  Addr::new(addr.as_integer() & !(PAGE_SIZE - 1) as u64)
}
