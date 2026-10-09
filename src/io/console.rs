//! Console for IO.
//!
//! When a process reads or writes to
//! the console, it is responsible for reading the
//! keyboard input or writing the output to the
//! monitor.

use core::fmt;
use core::str::from_utf8;
use crate::proc::spin::{Mutex, MutexGuard};
use crate::proc::processing::{either_copyin, print_proc_stat,
                              either_copyout};
use crate::proc::sync::*;
use crate::riscv::memory_types::{Addr};
use super::monitor::*;
use super::uart::{init_uart};

/// Number of columns of the terminal.
const INPUT_BUF_SIZE: usize = 128;

/// Keyboard input struct
struct InputBuffer {
  chars: [u8; INPUT_BUF_SIZE],
  e_offset: usize, // Edit offset
  r_offset: usize, // Read offset
  w_offset: usize, // Write offset
}

/// Enable/disable canonical mode. When canonical mode
/// is disabled (raw mode), this will happen:
/// 1. The user inputs are not pre-processed by the kernel
/// 2. The processes have total control over the terminal
/// 3. Reads to the console are NON-BLOCKING
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
pub fn init_console() {
  // Initialize the keyboard
  init_uart();
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
pub fn backspace() -> usize {
  MONITOR.lock().backspace()
}
/// Scroll up a line in the terminal.
pub fn page_up() {
  MONITOR.lock().page_up();
}
/// Scroll down a line in the terminal.
pub fn page_down() {
  MONITOR.lock().page_down();
}
/// Scroll to the page where input is being written.
pub fn page_follow() {
  MONITOR.lock().page_follow();
}
/// Move cursor to the specified row and column.
pub fn move_cursor(row: usize, col: usize) {
  MONITOR.lock().move_cursor(row, col);
}
/// Get the row and column of the cursor.
pub fn find_cursor() -> (usize, usize) {
  MONITOR.lock().find_cursor()
}

/// Process ANSI escape codes before printing.
/// # Arguments
/// - `buf`: character buffer
/// # Return
/// Number of positions to go back to avoid cutting codes
fn process_ansi(buf: &mut [u8]) -> usize {    
  // Find ESC character
  for i in 0..buf.len() {
    if buf[i] == b'\x1B' {
      // Check if the code is cut out
      // Maximum size for a code is 10 chars
      if i > buf.len()-10 {
        return buf.len() - i;
      }
      
      if buf[i+1] != b'['{
        return 0;
      }
      
      // Amount of bytes to remove from buf
      // after processing
      let mut remove: usize = 0;
      
      // Check which sequence it is
      match buf[i+2] {
        b'H' => { // Cursor to home (0,0) 
          move_cursor(0, 0);
          remove = 3; 
        }, 
        b'N' => { // Canonical mode  
          *(CANONICAL.lock()) = true;
          MONITOR.lock().scroll(true);
          remove = 3;
        },
        b'R' => { // Raw mode
          *(CANONICAL.lock()) = false;
          MONITOR.lock().scroll(false);
          remove = 3;
        },
        _ => { // It begins with a number
          // Clear screen 2J
          if buf[i+2] == b'2' && 
            buf[i+3] == b'J' {
            clear();
            remove = 4;
          } else {
            // Move cursor {row};{col}H
            let mut off: usize = i+2;
            
            // Maximum number size
            let mut numsz: usize = 3;
            
            // {row} number
            let mut row: usize = 0;
            while numsz > 0 && 
                b'0' <= buf[off] && buf[off] <= b'9' {
              row = row*10 + (buf[off]-b'0') as usize;
              off += 1;
              numsz -= 1;
            }
            
            // Check if there was a number and
            // the separator
            if numsz == 3 || buf[off] != b';' {
              return 0;
            }
            
            // Remove \x1B[{row};
            remove = 2 + (3-numsz) + 1;
            
            off += 1;
            numsz = 3;
            
            // {col} number
            let mut col: usize = 0;
            while numsz > 0 &&
                b'0' <= buf[off] && buf[off] <= b'9' {
              col = col*10 + (buf[off]-b'0') as usize;
              off += 1;
              numsz -= 1;
            }
            
            // Check if there was a number and
            // the final character
            if numsz == 3 || buf[off] != b'H' {
              return 0;
            }
            
            // Remove {col}H
            remove += (3-numsz) + 1;

            move_cursor(row, col);
          }
        },
      }
      // Remove code from output
      for j in 0..remove {
        buf[i+j] = 0;
      }
      break;
    }
  }
  0
}

/// Userspace write() in the console comes here and the data
/// is written to the monitor to print to the screen
/// # Arguments
/// - `usr_src`: true if the source address is from a user process
/// - `src`: source address
/// - `len`: length in bytes of the output
/// # Return
/// Number of bytes written
pub fn 
console_write(usr_src: bool, src: Addr, len: usize) 
-> usize {
   // Buffer to put the data while it is being transfered
   // from the memory
   let mut buf: [u8;32] = [0;32];
   let mut copy_len: usize = buf.len(); // Size of the next batch to be copied
   let mut cut: usize; // Avoid cutting ansii codes between two batches
   let mut s: &str; // String slice to be printed
   let mut i: usize = 0; // Counter
   
   while i < len {
     // Amount of bytes to be copied is bigger than len
     if copy_len > len - i {
       copy_len = len - i;
     }
     // either_copyin copies data from either the kernel's
     // address space or from some user's space. Break if it
     // fails
     if !either_copyin(Addr::to_addr(&buf), usr_src, src.clone() + i, copy_len) {
        return i; // Amount of bytes written to this point
     }
     copy_len = buf.len();
     
     // Process escape codes. There is a chance that a code
     // gets cut out, so it should not advance past it
     cut = process_ansi(&mut buf);
     
     // Get a string slice from the buffer
     s = from_utf8(&buf).expect("[console]: console write failed.");
        
     // write string to the screen
     write_string(s);
     
     // Clear buffer
     buf = [0;32];
     
     i += copy_len - cut;
   }
   
   // Amount of bytes written
   len
}

/// Userspace read() in the console comes here
/// and the data is read from the input buffer
/// # Arguments
/// - `usr_dst`: true if the destination is in userspace
/// - `dst`: destination address
/// - `len`: length in bytes
/// # Return
/// Number of bytes read
pub fn 
console_read(usr_dst: bool, mut dst: Addr, len: usize) 
-> usize {
  // Buffer of the output
  let mut buf: [u8;1] = [0;1];
  // Number of bytes read
  let mut bytes: usize = 0;
  let mut i: usize = 0;
  
  // Get the mode (canonical/raw)
  let canon: bool = *(CANONICAL.lock());

  // Get the input buffer
  let mut input: MutexGuard<InputBuffer> = INPUT.lock();
  
  // Raw mode
  if !canon {
    while bytes < len && input.r_offset < input.w_offset {  
      input.r_offset += 1;
      let i: usize = input.r_offset % INPUT_BUF_SIZE;
      buf[0] = input.chars[i];
      
      // Copy from kernel to either user or kernel
      if !either_copyout(dst.clone(), 
                         usr_dst, 
                         Addr::to_addr(&buf), 
                         1) {
        break;
      }
      bytes += 1;
      dst += 1;
    }
  } else if canon {
    let mut chr: u8 = b' ';
    
    // Read all bytes until the end of line
    while bytes < len { 
      // Wait until there is input to read
      while input.r_offset >= input.w_offset {
        input = INPUT_CVAR.wait(&INPUT, input);
      }
      
      input.r_offset += 1;
      let i: usize = input.r_offset % INPUT_BUF_SIZE;
      chr = input.chars[i];
      buf[0] = chr;
      
      if chr == b'\n' || chr == ctrl(b'D') || 
          chr == ctrl(b'M') {
        break;
      }
      
      // Copy from kernel to either user or kernel
      if !either_copyout(dst.clone(), 
                         usr_dst, 
                         Addr::to_addr(&buf), 
                         1) {
        break;
      }
      bytes += 1;
      dst += 1;
    }
  }
  
  // Null terminator
  either_copyout(dst.clone(), 
                 usr_dst, 
                 Addr::to_addr(&[0;1]), 
                 1);
  bytes
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
/// Print process statistics
const CTRL_P: u8 = ctrl(b'P');

/// Treat input comming from the uart_intr
/// # Arguments
/// - `chr`: the character typed
pub fn console_intr(chr: u8) {
  let mut input: MutexGuard<InputBuffer> = INPUT.lock();
  
  // In Canonical mode the input is preprocessed
  if *(CANONICAL.lock()) {
    // Show page where the input is being typed
    let mut show_page: bool = true;
    
    match chr {
      CTRL_Q => {
        page_up();
        show_page = false;
      },
      CTRL_A => {
        page_down();
        show_page = false;
      },
      CTRL_U => { // Kill line
        while input.e_offset > input.w_offset {
          input.e_offset -= backspace();
        }
      },
      CTRL_P => { // Print process stats
        print_proc_stat();
      },
      b'\x7F' => { // Delete
        if input.e_offset > input.w_offset {
          input.e_offset -= backspace();
        }
      },
      _ => { // Character for the user
        // Check if there is space for the input
        if input.e_offset-input.r_offset < INPUT_BUF_SIZE && chr != 0 {
          input.e_offset += 1;
          
          // Save character in the buffer
          let i: usize = input.e_offset % INPUT_BUF_SIZE;
          input.chars[i] = chr;
          
          // Check if the user finished typing
          if chr == b'\n' || chr == ctrl(b'D') || chr == ctrl(b'M') {
            input.w_offset = input.e_offset;
            
            // Wake up all processes waiting for input
            INPUT_CVAR.notify_all();
          } else {
            // Echo to the user
            putc(chr);
          }
        }
      },
    }
    
    // Show page where the input is being typed
    if show_page {
      page_follow();
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
