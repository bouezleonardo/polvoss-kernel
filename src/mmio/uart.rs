//! UART driver.
//!
//! The UART driver is responsible for
//! writing to the UART0 memory region.

use core::fmt;

use crate::config::constants::{UART0};

/// Base UART0 register
const BASE: *mut u8 = UART0 as *mut u8;

/// Writer for formatted prints to the UART.
/// It is used only for ANSI control codes.
struct Writer;

/// Write an ASCII character to the UART
pub fn uart_putc(chr: u8) {
  unsafe { BASE.write(chr) }
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
