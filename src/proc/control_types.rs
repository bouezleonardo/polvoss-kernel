//! Processing related types.
//!
//! This module contains data strutures necessary
//! for controlling processes and cpu state.

use super::spin::Mutex;
use crate::riscv::memory_types::{Addr, PageTable, 
                                 free_addr_space};
use crate::trap::trap_types::{Context, Trapframe};
use crate::memory::frame_alloc::kfree;

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

/// Signal code to terminate a process
pub const SIGKILL: i32 = 9;

/// Process Control Block (PCB)
pub struct Pcb {
  pub state: ProcState, // Process state
  pub kill_signal: i32, // Signal received from kill()
  pub exit_status: i32, // Exit status
  pub pid: usize,       // Process ID
  
  // Fields that only one context accesses at a time
  kstack: Option<Addr>, // Address of the process kernel stack
  pub size: usize,          // Size of process memory in bytes
  pagetable: Option<PageTable>, // Process page table
  trapframe: Option<Addr>,  // Process trapframe page
  pub ctx: Context,         // Kernel context for this process
  
  pub parent: Option<&'static Mutex<Pcb>>, // Parent PCB address
}
impl Pcb {
  /// Initialize a default PCB
  pub const fn new() -> Self {
    Self {
      state: ProcState::Unused,
      kill_signal: i32::MAX,
      exit_status: i32::MAX,
      pid: usize::MAX,
      kstack: None,
      size: 0,
      pagetable: None,
      trapframe: None,
      ctx: Context::new(),
      parent: None,
    }
  }
  
  /// Init trapframe memory
  pub fn init_trapframe(&mut self, addr: Addr) {
    // Check if there is already a trapframe allocated
    if self.trapframe.is_some() {
      panic!("[PCB]: Trapframe already initialized.");
    }
    self.trapframe = Some(addr);
  }
  /// Free the trapframe page
  pub fn free_trapframe(&mut self) {
    // Check if there is a trapframe allocated
    if self.trapframe.is_none() {
      panic!("[PCB]: no Trapframe to free.");
    }      
    kfree(self.trapframe.clone().unwrap());
    self.trapframe = None;
  }
  /// Read the trapframe
  pub fn trapframe(&self) -> Trapframe {
    if self.trapframe.is_none() {
      panic!("[PCB]: no Trapframe to read.");
    }
    
    let tpf: Addr = self.trapframe.clone().unwrap();
    tpf.read::<Trapframe>()
  }
  /// Write the trapframe
  pub fn write_trapframe(&mut self, frame: Trapframe) {
    if self.trapframe.is_none() {
      panic!("[PCB]: no Trapframe to write.");
    }
    
    let tpf: Addr = self.trapframe.clone().unwrap();
    tpf.write::<Trapframe>(frame);
  }
  /// Get the trapframe address
  pub fn trapframe_addr(&self) -> Addr {
    if self.trapframe.is_none() {
      panic!("[PCB]: no Trapframe address.");
    }
    self.trapframe.clone().unwrap()
  }
  
  /// Init pagetable memory
  pub fn init_pagetable(&mut self, pgt: PageTable) {
    // Check if there is already a pagetable allocated
    if self.pagetable.is_some() {
      panic!("[PCB]: Pagetable already initialized.");
    }
    self.pagetable = Some(pgt);
  }
  /// Free the pagetable page
  pub fn free_pagetable(&mut self) {
    // Check if there is a pagetable allocated
    if self.pagetable.is_none() {
      panic!("[PCB]: no Pagetable to free.");
    } 
    free_addr_space(self.pagetable.clone().unwrap());
    self.size = 0;
    self.pagetable = None;
  }
  /// Get the pagetable
  pub fn pagetable(&self) -> PageTable {
    if self.pagetable.is_none() {
      panic!("[PCB]: no Pagetable to read.");
    }
    self.pagetable.clone().unwrap()
  }
  
  /// Init kstack memory
  pub fn init_kstack(&mut self, addr: Addr) {
    // Check if there is already a kstack allocated
    if self.kstack.is_some() {
      panic!("[PCB]: kstack already initialized.");
    }
    self.kstack = Some(addr);
  }
  /// Free the kstack page
  pub fn free_kstack(&mut self) {
    // Check if there is a kstack allocated
    if self.kstack.is_none() {
      panic!("[PCB]: no kstack to free.");
    } 
    kfree(self.kstack.clone().unwrap());
    self.kstack = None;
  }
  /// Get the kstack
  pub fn kstack(&self) -> Addr {
    if self.kstack.is_none() {
      panic!("[PCB]: no kstack to read.");
    }
    self.kstack.clone().unwrap()
  }
  
  /// Free the process allocated memory
  pub fn free_memory(&mut self) {
    if self.trapframe.is_some() {
      // Free Trapframe page
      self.free_trapframe();
    }
    
    if self.pagetable.is_some() {
      // Free pagetable pade
      self.free_pagetable();
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
