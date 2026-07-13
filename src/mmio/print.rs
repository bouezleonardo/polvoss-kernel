use core::fmt;
use super::monitor::{Writer, line_feed, carriage_return};

/// Print function
pub fn _print(args: fmt::Arguments) {
  use fmt::Write;
  
  // Create a Writer and call write_fmt
  Writer{}.write_fmt(args).unwrap();
}

/// Print macro
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {{
    $crate::mmio::print::_print(format_args!($($arg)*));
  }};
}

/// Println macro
#[macro_export]
macro_rules! println {
  ($($arg:tt)*) => {{
    $crate::mmio::print::_print(format_args!($($arg)*));
    $crate::mmio::monitor::carriage_return();
    $crate::mmio::monitor::line_feed();
  }};
}
