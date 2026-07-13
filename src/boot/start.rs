// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.

use crate::mmio::monitor::{clear_screen};
use crate::{print, println};
use crate::memory::frame_alloc::{init_frame_alloc};
use crate::memory::virtual_memory::{init_virtual_memory, 
                                    use_virtual_memory};

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  // FIXME: temporary use of uart
  clear_screen();
  
  let oi = 70;
  print!("Hello world! {}\n\r", oi);
  
  init_frame_alloc();
  init_virtual_memory();
  use_virtual_memory();
  
  loop{println!("OK");}
}
