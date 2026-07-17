// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.


use crate::{print, println};
use crate::mmio::console::{page_up, page_down, clear};
use crate::memory::frame_alloc::{init_frame_alloc};
use crate::memory::virtual_memory::{init_virtual_memory, 
                                    use_virtual_memory};

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  clear();
  
  let year = 2026;
  print!("Hello world!\n\rWe are in {}", year);
  
  init_frame_alloc();
  init_virtual_memory();
  use_virtual_memory();
  
  page_down();
  
  for i in 1..100 {
    print!("\n\r({} x {} = {})", i, i, i*i);
  }
  
  for i in 1..100 {
    for j in 0..1000000{}
    page_up();
  }
  
  loop{}
}
