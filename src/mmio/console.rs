//! Console for IO.
//!
//! When a process reads or writes to
//! the console, it is responsible for reading the
//! keyboard input or writing the output to the
//! monitor.

use core::fmt;
use core::str::from_utf8;
use crate::proc::spin::{Mutex, MutexGuard};
use super::monitor::*;

/// Number of columns of the terminal.
const INPUT_BUF_SIZE: usize = 128;

/// Keyboard input struct
struct Input {
  chars: [u8; INPUT_BUF_SIZE],
}

/// enable/disable canonical mode
static CANONICAL: Mutex<bool> = Mutex::new(true);

/// Monitor struct to print to the screen
static MONITOR: Mutex<Monitor> = Mutex::new(Monitor::new(true));

pub fn init_console() {

}

/// Write a formatted string to the screen.
/// For use by the kernel only
pub fn write_fmt(args: fmt::Arguments) {
  use core::fmt::Write;
  MONITOR.lock().write_fmt(args).unwrap();
}
/// Write a string to the screen.
/// For use by the kernel only
pub fn write_string(s: &str) {
  MONITOR.lock().write_string(s);
}
/// Write a character to the screen.
/// For use by the kernel only
pub fn putc(chr: u8) {
  MONITOR.lock().putc(chr);
}
/// Clear screen.
/// For use by the kernel only
pub fn clear() {
  MONITOR.lock().clear();
}
/// Scroll up a line in the terminal.
/// For use by the kernel only
pub fn page_up() {
  MONITOR.lock().page_up();
}
/// Scroll down a line in the terminal.
/// For use by the kernel only
pub fn page_down() {
  MONITOR.lock().page_down();
}

/// Process ANSI escape codes before printing.
/// FIXME: this is not complete
/// # Arguments
/// - `buf`: character buffer
/// # Return
/// Maximum index where the buffer is ready to
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
    return buf.len();
  }
  
  // Check if the code is cut out
  if pos == buf.len()-1 || pos == buf.len()-2 {
    return pos-1;
  }
  
  // Check which sequence it is
  if buf[pos+1] == b'['{
    match buf[pos+2] {
      b'H' => clear(),
      b'T' => {
        // Switch mode (canonical/raw)
        let mut mode: MutexGuard<bool> = CANONICAL.lock();
        *mode != *mode;
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
  
  buf.len()
}

/*/// Userspace write() in the console comes here and the data
/// is written to the monitor to print to the screen
/// # Arguments
/// - `usr_src`: true if the source address is from a user process
/// - `addr`: source address
/// - `len`: length in bytes of the output
pub fn console_write(usr_src: bool, addr: u64, len: usize) {
   // Buffer to put the data while it is being transfered
   // from the memory
   let mut buf: [u8;32] = [0;32];
   // Counter
   let mut i: usize = 0;
   // Size of the next batch to be copied
   let mut copy_len: usize = buf.len();
   // String slice to be printed
   let mut s: &str;
   
   while i < len {
     // If the next bach copied
     if copy_len > len - i {
       copy_len = len - i;
     }
     // either_copyin copies data from either the kernel's
     // address space or from some user's space. Break if it
     // fails
     if !either_copyin(& mut buf, usr_src, addr+i as u64, copy_len) {
        break;
     }
     copy_len = buf.len();
     
     // Process escape codes.
     copy_len = process_ansi(&mut buf);
     
     // Get a string slice from the buffer
     s = from_utf8(&buf).expect("[console]: console write failed");
     
     // write string to the screen
     write_string(s);
     
     i += copy_len;
   }
}*/

/// Userspace read() in the console comes here
pub fn console_read() {

}

/// Userspace read() in the console 
pub fn console_intr(chr: u8) {

}
