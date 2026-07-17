//! Console for IO.
//!
//! When a process reads or writes to
//! the console, it is responsible for reading the
//! keyboard input or writing the output to the
//! monitor.

use core::fmt;
use crate::proc::spin::{Mutex, MutexGuard};
use super::monitor::*;
//use super::keyboard::*;

/// Monitor struct to print to the screen
static MONITOR: Mutex<Monitor> = Mutex::new(Monitor::new(true));

pub fn init_console() {

}

/// Userspace read() in the console
pub fn console_write() {

}

pub fn console_read() {

}

/// Write a formatted string to the screen.
/// For use by the kernel only
pub fn write_fmt(args: fmt::Arguments) {
  use core::fmt::Write;
  MONITOR.lock().write_fmt(args).unwrap();
}

/// Write a character to the screen.
/// For use by the kernel only
pub fn putc(chr: u8) {
  MONITOR.lock().putc(chr);
}

pub fn clear() {
  MONITOR.lock().clear();
}

pub fn page_up() {
  MONITOR.lock().page_up();
}

pub fn page_down() {
  MONITOR.lock().page_down();
}
