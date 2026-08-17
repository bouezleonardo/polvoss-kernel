//! Console for IO.
//!
//! When a process reads or writes to
//! the console, it is responsible for reading the
//! keyboard input or writing the output to the
//! monitor.

use core::fmt;
use core::str::from_utf8;
use crate::proc::spin::{Mutex, MutexGuard};
use crate::proc::processing::{either_copyin};
use crate::proc::sync::*;
use crate::riscv::memory_types::{Addr};
use super::monitor::*;
use super::uart::{uart_init};

/// Number of columns of the terminal.
const INPUT_BUF_SIZE: usize = 128;

/// Keyboard input struct
struct InputBuffer {
  chars: [u8; INPUT_BUF_SIZE],
  e_offset: usize, // Edit offset
  r_offset: usize, // Read offset
  w_offset: usize, // Write offset
}

/// enable/disable canonical mode
static CANONICAL: Mutex<bool> = Mutex::new(true);

/// Monitor struct to print to the screen
static MONITOR: Mutex<Monitor> = Mutex::new(Monitor::new(true));

/// Input condition variable for processes waiting to read
static INPUT_CVAR: Condvar = Condvar::new();

/// Input buffer
static INPUT: Mutex<InputBuffer> = Mutex::new(InputBuffer {
                                  chars: [0; INPUT_BUF_SIZE],
                                  e_offset: 0,
                                  r_offset: 0, 
                                  w_offset: 0, 
                                });
/// Initialize the console
pub fn console_init() {
  // Initialize the keyboard
  uart_init();
}

/// Write a formatted string to the screen.
pub fn write_fmt(args: fmt::Arguments) {
  use core::fmt::Write;
  MONITOR.lock().write_fmt(args).unwrap();
}
/// Write a string to the screen.
pub fn write_string(s: &str) {
  MONITOR.lock().write_string(s);
}
/// Write a character to the screen.
pub fn putc(chr: u8) {
  MONITOR.lock().putc(chr);
}
/// Clear screen.
pub fn clear() {
  MONITOR.lock().clear();
}
/// Backspace.
pub fn backspace() {
  MONITOR.lock().backspace();
}
/// Scroll up a line in the terminal.
pub fn page_up() {
  MONITOR.lock().page_up();
}
/// Scroll down a line in the terminal.
pub fn page_down() {
  MONITOR.lock().page_down();
}

/// Process ANSI escape codes before printing.
/// FIXME: this is not complete
/// # Arguments
/// - `buf`: character buffer
/// # Return
/// Number of positions to go back to avoid cutting codes
fn process_ansi(buf: &mut [u8]) -> usize {
  // Code's first position
  let mut pos: usize = 0;
  let mut found: bool = false;
  
  // Find ESC character
  for i in 0..buf.len() {
    if buf[i] == b'\x1B' {
      pos = i; // Save position
      found = true;
      break;
    }
  }
  // If there is no ESC
  if !found {
    return 0;
  }
  
  // Check if the code is cut out
  if pos == buf.len()-1 || pos == buf.len()-2 {
    return buf.len() - pos;
  }
  
  // Check which sequence it is
  if buf[pos+1] == b'['{
    match buf[pos+2] {
      b'H' => clear(),
      b'T' => {
        // Switch mode (canonical/raw)
        let mut mode: MutexGuard<bool> = CANONICAL.lock();
        *mode = !(*mode);
        let mut monitor: MutexGuard<Monitor> = MONITOR.lock();
        monitor.scroll(*mode);
      },
      _ => found = false,
    }
  } else {
    found = false;
  }
  
  if found {
    // Erase code
    buf[pos] = 0;
    buf[pos+1] = 0;
    buf[pos+2] = 0;
  }
  
  0
}

