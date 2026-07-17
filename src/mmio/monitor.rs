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

/// The Monitor stores a circular buffer for the lines
/// of text and read and write offsets that allow the
/// lines of text in the screen to be scrolled
pub struct Monitor {
  chars: [[u8; COLS]; LINES], // Circular buffer for the lines 
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
  if row < M_HEIGHT && col < M_WIDTH {  
    // FIXME: uses uart temporarily
    uart_move_cursor(row, col);
    uart_putc(chr);
    
    // Unsafe because this is a raw pointer dereference
    /*
    unsafe { (*BUFFER)[row][col] = chr; }
    */
  }
}

impl Monitor {
  /// Initialize a Monitor 
  pub const fn new(scr: bool) -> Self {
    Self {
      chars: [[b' ';COLS];LINES],
      r_offset: 0,  
      w_offset: 0, 
      scroll: scr,
      row: 0,
      col: 0,    
    }
  }
  
  // Change scroll mode
  pub fn scroll(&mut self, scr:bool) {
    self.scroll = scr;
  }
  
  /// Read from the chars buffer
  fn read_buffer(&self, i: usize, j: usize, offset: usize) -> u8 {
    // The lines are a circular buffer
    if i < LINES && j < COLS {
      return self.chars[(i + offset) % LINES][j];
    }
    return 0;
  }
  
  /// Write to the chars buffer
  fn write_buffer(&mut self, chr: u8, i: usize, j: usize, offset: usize) {
    // The lines are a circular buffer
    if i < LINES && j < COLS { 
      self.chars[(i + offset) % LINES][j] = chr;
    }
  }
  
  /// Clean a line of the chars buffer
  fn clean_line(&mut self, line: usize) {
    // Clean the buffer
    self.chars[(line + self.w_offset) % LINES] = [0;COLS];
    
    // Clean the screen
    for j in 0..M_WIDTH {
      write_at(b' ', line, j);
    }
    
    /*unsafe { (*BUFFER)[line] = [b' ', COLS]; }*/
  }
  
  // FIXME: uses uart temporarily
  pub fn move_cursor(&self, row: usize, col: usize) {
    uart_move_cursor(self.row, self.col);
  }
  
  fn line_feed(&mut self) {
    self.row += 1;

    if self.row > M_HEIGHT-1 && self.scroll {
      // Check if the data printed to the terminal
      // should scroll with the outputs.
      if self.r_offset == self.w_offset {
        // Go foward in the buffer
        self.r_offset += 1;
        // Refresh screen comparing with the previous offset
        self.refresh(self.r_offset-1);
      }
      self.scroll_down();
      self.row = M_HEIGHT-1;
    }
  }
  
  fn backspace(&mut self) {
    self.col -= 1;
  }
  
  fn carriage_return(&mut self) {
    self.col = 0;
  }
  
  fn tab(&mut self) {
    // Set col position to the next multiple 8
    self.col = (self.col + 7) / 8 * 8;
  }
  
  /// Print a character in the in the monitor
  /// # Arguments
  ///  - `chr`: character to print
  pub fn putc(&mut self, chr: u8) {
    match chr {
      b'\n' => self.line_feed(),
      b'\r' => self.carriage_return(),
      b'\t' => self.tab(),
      _ => {
        // Update the chars buffer
        if self.r_offset == self.w_offset || self.scroll { 
          self.write_buffer(chr, self.row, self.col, self.w_offset);
        }
        
        // Write chr to the screen if it is on view
        if self.r_offset == self.w_offset {
          write_at(chr, self.row, self.col);
        }
        
        // Go right a column
        self.col += 1;
        // Go down a line
        if self.col >= M_WIDTH && self.scroll {
          self.line_feed();
          self.carriage_return();
        }
      },
    }
    
    if self.r_offset == self.w_offset {
      self.move_cursor(self.row, self.col);
    }
  }
  
  /// Write an ASCII string to the screen
  pub fn write_string(&mut self, s: &str) {
    // Make sure the string is ASCII, panic if it is not
    assert!(s.is_ascii(), "[monitor]: non-ASCII string write.");
    
    // Put the characters in the screen
    for byte in s.bytes() {
      self.putc(byte);
    }
  }
  
  /// Go down one line to read
  pub fn page_down(&mut self) {
    if self.r_offset < self.w_offset {
      // Go foward in the buffer
      self.r_offset += 1;
      // Refresh screen comparing with the previous offset
      self.refresh(self.r_offset-1);
    }
  }
  
  /// Go up one line to read
  pub fn page_up(&mut self) {
    if self.r_offset > 0 {
      // Go backwards in the buffer
      self.r_offset -= 1;
      // Refresh screen comparing with the previous offset
      self.refresh(self.r_offset+1);
    }
  }
  
  /// Scroll down the data to write more characters
  fn scroll_down(&mut self) {
    self.w_offset += 1;
    // Clean the next last line
    self.clean_line(M_HEIGHT-1);
  }
  
  /// Rewrite the buffer to the screen based on r_offset
  fn refresh(&self, old_offset: usize){   
    // old and new chr are used to avoid rewritting the same
    // character again
    let mut old_chr: u8 = 0;
    let mut new_chr: u8 = 0;
    
    for i in 0..M_HEIGHT {
      for j in 0..M_WIDTH {
        new_chr = self.read_buffer(i, j, self.r_offset);
                      
        if self.r_offset > 0 {
          old_chr = self.read_buffer(i, j, old_offset);
        }
        
        if new_chr != old_chr {
          write_at(new_chr, i, j);
        }
      }
    }
  }
  
  /// Clear monitor
  pub fn clear(&mut self) {
    self.chars = [[b' ';COLS];LINES];
    self.col = 0;
    self.row = 0;
    self.r_offset = 0;
    self.w_offset = 0;
    self.move_cursor(self.row, self.col);
    
    // FIXME: temporary solution
    for i in 0..M_HEIGHT {
      for j in 0..M_WIDTH {
        write_at(b' ', i, j);
      }
    }
    
    // Clear memory region
    //unsafe { *BUFFER = [[b' ';M_WIDTH];M_HEIGHT] }
  }
}

// Implement the Write trait for formatted output
impl fmt::Write for Monitor {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}
