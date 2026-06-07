// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality like the
//! init process and scheduling.

use crate::mmio::monitor::*;

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  monitor_putc_at(b'H', 0, 0);
  monitor_putc_at(b'e', 0, 0);
  monitor_putc_at(b'l', 0, 0);
  monitor_putc_at(b'l', 0, 0);
  monitor_putc_at(b'o', 0, 0);
  monitor_putc_at(b' ', 0, 0);
  monitor_putc_at(b'w', 0, 0);
  monitor_putc_at(b'o', 0, 0);
  monitor_putc_at(b'r', 0, 0);
  monitor_putc_at(b'l', 0, 0);
  monitor_putc_at(b'd', 0, 0);
  loop {}
}
