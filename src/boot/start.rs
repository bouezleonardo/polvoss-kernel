// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.


use crate::{print, println, clear_screen};
use crate::memory::frame_alloc::{init_frame_alloc};
use crate::memory::virtual_memory::{init_virtual_memory, 
                                    use_virtual_memory};

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  clear_screen!();
  
  let year = 2026;
  print!("Hello world!\n\rWe are in {}", year);
  
  init_frame_alloc();
  init_virtual_memory();
  use_virtual_memory();
  
  for i in 1..1000 {
    //for i in 0..1000000{}
    print!("({} x {} = {})", i, i, i*i);
  }
  
  loop{}
}
