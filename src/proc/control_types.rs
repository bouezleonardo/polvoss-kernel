//! Processing related types.
//!
//! This module contains data strutures necessary
//! for controlling processes, context switches and
//! cpu state.
//!

use super::spin::Mutex;
use crate::riscv::memory_types::{Addr, PageTable};

/// These registers represent a context of
/// execution and must be preserved to make 
/// a context switch. This is used to switch
/// from a kernel's context to the scheduler
/// and back. Read chapter 1.1. of
/// the RISCV ABI doc.
#[derive(Copy, Clone)]
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
pub struct Trapframe {
  // Data for the kernel
  pub kernel_satp: usize, // Kernel page table
  pub kernel_sp: usize, // Top of kernel stack for this process
  pub kernel_hartid: usize, // CPU ID
  pub kernel_trap: usize, // Address of user_trap()
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

/// Possible process states
#[derive(Copy, Clone)]
enum ProcState {
  UNUSED,  // Free PCB
  NEW,     // The process is being prepared to run
  READY,   // Process ready to run
  RUNNING, // Process has the CPU
  SLEEPING,// Process is sleeping
  ZOMBIE,  // A child terminated, but the parent did not wait()
}

/// Process Control Block
pub struct Pcb {
  pub state: ProcState, // Process state
  pub killed: bool,     // Process is killed
  pub exit_status: i32, // Exit status
  pub pid: usize,       // Process ID
  pub chan: Option<u64>, // Channel that the process is sleeping
  
  // Private fields that only one context accesses at a time
  pub kstack: u64,      // Address of the process kernel stack
  pub size: usize,      // Size of process memory in bytes
  pub pagetable: PageTable, // Process page table
  pub trapframe: Addr,  // Address of the process trapframe page
  pub ctx: Context,     // Kernel context for this process
  
  pub parent: Option<Addr>, // Parent PCB address
}
impl Pcb {
  pub const fn new() -> Self {
    Self {
      state: ProcState::UNUSED,
      killed: false,
      exit_status: 0,
      pid: 0,
      chan: None,
      kstack: 0,
      size: 0,
      pagetable: PageTable::new(Addr::new(0)),
      trapframe: Addr::new(0),
      ctx: Context::new(),
      parent: None,
    }
  }
}


/// CPU control struct. noff and intena
/// are used to enable/disable interrupts
/// when necessary to make it possible to
/// nest mutex lock()s
pub struct Cpu {
  pub proc: Option<Addr>, // PCB address of the current process running
  pub ctx: Context,       // Context to go back to the scheduler
  pub noff: usize,        // Depth of the push_off()s in sequence
  pub intena: bool,       // Interrupts were enable before push_off()
}
impl Cpu {
  pub const fn new() -> Self {
    Self {
      proc: None,
      ctx: Context::new(),
      noff: 0,
      intena: false,
    }
  }
}
