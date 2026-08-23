// proc/spin.rs

//! Simple spinlock (Not atomic).
//!
//! This is a simple spinlock implementation 
//! that is not atomic, thus should not be used
//! in a multi-CPU enviroment. Later when atomic
//! instructions are supported by hardware, this
//! should be replaced by the `spin` crate or 
//! updated to use atomic operations.

use core::{
    cell::{Cell, UnsafeCell},
    hint::spin_loop,
    ops::{Deref, DerefMut},
};

use crate::riscv::supervisor_mode::{intr_on, intr_off, 
                                    intr_enabled};
use super::processing::{set_cpu_lock_count, 
                        cpu_lock_count, cpu_intena, 
                        set_cpu_intena};
use core::hint::black_box;

/// Save state and disable interrupts before 
/// locking a mutex to avoid deadlocks 
pub fn lock_stack_push(){
  // Save the old state because interrupts need
  // to be disabled before accessing the cpu
  // structs
  let old_intr: bool = intr_enabled();
  
  // Disable interrupts
  intr_off();
  
  // Save the state of interrupts before locks
  if cpu_lock_count() == 0{
    set_cpu_intena(old_intr);
  }
  
  // Increment lock_count for nested locks
  set_cpu_lock_count(cpu_lock_count()+1);
}

/// Enable interrupts after unlocking a mutex 
/// if they were enabled before 
pub fn lock_stack_pop(){
  if cpu_lock_count() == 0{
    panic!("[spin]: CPU lock count is 0 before unlock.");
  }
  
  // Decrement lock_count for nested locks
  set_cpu_lock_count(cpu_lock_count()-1);
  
  // Check if interrupts need to be renabled
  if cpu_lock_count() == 0 && cpu_intena() {
    // Enable interrupts
    intr_on();
  }
}

/// Mutex struct
pub struct Mutex<T> {
  locked: Cell<bool>,
  value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}

pub struct MutexGuard<'a, T> {
  mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
  /// Create a new mutex
  pub const fn new(value: T) -> Self {
    Self {
      locked: Cell::new(false),
      value: UnsafeCell::new(value),
    }
  }
  
  /// Lock the Mutex
  pub fn lock(&self) -> MutexGuard<'_, T> {
    black_box({
      // Disable interrupts
      lock_stack_push();

      while self.locked.get() {
        if intr_enabled() {
          panic!("[spin]: interrupts enabled while holding lock.");
        }
        spin_loop();
      }
      
      self.locked.set(true);
    });
   
    MutexGuard { mutex: self }
  }
}

/// Define drop for the MutexGuard
impl<T> Drop for MutexGuard<'_, T> {
  /// Unlock the Mutex
  fn drop(&mut self) {
    if self.mutex.locked.get() {
      black_box({
        self.mutex.locked.set(false);
        lock_stack_pop();
      });
    }
  }
}

/// Define dereferrence for the MutexGuard
impl<T> Deref for MutexGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe { &*self.mutex.value.get() }
  }
}

/// Define mutable dereferrence for the MutexGuard
impl<T> DerefMut for MutexGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    unsafe { &mut *self.mutex.value.get() }
  }
}
