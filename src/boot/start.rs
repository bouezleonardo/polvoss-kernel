// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.


use crate::{print, println};
use crate::io::console::{clear, init_console};
use crate::proc::processing::{cpu_id};
use crate::trap::plic::{init_plic, plic_enable};
use crate::trap::trap_handlers::{install_kernelvec};
use crate::memory::frame_alloc::{init_frame_alloc};
use crate::riscv::supervisor_mode::{intr_on};
use crate::memory::virtual_memory::{init_virtual_memory, 
                                    use_virtual_memory};

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  if cpu_id() == 0 {
    clear();
    
    print!("Hello world!\n\rWe are in 2026");
      
    // Frame allocation
    init_frame_alloc();
    
    // Virtual memory
    init_virtual_memory(); 
    use_virtual_memory();
    
    // Traps
    install_kernelvec();
    
    // PLIC
    init_plic();
    plic_enable();
    
    // Console
    init_console();
    
    for i in 1..100 {
      print!("({} x {} = {})", i, i, i*i);
    }
    
    for i in 1..100 {    
      print!("\n\r({} x {} = {})", i, i, i*i);
    }
    
    intr_on();
  }
  plic_enable();
  
  loop{}
}
