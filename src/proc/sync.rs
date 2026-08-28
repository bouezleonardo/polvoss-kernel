//! Syncronization mechanisms.
//!
//! This module contains syncronization
//! mechanisms for processes.

use super::spin::*;
use super::processing::{current_proc_unwrap, 
                        call_scheduler};
use super::control_types::{Pcb, ProcState};
use crate::config::constants::NUM_PROC;

/// Condition variable queue.
struct CondvarQueue {
  // Array of waiting processes
  procs: [Option<&'static Mutex<Pcb>>; NUM_PROC],
  // Number of waiting processes
  count: usize,
}

/// Condition variable.
/// Processes can sleep on a condition variable and
/// be woken when another process notifies it. When
/// it is woken, it can check a condition to continue
/// or wait again.
/// The queue stores references to the mutexes of
/// processes waiting on this condition variable.
pub struct Condvar {
  // The queue is protected by a mutex to allow for
  // shared Condvars
  queue: Mutex<CondvarQueue>,
}

/// Push a process to the queue
fn push(proc: Option<&'static Mutex<Pcb>>, 
queue: &mut MutexGuard<CondvarQueue>) {
  let count: usize = queue.count;
  
  if count < NUM_PROC {
    queue.procs[count] = proc;
    queue.count += 1;
  }
}
/// Pop a process from the queue
fn pop(queue: &mut MutexGuard<CondvarQueue>) {
  let count: usize = queue.count;
  
  if count > 0 {
    queue.procs[count-1] = None;
    queue.count -= 1;
  }
}
/// Show the next process in the queue
fn peek(queue: &MutexGuard<CondvarQueue>)
-> Option<&'static Mutex<Pcb>> {
  let count: usize = queue.count;
  if count == 0 {
    return None;
  }
  queue.procs[count-1]
}

impl Condvar {
  /// Initialize the condition variable
  pub const fn new() -> Self {
    Self {
      queue: Mutex::new(CondvarQueue{
        procs: [None; NUM_PROC],
        count: 0,
      }),
    }
  }
  /// Process waits until it is notified to check a 
  /// condition.
  /// # Arguments
  /// - `mutex`: mutex of the condition.
  /// - `guard`: mutex guard of the condition (that 
  ///   is checked outside the Condvar).
  /// # Return
  /// A new mutex guard of the condition.
  pub fn 
  wait<'a, T>(
    &self, 
    mutex: &'a Mutex<T>, 
    guard: MutexGuard<'a, T>
  ) -> MutexGuard<'a, T> {
    // Current process
    let proc_mutex: &'static Mutex<Pcb> = 
    current_proc_unwrap("cond");
    // Guard for the process
    let mut proc: MutexGuard<Pcb>;
    // Guard for the queue
    let mut queue: MutexGuard<CondvarQueue>;
    
    // Lock queue mutex
    queue = self.queue.lock();
    
    // Push the process to the queue
    push(Some(proc_mutex), &mut queue);
      
    // Lock the process mutex
    proc = proc_mutex.lock();
    
    // Change process state
    proc.state = ProcState::Waiting;
    
    // Drop the guards to allow notify
    drop(queue);
    drop(guard);
    
    // Call the scheduler
    // This drops the proc guard before scheduling
    call_scheduler(proc);
    
    // Reacquire the condition mutex before returning,
    // so the caller can safely check the condition.
    mutex.lock()
  }
  
  /// Process waits until it is notified to check a 
  /// condition on its own PCB. The only difference
  /// from wait() is that this uses the process's
  /// PCB mutex to syncronize.
  /// # Arguments
  /// - `mutex`: mutex of the process PCB.
  /// - `guard`: mutex guard of the PCB (that 
  ///   is checked outside the Condvar).
  /// # Return
  /// A new mutex guard of the PCB.
  pub fn 
  wait_self(
    &self, 
    mutex: &'static Mutex<Pcb>, 
    mut guard: MutexGuard<Pcb>
  ) -> MutexGuard<'_, Pcb> {
    // Guard for the queue
    let mut queue: MutexGuard<CondvarQueue>;
    
    // Lock queue mutex
    queue = self.queue.lock();
    
    // Push the process to the queue
    push(Some(mutex), &mut queue);
    
    // Change process state
    guard.state = ProcState::Waiting;
    
    // Drop the guards to allow notify
    drop(queue);
    
    // Call the scheduler
    // This drops the guard before scheduling
    call_scheduler(guard);
    
    // Reacquire the condition mutex before returning,
    // so the caller can safely check the condition.
    mutex.lock()
  }
  
  /// Notify all processes on the queue.
  /// Caller should hold the condition guard.
  pub fn notify_all(&self) {
    // Element of the queue
    let mut opt: Option<&'static Mutex<Pcb>>;
    // Guard for the process
    let mut proc: MutexGuard<Pcb>;
    // Guard for the queue
    let mut queue: MutexGuard<CondvarQueue>;
    
    // Lock queue mutex
    queue = self.queue.lock();
    
    // Get the next process on the queue
    opt = peek(&queue);
    while opt.is_some() {
      // Lock the process mutex
      proc = opt.unwrap().lock();
      
      // Change process state
      proc.state = ProcState::Ready;
      
      // Remove process from the queue
      pop(&mut queue);
      
      // Get the next process on the queue
      opt = peek(&queue);
    }
  }
}
