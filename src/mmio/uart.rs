//! UART driver.
//!
//! The UART driver is responsible for
//! writing to the UART0 memory region.

use core::fmt;

use crate::config::constants::{UART0};
use super::console::console_intr;

// Receive register
const RHR: u8 = 0b000;
// Interrupt enable register
const IER: u8 = 0b001;
// Interrupt status register
const ISR: u8 = 0b010;
// Line status register 
const LSR: u8 = 0b101;

// Data ready to be read code
const LSR_RX_READY: u8 = 1;


fn read_register(reg: u8) -> u8 {
  let addr: u64 = UART0 + reg as u64;
  unsafe { return (addr as *const u8).read() }
}

/// Read a character from the receive FIFO (keyboard)
/// # Return
/// An option with the character read
fn uart_getc() -> Option<u8> {
  let mut opt: Option<u8> = None;
  
  // Check the first bit of LSR to see if there are
  // bytes to be read
  if read_register(LSR) & LSR_RX_READY == 1 {
    opt = Some(read_register(RHR));
  }
  opt
}

/// UART interrupt handler. 
/// dev_intr function calls this handler after identifying
/// the interrupt is from UART
pub fn uart_intr() {
  let mut opt: Option<u8>;
  
  // Acknowledge interrupt
  read_register(ISR);
  
  // Read characters typed in the keyboard
  opt = uart_getc();
  while opt.is_some() {
    console_intr(opt.unwrap());
    opt = uart_getc();
  }
}

/********************|TEMPORARY|**********************/

/// Writer for formatted prints to the UART.
/// It is used only for ANSI control codes.
struct Writer;

/// Write an ASCII character to the UART
pub fn uart_putc(chr: u8) {
  unsafe { (UART0 as *mut u8).write(chr) }
}

/// Write an ASCII string to the UART
fn uart_write_string(s: &str) {
  for byte in s.bytes() {
    uart_putc(byte);
  }
}

// This is for use inside the uart module
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    uart_write_string(s);
    Ok(())
  }
}
/// Formatted print
fn uart_print(args: fmt::Arguments){
  use fmt::Write;
  Writer{}.write_fmt(args).unwrap();
}

/// Macro for for formatted print
#[macro_export]
macro_rules! uart_print {
  ($($arg:tt)*) => {{
    $crate::mmio::uart::uart_print(format_args!($($arg)*));
  }};
}

/// Move cursor to the row and col
pub fn uart_move_cursor(row: usize, col: usize) {
  uart_print!("\x1B[{};{}H", row+1, col+1);
}

/// Erase a character
pub fn uart_backspace() {
  uart_print!("\x08 \x08");
}

// Clear screen
pub fn uart_clear() {
  uart_print!("\x1B[2J\x1B[H");
}
