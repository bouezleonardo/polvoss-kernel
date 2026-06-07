// riscv/supervisor_mode.rs

//! Supervisor mode inline assembly functions.
//!
//! The following functions execute inline assembly
//! code for Supervisor mode. They access Supervisor
//! mode Control and Status Registers (CSRs). Code 
//! running in Machine mode can also execute these 
//! functions as it is a higher operation mode. 

use core::arch::asm;

/****************|SIE REGISTER|******************/

// The Supervisor interrupt-enable register (sie) 
// register enables or disables individual
// interrupts in Supervisor mode. Read section
// 12.1.3. of RISC-V privileged doc.

/// sie timer interrupts enable code
pub const SIE_STIE: usize = 1 << 5;

/// sie external interrupts enable code
pub const SIE_SEIE: usize = 1 << 9;

/// Read sie register
pub fn read_sie() -> usize {
  let mut sie: usize;
  
  // csrr reads sie into {} register 
  unsafe { asm!("csrr {}, sie", out(reg) sie); }
  
  sie
}

/// Write to sie register
pub fn write_sie(sie: usize) {
  // csrw writes {} into sie register 
  unsafe { asm!("csrw sie, {}", in(reg) sie); }
}
