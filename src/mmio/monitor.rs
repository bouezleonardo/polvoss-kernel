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

/// Buffer type is a 2D `u8` array with
/// [`crate::config::constants::M_WIDTH`] columns
/// and [`crate::config::constants::M_HEIGHT`] lines
type Buffer = [[u8;M_WIDTH];M_HEIGHT];

/// Monitor struct encapsulates the character buffer
/// and the current position in the monitor
struct Monitor {
  buffer: *mut Buffer, // Raw pointer
  row: usize,          // Current row
  column: usize,       // Current column
}

/// Monitor variable's buffer points to the address
/// specified by [`crate::config::constants::M_BASE`]
/// and starts in row = 0, column = 0
static mut MONITOR: Monitor = Monitor{
  buffer:  M_BASE as *mut Buffer, // Points to M_BASE
  row: 0,
  column: 0,
};

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
    // and MONITOR is an unprotected mutable static
    unsafe { (*MONITOR.buffer)[y][x] = chr; }
  }
}
