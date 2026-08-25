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
  pub kill_signal: i32, // Signal received from kill()
  pub exit_status: i32, // Exit status
  pub pid: usize,       // Process ID
  
  // Private fields that only one context accesses at a time
  pub kstack: Addr,     // Address of the process kernel stack
  pub size: usize,      // Size of process memory in bytes
  pub pagetable: PageTable, // Process page table
  trapframe: Addr,  // Process trapframe page
  pub ctx: Context,     // Kernel context for this process
  
  pub parent: Option<&'static Mutex<Pcb>>, // Parent PCB address
}
impl Pcb {
  // Initialize a default PCB
  pub const fn new() -> Self {
    Self {
      state: ProcState::Unused,
      kill_signal: 0,
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
  // Get the trapframe
  pub fn trapframe(&self) -> Trapframe {
    self.trapframe.read::<Trapframe>()
  }
  // Update the trapframe
  pub fn update_trapframe(&mut self, tpf: Trapframe) {
    self.trapframe.write::<Trapframe>(tpf);
  }
}

/// CPU control struct. noff and intena
/// are used to enable/disable interrupts
/// when necessary to make it possible to
/// nest mutex lock()s
pub struct Cpu {
  pub proc: Option<&'static Mutex<Pcb>>,  // PCB of the current process running
  pub ctx: Context,       // Context to go back to the scheduler
  pub lock_count: usize,  // Amount of mutex locks in sequence
  pub intena: bool,     // Interrupts were enable before locking a mutex
}
impl Cpu {
  // Initialize a default CPU
  pub const fn new() -> Self {
    Self {
      proc: None,
      ctx: Context::new(),
      lock_count: 0,
      intena: false,
    }
  }
}
