//! Trap handling related types.
//!
//! This module contains data strutures necessary
//! for trap handling.

/// These registers represent a context of
/// execution and must be preserved to make 
/// a context switch. This is used to switch
/// from a kernel's context to the scheduler
/// and back. Read chapter 1.1. of
/// the RISCV ABI doc.
#[derive(Copy, Clone)]
#[repr(C)] // Ensure struct follows C memory layout
pub struct Context {
  pub ra: usize,  // Return address
  pub sp: usize,  // Stack pointer
  // Preserved across calls
  pub s0: usize,
  pub s1: usize,
  pub s2: usize,
  pub s3: usize,
  pub s4: usize,
  pub s5: usize,
  pub s6: usize,
  pub s7: usize,
  pub s8: usize,
  pub s9: usize,
  pub s10: usize,
  pub s11: usize,
}
impl Context {
  // Initialize a default Context
  pub const fn new() -> Self {
    Self {ra: 0, sp: 0, s0: 0, s1: 0,
      s2: 0, s3: 0, s4: 0, s5: 0,
      s6: 0, s7: 0, s8: 0, s9: 0, 
      s10: 0, s11: 0,}
  }
}

/// The trapframe is the region of memory
/// in every process' address space that stores
/// the data necessary to start the trap handling
/// in the kernel and go back to the process later
#[derive(Copy, Clone)]
#[repr(C)] // Ensure struct follows C memory layout
pub struct Trapframe {
  // Data for the kernel
  pub kernel_satp: usize, // Kernel page table
  pub kernel_sp: usize, // Top of trap stack for this process
  pub kernel_hartid: usize, // CPU ID
  pub epc: usize, // Next instruction to be executed coming back from the kernel
  
  // Process state
  pub ra: usize,  // Return address
  pub sp: usize,  // Stack pointer
  pub gp: usize,  // Global pointer
  pub tp: usize,  // Thead pointer
  
  pub t0: usize,
  pub t1: usize,
  pub t2: usize,
  
  pub s0: usize,
  pub s1: usize,
  
  pub a0: usize,
  pub a1: usize,
  pub a2: usize,
  pub a3: usize,
  pub a4: usize,
  pub a5: usize,
  pub a6: usize,
  pub a7: usize,
  
  pub s2: usize,
  pub s3: usize,
  pub s4: usize,
  pub s5: usize,
  pub s6: usize,
  pub s7: usize,
  pub s8: usize,
  pub s9: usize,
  pub s10: usize,
  pub s11: usize,
  
  pub t3: usize,
  pub t4: usize,
  pub t5: usize,
  pub t6: usize,
}
/*impl Trapframe {
  pub const fn new() -> Self {
    Self {
        kernel_satp: 0, kernel_sp: 0, kernel_hartid: 0, kernel_trap: 0, epc: 0,
        ra: 0, sp: 0, gp: 0, tp: 0,
        t0: 0, t1: 0, t2: 0,
        s0: 0, s1: 0,
        a0: 0, a1: 0, a2: 0, a3: 0,
        a4: 0, a5: 0, a6: 0, a7: 0,
        s2: 0, s3: 0, s4: 0, s5: 0,
        s6: 0, s7: 0, s8: 0, s9: 0,
        s10: 0, s11: 0,
        t3: 0, t4: 0, t5: 0, t6: 0,
    }
  }
}*/
