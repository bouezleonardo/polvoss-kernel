//! Clock mechanisms.
//!
//! This module includes clock mechanisms
//! such as the clock interrupt handler.

use crate::config::constants::{TICK_TIME, MILISECOND};
use crate::proc::processing::cpu_id;
use crate::proc::spin::*;
use crate::proc::sync::*;
use crate::riscv::supervisor_mode::{write_stimecmp,
                                    read_time};
use crate::print;

/// Syncronize processes waiting for ticks
pub static TICKS_CVAR: Condvar = Condvar::new();
/// Count the number of ticks
pub static TICKS: Mutex<u64> = Mutex::new(0);

/// Interrupt handler for clock
pub fn clock_intr() {
  // Only CPU 0 should increment ticks to avoid
  // incrementing extra times
  if cpu_id() == 0 {
    // Increment ticks
    let mut ticks: MutexGuard<u64> = TICKS.lock();
    *ticks += 1;
    
    // Wakes up all sleeping processes on this condvar
    TICKS_CVAR.notify_all();
  }
  // Write next time to have a clock interrupt
  write_stimecmp(read_time()+TICK_TIME*MILISECOND);
}
