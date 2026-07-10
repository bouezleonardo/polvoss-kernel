// mmio/monitor.rs

//! Monitor driver.
//!
//! The monitor driver is responsible for
//! writing to the monitor's memory region, 
//! which is defined in [`crate::config::constants`]
//! module.

use crate::config::constants::{M_BASE,
                               M_WIDTH,
                               M_HEIGHT};
use libm::{ceil, log10};

/// Buffer type is a 2D `u8` array with
/// [`crate::config::constants::M_WIDTH`] columns
/// and [`crate::config::constants::M_HEIGHT`] lines
type Buffer = [[u8;M_WIDTH];M_HEIGHT];

/// BUFFER is a raw pointer that points to the address
/// specified by [`crate::config::constants::M_BASE`]
const BUFFER: *mut Buffer = M_BASE as *mut Buffer;

/// Print a character in the (x, y) position in the monitor
/// # Arguments
///  - `chr`: character to print
///  - `x`: x position
///  - `y`: y position
pub fn monitor_putc_at(chr: u8, x: usize, y: usize) {
  let offset: usize = x + y * M_WIDTH;
  
  // Check if the postion is within BUFFER's memory region
  if offset < M_HEIGHT * M_WIDTH {
    // Unsafe because this is a raw pointer derreference
    unsafe { (*BUFFER)[y][x] = chr; }
  }
}

/// TODO: not ready
pub fn monitor_integer_at(mut num: i64, x: usize, y: usize) {
  let mut digit_count: usize = 1;
  let mut chars: [u8;20] = [48;20];
  
  // Account for sign
  if num < 0 {
    monitor_putc_at(b'-', x, y);
    num *= -1;
  } 
  
  // Number of digits in the number
  if num != 0 {
    digit_count = ceil(log10(num as f64)) as usize;
  }
  
  for i in 0..digit_count {
    chars[i] = (num % 10) as u8;
    num /= 10;
  }
  
  // TODO: fix the position here
  for i in 0..digit_count {
    monitor_putc_at(chars[digit_count-1-i]+48, x, y);
  }
}
