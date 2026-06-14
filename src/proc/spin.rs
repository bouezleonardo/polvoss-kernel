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
  pub const fn new(value: T) -> Self {
    Self {
      locked: Cell::new(false),
      value: UnsafeCell::new(value),
    }
  }

  pub fn lock(&self) -> MutexGuard<'_, T> {
    while self.locked.get() {
      spin_loop();
    }

    self.locked.set(true);

    MutexGuard { mutex: self }
  }
}

impl<T> Drop for MutexGuard<'_, T> {
  fn drop(&mut self) {
    self.mutex.locked.set(false);
  }
}

impl<T> Deref for MutexGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe { &*self.mutex.value.get() }
  }
}

impl<T> DerefMut for MutexGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    unsafe { &mut *self.mutex.value.get() }
  }
}
