// riscv/utils.rs

//! Inline assembly utility functions.
//!
//! The following functions execute inline assembly
//! code for various porposes. They do not access
//! Control and Status Registers (CSRs).

use core::arch::asm;

/// Read a byte from an address in RAM
/// # Arguments
///  - `addr`: address to read from
/// # Return
/// The byte stored in this address
pub fn read_byte(addr: usize) -> u8 {
  let mut byte: u8;
  unsafe {
    asm!(
      "lb {0}, 0({1})", // Load byte
      out(reg) byte,
      in(reg) addr
    );
  }
  
  byte
}

/// Write a byte into RAM given an address
/// # Arguments
///  - `byte`: byte to write
///  - `addr`: address to write into
pub fn write_byte(byte: u8, addr: usize) {
  unsafe {
    asm!(
      "sb {0}, 0({1})", // Store byte
      in(reg) byte,
      in(reg) addr
    );
  }
}
