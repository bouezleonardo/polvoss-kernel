// mmio/console.rs

//! Console for IO.
//!
//! When the kernel or a process reads or writes to
//! the console, it is responsible for reading the
//! keyboard input or writing the outuput to the
//! monitor.

use crate::config::constants::{M_BASE,
                               M_WIDTH,
                               M_HEIGHT};

use spin::Mutex;

struct Console {
  input_buf: [u8;M_HEIGHT],
  row: usize,
  column: usize,
  
}

static mut CONSOLE: 

///
pub fn console_init() {

}
