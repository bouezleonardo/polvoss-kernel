// mmio/console.rs

//! Console for IO.
//!
//! When a process reads or writes to
//! the console, it is responsible for reading the
//! keyboard input or writing the output to the
//! monitor.

use spin::Mutex;

struct Console {
  Monitor mon,
  Keyboard kyb,
  cooked: bool,    // Enable/Disable cooked mode
}

///
pub fn console_init() {

}
