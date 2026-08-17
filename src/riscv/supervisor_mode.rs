// riscv/supervisor_mode.rs

//! Supervisor mode inline assembly functions.
//!
//! The following functions execute inline assembly
//! code for Supervisor mode. They access Supervisor
//! mode Control and Status Registers (CSRs). Code 
//! running in Machine mode can also execute these 
//! functions as it is a higher operation mode. 

use core::arch::asm;

/**************|sstatus REGISTER|****************/

// The Supervisor Status (sstatus) register 
// contains information about a CPU's operating
// state. Read section 12.1.1. of RISC-V privileged
// doc.

/// SPP Supervisor mode code
pub const SPP_S: usize = 1 << 8;

/// SPP User mode code
pub const SPP_U: usize = 0 << 8;

/// Global enable bit for interrupts
pub const SSTATUS_SIE: usize = 1 << 1;

/// Read sstatus register
pub fn read_sstatus() -> usize {
  let mut sstatus: usize;
  
  // csrr reads sstatus into {} register 
  unsafe{ asm!("csrr {}, sstatus", out(reg) sstatus); }
  
  sstatus
}

/// Write to sstatus register
pub fn write_sstatus(sstatus: usize) {

  // csrw writes {} into sstatus
  unsafe{ asm!("csrw sstatus, {}", in(reg) sstatus); }
}

/****************|SIE AND sip|******************/

// The Supervisor interrupt-enable (sie) 
// register enables or disables individual
// interrupts in Supervisor mode. Read section
// 12.1.3. of RISC-V privileged doc.

/// sie software interrupts enable code
pub const SIE_SSIE: usize = 1 << 1;

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

/// Read sip register
pub fn read_sip() -> usize {
  let mut sip: usize;
  
  // csrr reads sie into {} register 
  unsafe { asm!("csrr {}, sip", out(reg) sip); }
  
  sip
}

/// Write to sip register
pub fn write_sip(sip: usize) {
  // csrw writes {} into sie register 
  unsafe { asm!("csrw sip, {}", in(reg) sip); }
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
  
  unsafe { asm!{"mv {}, tp", out(reg) tp}; }
  
  tp
}

/// Write to satp register
pub fn write_tp(tp: usize) {
  // csrw writes {} into tp register 
  unsafe { asm!("csrw satp, {}", in(reg) tp); }
}

/***************|STVEC REGISTER|*****************/

// The Supervisor Trap Vector Base Address (stvec)
// register holds the trap vector's address. When
// a trap (interrupt or exception) happens, the
// code in the address saved in stvec will be 
// executed. Read section 12.1.2. of RISC-V 
// privileged doc.

// Write stvec register
pub fn write_stvec(stvec: usize) {
  unsafe { asm!{"csrw stvec, {}", in(reg) stvec}; }
}

/**************|SCAUSE REGISTER|****************/

// The  Supervisor Cause (scause) register 
// indicates which interrupt or exception happened.
// Read section 12.1.8. of RISC-V privileged doc.

// Read scause register
pub fn read_scause() -> usize {
  let mut scause: usize;
  
  unsafe { asm!{"csrr {}, scause", out(reg) scause}; }
  
  scause
}

/**************|STVAL REGISTER|****************/

// The  Supervisor Trap Value (stval) register 
// offers additional information about the trap.
// Read section 12.1.9. of RISC-V privileged doc.

// Read scause register
pub fn read_stval() -> usize {
  let mut stval: usize;
  
  unsafe { asm!{"csrr {}, stval", out(reg) stval}; }
  
  stval
}

/*******************|SRET|*********************/

// The sret instruction returns from Supervisor
// mode and switches to the mode specified in SPP.
// It returns to the address saved in sepc. Read
// section 12.1.7 of RISC-V privileged doc.

/// Return from Supervisor mode
pub fn sret() -> ! {
  unsafe{ asm!("sret", options(noreturn)); }
}

/// Write to sepc register
pub fn write_sepc(addr: usize) {
  unsafe{ asm!("csrw sepc, {}", in(reg) addr); }
}

// Read sepc register
pub fn read_sepc() -> usize {
  let mut sepc: usize;
  
  unsafe { asm!{"csrr {}, sepc", out(reg) sepc}; }
  
  sepc
}

/*******************|TIME|*********************/

// The time register stores a count that the
// hardware increments at a steady rate. The 
// stimecmp register contains a value of time in
// which there will be a timer interrupt. Read
// sections 12.1.4 and 12.1.12 of RISC-V 
// privileged doc.

// In RISC-V 32 bits, time and stimecmp are split in two
// halfs of 32 bits.

pub fn read_time() -> u64 {
  let mut time_l: u32;
  let mut time_h: u32;
  
  unsafe {
    // Low 32 bits of time register
    asm!{"csrr {}, time", out(reg) time_l};
    // Upper 32 bits of time register
    asm!{"csrr {}, timeh", out(reg) time_h};
  }
  
  time_l as u64 + ((time_h as u64) << 32)
}

/// Write to the full stimecmp register
pub fn write_stimecmp(count: u64) {
  let count_l: u32 = count as u32;
  let count_h: u32 = (count >> 32) as u32;
  
  unsafe{
    // Low 32 bits of stimecmp register
    asm!("csrw stimecmp, {}", in(reg) count_l);
    // Upper 32 bits of stimecmp register
    asm!("csrw stimecmph, {}", in(reg) count_h);
  }
}

/// Read to the full stimecmp register
pub fn read_stimecmp() -> u64{
  let count_l: u32;
  let count_h: u32;
  
  unsafe{
    // Low 32 bits of stimecmp register
    asm!("csrr {}, stimecmp", out(reg) count_l);
    // Upper 32 bits of stimecmp register
    asm!("csrr {}, stimecmph", out(reg) count_h);
  }
  
  count_l as u64 + ((count_h as u64) << 32)
}

/****************|AUXILIARY|*****************/

/// Check if interrupts are globally enabled
pub fn intr_enabled() -> bool {
  if read_sstatus() & SSTATUS_SIE == SSTATUS_SIE {
    return true;
  }
  false
}
/// Enable interrupts globally
pub fn intr_on() {
  write_sstatus(read_sstatus() | SSTATUS_SIE);
}
/// Disable interrupts globally
pub fn intr_off() {
  write_sstatus(read_sstatus() & !SSTATUS_SIE);
}
