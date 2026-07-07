// riscv/memory_types.rs

//! Type definitions for memory management.
//!
//! The types defined in this module are intended to
//! make the use of pointers and other memory
//! manipulation mechanisms easier.

use core::ops::Add;
use crate::config::constants::{PAGE_SIZE};

// These types are intended for use on the Sv32 virtual
// memory scheme. Read section 12.3.1. of RISC-V privileged 
// doc.

/// Wrapper for an u64 that represents a memory address.
// The physical addresses in Sv32 are 34 bits, while
// the virtual ones are 32 bits, thus an u64 is suficient
// to represent both.
#[derive(Clone)] // Allows explicit cloning
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
  
  fn add(self, rhs:usize) -> Self{
    Self(self.0 + rhs as u64)
  }
}

/// Valid PTE field
const PTE_V: u8 = 1 << 0;
/// Read PTE field
const PTE_R: u8 = 1 << 1;
/// Write PTE field
const PTE_W: u8 = 1 << 2;
/// Execute PTE field
const PTE_X: u8 = 1 << 3;
/// User PTE field
const PTE_U: u8 = 1 << 4;

/// Wrapper for an usize that represents a PTE.
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
  /// Get the PTE in the specified `index`
  pub fn read_pte(&self, index: usize) -> PageTableEntry {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     
     let addr: Addr = self.as_addr()+index;
     
     addr.read::<PageTableEntry>()
  }
  /// Set the PTE in the specified `index`
  pub fn write_pte(&self, pte: PageTableEntry, index: usize) {
     // Panic if index is greater than 1023
     if index > 1023 {
       panic!("[page table]: invalid index.");
     }
     
     let addr: Addr = self.as_addr()+index;
     
     addr.write::<PageTableEntry>(pte);
  }
  
  /// Set the page table to `value`
  pub fn pageset(&self, value: u8) {
    self.0.memset(value, PAGE_SIZE);
  }
}
