// memory.rs

//! Type definitions for memory management.
//!
//! The types defined in this module are intended
//! to make the use of pointers and other memory
//! manipulation mechanisms easier.

use core::ops::Add;

/// Wrapper for a usize that represents physical memory
pub struct PhysAddr (usize);
impl PhysAddr {
  /// Constructor
  pub fn new(addr: usize) -> Self {
    Self(addr)
  }
  /// Get the address
  pub fn get_addr(&self) -> usize {
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
}

impl Add<usize> for PhysAddr {
  // The output is another PhysAddr
  type Output = Self;
  
  fn add(self, rhs:usize) -> Self{
    Self(self.0 + rhs)
  }
}
