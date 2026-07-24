//! Print function and macros for the kernel.
//!
//! This module contains print functions and macros
//! for kernel output

use core::fmt;
use super::console::{write_fmt, putc};

/// Print function
pub fn _print(args: fmt::Arguments, new_line: bool) {
  write_fmt(args);
  
  if new_line {
    putc(b'\n');
    putc(b'\r');
  }
}

/// Print macro
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {{
    crate::mmio::print::_print(format_args!($($arg)*), false);
  }};
}

/// Println macro
#[macro_export]
macro_rules! println {
  ($($arg:tt)*) => {{
    crate::mmio::print::_print(format_args!($($arg)*), true);
  }};
}