/// Userspace write() in the console comes here and the data
/// is written to the monitor to print to the screen
/// # Arguments
/// - `usr_src`: true if the source address is from a user process
/// - `src`: source address
/// - `len`: length in bytes of the output
pub fn console_write(usr_src: bool, src: Addr, len: usize) {
   // Buffer to put the data while it is being transfered
   // from the memory
   let mut buf: [u8;32] = [0;32];
   let mut i: usize = 0; // Counter
   let mut copy_len: usize = buf.len(); // Size of the next batch to be copied
   let mut cut: usize; // Avoid cutting ansii codes between two batches
   let mut s: &str; // String slice to be printed
   
   while i < len {
     // Amount of bytes to be copied is bigger than len
     if copy_len > len - i {
       copy_len = len - i;
     }
     // either_copyin copies data from either the kernel's
     // address space or from some user's space. Break if it
     // fails
     if !either_copyin(Addr::to_addr(&buf), usr_src, src.clone() + i, copy_len) {
        break;
     }
     copy_len = buf.len();
     
     // Process escape codes. There is a chance that a code
     // gets cut out, so it should not advance past it
     cut = process_ansi(&mut buf);
     
     // Get a string slice from the buffer
     s = from_utf8(&buf).expect("[console]: console write failed.");
     
     // write string to the screen
     write_string(s);
     
     i += copy_len - cut;
   }
}

/// Userspace read() in the console comes here
pub fn console_read() {

}

/// Read one byte from the input buffer.
/// This is used by the kernel only.
/// # Arguments
/// - `byte`: byte read
/// # Return
/// Number of bytes read
pub fn read_byte(byte: &mut u8) -> usize {
  let mut input: MutexGuard<InputBuffer> = INPUT.lock();
  
  // Check if the read offset is less than the edit
  if input.r_offset < input.w_offset {
    let i: usize = input.r_offset % INPUT_BUF_SIZE;   
    *byte = input.chars[i];
    input.r_offset += 1;
    
    return 1;
  }
  0
}
/// Read one line from the input buffer.
/// This is used by the kernel only.
/// # Arguments
/// - `buf`: buffer for the line
/// # Return
/// Number of bytes read
pub fn read_line(buf: &mut [u8]) -> usize {
  let mut byte: u8 = 0;
  let mut bytes_read: usize = 0;
  let mut i: usize = 0;
  
  bytes_read = read_byte(&mut byte);
  while byte != b'\n' && i < buf.len() {
    // Busy wait
    for j in 0..10000{}
    
    bytes_read += read_byte(&mut byte);
    
    buf[i] = byte;
    
    i += 1;
  }
  bytes_read
}

/// Get the CTRL + chr character
const fn ctrl(chr:u8) -> u8 {
  chr-b'@'
}

/// Page up
const CTRL_Q: u8 = ctrl(b'Q');
/// Page down
const CTRL_A: u8 = ctrl(b'A');
/// Kill line
const CTRL_U: u8 = ctrl(b'U');
/// Backspace
const CTRL_H: u8 = ctrl(b'H');

/// Treat input comming from the uart_intr
/// # Arguments
/// - `chr`: the character typed
pub fn console_intr(chr: u8) {
  let mut input: MutexGuard<InputBuffer> = INPUT.lock();
  
  // In Canonical mode the input is preprocessed
  if *(CANONICAL.lock()) {
    match chr {
      CTRL_Q => page_up(),
      CTRL_A => page_down(),
      CTRL_U => { // Kill line
        while input.e_offset > input.w_offset {
          backspace();
          input.e_offset -= 1;
        }
      },
      CTRL_H => { // Backspace
        if input.e_offset > input.w_offset {
          backspace();
          input.e_offset -= 1;
        }
      },
      _ => { // Character for the user
        // Check if there is space for the input
        if input.e_offset-input.r_offset < INPUT_BUF_SIZE && chr != 0 {
          input.e_offset += 1;
    
          // Echo to the user
          putc(chr);
          
          // Save character in the buffer
          let i: usize = input.e_offset % INPUT_BUF_SIZE;
          input.chars[i] = chr;
          
          // Check if the user finished typing
          if chr == b'\n' {
            input.w_offset = input.e_offset;
            
            // Wake up all processes waiting for input
            INPUT_CVAR.notify_all();
          }
        }
      },
    }
  } else {
    // Raw mode input
    if input.e_offset-input.r_offset < INPUT_BUF_SIZE { 
      input.e_offset += 1;       
      input.w_offset += 1;
      let i: usize = input.e_offset % INPUT_BUF_SIZE;   
      input.chars[i] = chr;
    }
  }
}
