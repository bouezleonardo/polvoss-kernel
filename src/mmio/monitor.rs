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

/// Pointer to the memory region
const BUFFER: *mut Buffer = M_BASE as *mut Buffer;

/// Number of columns of the terminal.
pub const COLS: usize = M_WIDTH;

/// Number of lines of the terminal
pub const LINES: usize = M_HEIGHT*10;

pub struct Monitor {
  chars: [[u8; COLS]; LINES], // Circular buffer for characters 
  r_offset: usize, // Read offset.
  w_offset: usize, // Write offset. 
  scroll: bool,    // Enable scrolling
  row: usize,      // Cursor row on the screen
  col: usize,      // Cursor column on the screen
}

/// Print a character in the (col, row) position 
/// in the monitor
/// # Arguments
///  - `chr`: character to print
///  - `row`: row (y position)
///  - `col`: col (x position)
pub fn write_at(chr: u8, row: usize, col: usize) {
  let offset: usize = col + row * M_WIDTH;
  
  assert!(offset < M_HEIGHT * M_WIDTH, "[monitor]: access out of bounds.");
    
  // FIXME: uses uart temporarily
  uart_move_cursor(row, col);
  uart_putc(chr);
    
  // Unsafe because this is a raw pointer dereference
  /*unsafe {    
    (*BUFFER)[row][col] = chr;
  }*/
}

impl Monitor {
  /// Initialize a Monitor 
  pub const fn new() -> Self {
    Self {
      chars: [[b' ';COLS];LINES],
      r_offset: 0,  
      w_offset: 0, 
      scroll: true,
      row: 0,
      col: 0,    
    }
  }
  
  /// Print a character in the in the monitor
  /// # Arguments
  ///  - `chr`: character to print
  pub fn putc(&mut self, chr: u8) {
    // Check if it is possible to print
    if self.row >= M_HEIGHT {
      // If scroll is enabled
      if self.scroll {
        // Make the page view follow the line outputs
        if self.w_offset == self.r_offset {
          self.page_down();
        }
    
        // Scroll down the data
        self.scroll_down();
        
        // Reset line and
        self.row = M_HEIGHT-1;
        self.col = 0;
      }
    }
    // Update the chars buffer
    self.write_buffer(chr, self.row, self.col);
      
    // Write chr to the screen
    if self.r_offset == self.w_offset {
      write_at(chr, self.row, self.col);
    }
      
    // Go right a column
    self.col += 1;
    // Go down a line
    if self.col == M_WIDTH {
      self.col = 0;
      self.row += 1;
    }
  }
  
  /// Read from the chars buffer
  fn read_buffer(&self, i: usize, j: usize) -> u8 {
    // The lines are a circular buffer
    self.chars[(i + self.r_offset) % LINES][j]
  }
  
  /// Write to the chars buffer
  fn write_buffer(&mut self, chr: u8, i: usize, j: usize) {
    // The lines are a circular buffer
    self.chars[(i + self.w_offset) % LINES][j] = chr;
  }
  
  /// Clean a line of the chars buffer
  fn clean_line(&mut self, line: usize) {
    // The lines are a circular buffer
    self.chars[(line + self.w_offset) % LINES] = [b' ';COLS];
  }

  // FIXME: uses uart temporarily
  pub fn line_feed(&mut self) {
    if self.row < M_HEIGHT-1 {
      self.row += 1;
      uart_putc(b'\n');
    } else if self.scroll {
      if self.r_offset == self.w_offset {
        self.page_down();
      }
      self.scroll_down();
    }
  }

  // FIXME: uses uart temporarily
  pub fn carriage_return(&mut self) {
    self.col = 0;
    uart_putc(b'\r');
  }
  
  /// Write an ASCII string to the screen
  pub fn write_string(&mut self, s: &str) {
    // Make sure the string is ASCII, panic if it is not
    assert!(s.is_ascii(), "[monitor]: non-ASCII string write.");
    
    // Check for control characters
    for byte in s.bytes() {
      match byte {
        b'\n' => self.line_feed(),
        b'\r' => self.carriage_return(),
        _ => self.putc(byte),
      }
    }
  }
  
  /// Go down one line to read
  pub fn page_down(&mut self) {
    if self.r_offset <= self.w_offset {
      self.r_offset += 1;
      self.rewrite();
    }
  }
  
  /// Go up one line to read
  pub fn page_up(&mut self) {
    if self.r_offset <= 1 {
      self.r_offset = 0;
    } else {
      self.r_offset -= 1;
    }
    self.rewrite();
  }
  
  /// Scroll down the data to write more characters
  fn scroll_down(&mut self) {
    self.w_offset += 1;
    
    // Clean the next last line
    self.clean_line(M_HEIGHT);
  }
  
  /// Rewrite the buffer to the screen
  pub fn rewrite(&self){
    for i in 0..M_HEIGHT {
      for j in 0..M_WIDTH {
        write_at(self.read_buffer(i, j), i, j);
      }
    }
  }
  
  /// Clear monitor
  pub fn clear_screen(&mut self) {
    self.chars = [[b' ';COLS];LINES];
    self.col = 0;
    self.row = 0;
    self.r_offset = 0;
    self.w_offset = 0;
    
    // FIXME: temporary solution
    for i in 0..M_HEIGHT {
      for j in 0..M_WIDTH {
        write_at(b' ', i, j);
      }
    }
    
    // Clear memory region
    //unsafe { *BUFFER = [[b' ';M_WIDTH];M_HEIGHT] }
  }
  
  // FIXME: uses uart temporarily
  pub fn backspace(&mut self) {
    if self.col > 0 {
      self.col -= 1;
      uart_backspace();
    } else if self.row > 0 {
      self.row -= 1;
      uart_backspace();
    }
  }
}

// Implement the Write trait for formatted output
impl fmt::Write for Monitor {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}
