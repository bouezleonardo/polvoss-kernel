//! Syncronization mechanisms.
//!
//! This module contains syncronization
//! mechanisms for processes.

use super::spin::*;
use super::processing::{PCB, current_proc, call_scheduler};
use super::control_types::{Pcb, ProcState};
use crate::config::constants::NUM_PROC;

/// Condition variable.
/// Processes can sleep on a condition variable and
/// be woken when another process notifies it. When
/// it is woken, it can check a condition to continue
/// or wait again.
/// The queue stores references to the mutexes of
/// processes waiting on this condition variable.
pub struct Condvar(u64);

impl Condvar {
  /// Initialize the condition variable
  pub const fn new(chan: u64) -> Self {
    // Set the channel
    Self(chan)
  }
  
  /// Process waits until it is notified to check a condition.
  /// # Arguments
  /// - `mutex`: mutex of the condition.
  /// - `guard`: mutex guard of the condition.
  /// # Return
  /// A new mutex guard of the condition.
  pub fn 
  wait<'a, T>(&self, mutex: &'a Mutex<T>, guard: MutexGuard<'a, T>) 
  -> MutexGuard<'a, T> {
    let opt: Option<&'static Mutex<Pcb>>;
    let mut proc: MutexGuard<Pcb>;
    
    // Get the current process
    opt = current_proc();
    
    if opt.is_none() {
      panic!("[sleep]: no process running.");
    }
   
    // Lock the process mutex
    proc = opt.unwrap().lock();
    
    // Change process state
    proc.state = ProcState::Waiting;
    
    // Set the process channel
    proc.chan = Some(self.0);
    
    // Drop the guard to allow notifies
    drop(guard);
    
    // Call the scheduler
    // This drops the proc guard before scheduling
    call_scheduler(proc);
    
    // Reacquire the condition mutex before returning,
    // so the caller can safely check the condition.
    mutex.lock()
  }
  
  /// Notify all process on the channel.
  /// Caller should hold the condition guard.
  pub fn notify(&self) {    
    let mut proc: MutexGuard<Pcb>;
    let cur: &'static Mutex<Pcb>;
    
    // Get the current running process
    cur = current_proc().expect("[cond]: no current process.");
    
    // Change process state
    for i in 0..NUM_PROC {      
      // Avoid locking own mutex to not risk a deadlock
      if !core::ptr::eq(&PCB[i], cur) { 
        // Lock the process mutex
        proc = PCB[i].lock();
      
        if Some(self.0) == proc.chan 
           && proc.state == ProcState::Waiting {
          proc.state = ProcState::Ready;
          proc.chan = None;
        }
      }
    }
  }
}
