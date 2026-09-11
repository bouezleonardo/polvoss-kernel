//! Scheduler.
//!
//! The scheduler is only called directly by
//! the CPU contexts while booting in start().
//! To come back here, a process running on kernel
//! mode needs to switch() to the CPU context.

use crate::print;
use crate::riscv::context_switch::*;
use crate::trap::trap_types::Context;
use super::spin::*;
use super::control_types::*;
use super::processing::{cpu_context, set_current_proc};
use super::round_robin::*;

/// Scheduler loop
pub fn scheduler() -> ! {
  // MutexGuard of the locked process
  let mut guard: MutexGuard<Pcb>;
  // Process mutex
  let mut proc: &'static Mutex<Pcb>;
  // Option containing the guard and mutex
  let mut opt: Option<(MutexGuard<Pcb>, &'static Mutex<Pcb>)>;
  // Print message if there is no process to run
  let mut print_msg: bool = true;
  
  loop{
    // Get the next process to run on this CPU
    // May be `None` if there is no process to run
    opt = round_robin();
    
    if opt.is_some() {
      print_msg = true;
      
      // Get the mutex guard
      (guard, proc) = opt.unwrap();
      
      // Start process execution
      dispatch(guard, proc);
      
      // Return from the process
      set_current_proc(None);
    } else if print_msg {
      print!("\n\r[scheduler]: No process to run.\n\r");
      print_msg = false;
    }
  }
}

/// Update process state and context switch.
fn 
dispatch(mut guard: MutexGuard<Pcb>, proc: &'static Mutex<Pcb>) 
{
  // Update current process running on this CPU
  set_current_proc(Some(proc));
  
  // Update process state
  guard.state = ProcState::Running;
  
  // Process kernel context
  let proc_ctx: *const Context = 
    &guard.ctx as *const Context;
  
  // Unlock process mutex to avoid deadlock
  drop(guard);
  
  // Context switch: cpu -> proc
  switch(cpu_context(), proc_ctx);
}
