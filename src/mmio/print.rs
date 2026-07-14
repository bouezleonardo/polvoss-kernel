//! Output mechanisms.
//!
//! This module contains output mechanisms and macros
//! to control a Monitor instance, which enables writing
//! characters to the screen.

use core::fmt;
use crate::proc::spin::{Mutex, MutexGuard};
use super::monitor::{Monitor};

/// Instance of a Monitor to print to the screen
static MONITOR: Mutex<Monitor> = Mutex::new(Monitor::new());

/// Print function
pub fn _print(args: fmt::Arguments, new_line: bool) {
  use fmt::Write;
  //intr_off();
  
  // Lock the mutex for MONITOR
  let mut monitor: MutexGuard<Monitor> = MONITOR.lock();
  
  // Call write_fmt for formatted output
  monitor.write_fmt(args).unwrap();
  
  if new_line {
    monitor.line_feed();
    monitor.carriage_return();
  }
  
  //intr_off();
}

/// Clear the screen
pub fn _clear_screen() {
  //intr_off();
  MONITOR.lock().clear_screen();
  //intr_on();
}

/// Scroll up the screen contents
pub fn _page_up() {
  //intr_off();
  MONITOR.lock().page_up();
  //intr_on();
}

/// Scroll down the screen contents
pub fn _page_down() {
  //intr_off();
  MONITOR.lock().page_down();
  //intr_on();
}


/// Print macro
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {{
    crate::mmio::print::_print(format_args!($($arg)*), false);
  }};
}

/// Println macro
#[macro_export]
macro_rules! println {
  ($($arg:tt)*) => {{
    crate::mmio::print::_print(format_args!($($arg)*), true);
  }};
}

/// Println macro
#[macro_export]
macro_rules! clear_screen {
  ($($arg:tt)*) => {{
    crate::mmio::print::_clear_screen();
  }};
}
