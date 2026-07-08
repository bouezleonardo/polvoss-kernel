// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.

use crate::mmio::monitor::*;
use crate::memory::frame_alloc::{init_frame_alloc, kmalloc, kfree};
use crate::memory::virtual_memory::{init_virtual_memory};
use crate::riscv::memory_types::{Addr};

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
  
  init_frame_alloc();
  init_virtual_memory();
  
  monitor_putc_at(b' ', 0, 0);
  monitor_putc_at(b'O', 0, 0);
  monitor_putc_at(b'K', 0, 0);
  
  let mut opt: Option<Addr> = kmalloc();
  while opt.is_some() {
    kfree(opt.unwrap());
    opt = kmalloc();
  }
  
  monitor_putc_at(b'A', 0, 0);
  
  loop{}
}
