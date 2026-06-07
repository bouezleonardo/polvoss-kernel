// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality like the
//! init process and scheduling.

use crate::mmio::monitor::*;

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  putc(b'X');
  loop {}
}
