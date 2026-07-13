//! Monitor driver.
//!
//! The monitor driver is responsible for
//! writing to the monitor's memory region, 
//! which is defined in [`crate::config::constants`]
//! module.

use crate::config::constants::{M_BASE,
                               M_WIDTH,
                               M_HEIGHT};
use super::uart::*;
use core::fmt;

/// Buffer type is a 2D `u8` array with
/// [`crate::config::constants::M_WIDTH`] columns
/// and [`crate::config::constants::M_HEIGHT`] lines
type Buffer = [[u8;M_WIDTH];M_HEIGHT];

struct Monitor {
  row: usize, // Current row
  col: usize, // Current column
  buffer: *mut Buffer, // Pointer to the mmio
}

unsafe impl Sync for Monitor {}

pub struct Writer;

static mut MONITOR: Monitor = Monitor {
  row: 0,
  col: 0,
  buffer: M_BASE as *mut Buffer,
};

/// Print a character in the (col, row) position in the monitor
/// # Arguments
///  - `chr`: character to print
///  - `row`: row (y position)
///  - `col`: col (x position)
pub fn putc_at(chr: u8, row: usize, col: usize) {
  let offset: usize = col + row * M_WIDTH;
  
  // Check if the postion is within BUFFER's memory region
  if offset < M_HEIGHT * M_WIDTH {
    // Unsafe because this is a static
    unsafe {
      // FIXME: uses uart temporarily
      uart_move_cursor(row, col);
      uart_putc(chr);
      uart_move_cursor(MONITOR.row, MONITOR.col);
      
      //(*MONITOR.buffer)[row][col] = chr;
    }
  } else {
    panic!("[monitor]: access out of bounds.");
  }
}

pub fn putc(chr: u8) {
  unsafe {
    if MONITOR.col < M_WIDTH {
      putc_at(chr, MONITOR.row, MONITOR.col);
      
      MONITOR.col += 1;
      if MONITOR.col == M_WIDTH && MONITOR.row < M_HEIGHT-1 {
        MONITOR.col = 0;
        MONITOR.row += 1;
      }
    }
  }
}

pub fn write_string(s: &str) {
  // Make sure the string is ASCII, panic if it is not
  assert!(s.is_ascii());
  
  // Check for control characters
  for byte in s.bytes() {
    match byte {
      b'\n' => line_feed(),
      b'\r' => carriage_return(),
      _ => putc(byte),
    }
  }
}

// FIXME: uses uart temporarily
pub fn line_feed() {
  unsafe {
    if MONITOR.row < M_HEIGHT-1 {
      MONITOR.row += 1;
      uart_putc(b'\n');
    }
  }
}

// FIXME: uses uart temporarily
pub fn carriage_return() {
  unsafe { 
    MONITOR.col = 0;
    uart_putc(b'\r');
  }
}

/// Clear monitor
pub fn clear_screen() {
  unsafe { 
    MONITOR.col = 0;
    MONITOR.row = 0;
  }
  for i in 0..M_HEIGHT {
    for j in 0..M_WIDTH {
      putc_at(b' ', i, j);  
    }
  }
}

// FIXME: uses uart temporarily
pub fn backspace() {
  unsafe {
    if MONITOR.col > 0 {
      MONITOR.col -= 1;
      uart_backspace();
    } else if MONITOR.row > 0 {
      MONITOR.row -= 1;
      uart_backspace();
    }
  }
}

// Implement the Write trait for formatted output
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    write_string(s);
    Ok(())
  }
}
