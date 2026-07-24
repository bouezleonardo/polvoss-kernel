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

/****************|SATP REGISTER|******************/

// The Supervisor Address Translation and Protection
// (satp) register controls supervisor mode address
// translation and protection and is used to enable
// virtual memory. Read section 12.1.11. of RISC-V 
// privileged doc.

/// No translation or protection
pub const SATP_BARE: usize = 0 << 31;

/// Page-based 32-bit virtual addressing
pub const SATP_SV32: usize = 1 << 31;

/// Read satp register
pub fn read_satp() -> usize {
  let mut satp: usize;
  
  // csrr reads sie into {} register 
  unsafe { asm!("csrr {}, satp", out(reg) satp); }
  
  satp
}

/// Write to satp register
pub fn write_satp(satp: usize) {
  // csrw writes {} into sie register 
  unsafe { asm!("csrw satp, {}", in(reg) satp); }
}

/// Flush the Translation lookaside buffer (TLB)
// This is done to synchonize the use of the page
// tables and avoid inconsistent states when multiple
// CPUs working at the same time
pub fn sfence_vma() {
  // zero, zero flush all TLB entries
  unsafe { asm!("sfence.vma zero, zero"); }
}

/*****************|TP REGISTER|*******************/

// The Thread Pointer (tp) register is used to
// store the CPU (hart) ID after leaving machine
// mode

// Read tp register
pub fn read_tp() -> usize {
  let mut tp: usize;
  
  unsafe { asm!{"li {}, tp", out(reg) tp}; }
  
  tp
}
