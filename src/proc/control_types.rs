//! Processing related types.
//!
//! This module contains data strutures necessary
//! for controlling processes and cpu state.

use super::spin::Mutex;
use crate::riscv::memory_types::{Addr, PageTable};
use crate::trap::trap_types::{Context, Trapframe};

/// Possible process states
#[derive(Copy, Clone, PartialEq)]
pub enum ProcState {
  Unused,  // Free PCB
  New,     // The process is being prepared to run
  Ready,   // Process ready to run
  Running, // Process has the CPU
  Waiting, // Process is waiting (not busy waiting)
  Zombie,  // A child terminated, but the parent did not wait()
}

/// Process Control Block (PCB)
pub struct Pcb {
  pub state: ProcState, // Process state
  pub killed: bool,     // Process is killed
  pub exit_status: i32, // Exit status
  pub pid: usize,       // Process ID
  
  // Private fields that only one context accesses at a time
  pub kstack: Addr,     // Address of the process kernel stack
  pub size: usize,      // Size of process memory in bytes
  pub pagetable: PageTable, // Process page table
  pub trapframe: Addr,  // Address of the process trapframe page
  pub ctx: Context,     // Kernel context for this process
  
  pub parent: Option<Addr>, // Parent PCB address
}
impl Pcb {
  // Initialize a default PCB
  pub const fn new() -> Self {
    Self {
      state: ProcState::Unused,
      killed: false,
      exit_status: 0,
      pid: 0,
      kstack: Addr::new(0),
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
  pub proc: Option<&'static Mutex<Pcb>>,  // PCB of the current process running
  pub ctx: Context,       // Context to go back to the scheduler
  pub noff: usize,        // Depth of the push_off()s in sequence
  pub intena: bool,       // Interrupts were enable before push_off()
}
impl Cpu {
  // Initialize a default CPU
  pub const fn new() -> Self {
    Self {
      proc: None,
      ctx: Context::new(),
      noff: 0,
      intena: false,
    }
  }
}
