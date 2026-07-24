// riscv/memory_types.rs

//! Type definitions for memory management.
//!
//! The types defined in this module are intended to
//! make the use of pointers and other memory
//! manipulation mechanisms easier.

use core::ops::{Add, AddAssign, Sub};
use crate::config::constants::{PAGE_SIZE};
use crate::memory::frame_alloc::kmalloc;
use super::supervisor_mode::{write_satp, sfence_vma, SATP_SV32};

// These types are intended for use on the Sv32 virtual
// memory scheme. Read section 12.3.1. of RISC-V privileged 
// doc.

/********************|TYPES AND CONSTANTS|**********************/

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
  /// Get the address as an u64
  pub fn as_integer(&self) -> u64 {
    self.0
  }
  /// Get the address of a reference
  pub fn to_addr<T>(&mut self, var: &T) {
    self.0 = var as *const T as u64;
  }
  /// Dereference raw pointer and write to address
  pub fn write<T>(&self, value: T) {
    unsafe { (self.0 as *mut T).write(value) }
  }
  /// Dereference raw pointer and read from address
  pub fn read<T>(&self) -> T {
    unsafe { (self.0 as *const T).read() }
  }
  /// Write `value` to `count` bytes
  pub fn memset(&self, value: u8, count: usize) {
    unsafe { (self.0 as *mut u8).write_bytes(value, count) }
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

/// Wrapper for an usize that represents a PTE.
#[derive(Copy, Clone)] // Allows copy and cloning
pub struct PageTableEntry (usize);
impl PageTableEntry {
  /// Create a pte
  pub const fn new(pte: usize) -> Self {
    Self(pte)
  }
  /// Set the PTE from the physical address
  pub fn to_pte(&mut self, addr: Addr) {
    self.0 = ((addr.as_integer() >> 12) << 10) as usize;
  }
  /// Get the physical address from the PTE
  pub fn to_addr(&self) -> Addr {
    Addr::new(((self.0 as u64) >> 10) << 12)
  }
  /// Check if a PTE field (UXWRV) is set
  pub fn check_fields(&self, fields: u8) -> bool {
    if self.0 & fields as usize == 1 {
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
     // Each index corresponds to a PTE (4 bytes)
     let addr: Addr = self.as_addr()+(index * 4);
     
     addr.read::<PageTableEntry>()
  }
  /// Write to the PTE in the specified `index`
  pub fn write_pte(&self, pte: PageTableEntry, index: usize) {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     // Each index corresponds to a PTE (4 bytes)
     let addr: Addr = self.as_addr()+(index * 4);
     
     addr.write::<PageTableEntry>(pte);
  }
  /// Get the PTE address in the specified `index`
  pub fn pte_addr(&self, index: usize) -> Addr {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     // Each index corresponds to a PTE (4 bytes)
     self.as_addr()+(index * 4)
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
   pgt = PageTable::new(pte.to_addr());
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
   pte.to_pte(frame.unwrap());
   pte.write_fields(PTE_V);
 
   // Write the update the PTE in the page table
   pgt.write_pte(pte, index);
 
   // Next level page table (leaf page table)
   pgt = PageTable::new(pte.to_addr());
   // Clear the page
   pgt.pageset(0);
  } else {
    return None;
  }
  // Index of the leaf PTE (level 0) for va
  index = find_index(0, va.clone());
  
  Some((pgt, index))
}

pub fn install_page_table(addr: u64) {
  // Use Sv32 and remove offset from address
  let satp: usize = SATP_SV32 | (addr >> 12) as usize; 
  
  // Wait for writes to the page table memory to finish
  sfence_vma();
  
  write_satp(satp);
  
  // Flush TLB
  sfence_vma();
}
