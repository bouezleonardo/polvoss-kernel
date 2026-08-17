// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.


use crate::{print, println};
use crate::io::console::{page_up, page_down, clear, 
                         backspace, init_console};
use crate::proc::processing::{cpu_id};
use crate::trap::plic::{init_plic, plic_enable};
use crate::trap::trap_handlers::{install_kernelvec, 
                                generate_interrupt};
use crate::memory::frame_alloc::{init_frame_alloc};
use crate::riscv::supervisor_mode::{intr_on, read_time, write_stimecmp, 
                                    read_sstatus, read_sie, read_stimecmp,
                                    read_sip};
use crate::memory::virtual_memory::{init_virtual_memory, 
                                    use_virtual_memory};
use crate::config::constants::{MILISECOND, TICK_TIME}; 

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
    
    intr_on();
    
    print!("\n\r sstatus: {:#b}\n\r sie: {:#b}\n\r sip: {:#b}", 
          read_sstatus(), read_sie(), read_sip());
    
    //generate_interrupt();
    
    /*
    for i in 1..100 {
      print!("\n\r({} x {} = {})", i, i, i*i);
    }
    
    for i in 1..3000 {
      for j in 0..10000{}
      backspace();
    }
    
    for i in 1..100 {
      print!("\n\r({} x {} = {})", i, i, i*i);
    }
    
    for i in 1..5000 {
      for j in 0..10000{}
      page_up();
    }
    
    for i in 1..3000 {
      for j in 0..100000{}
      backspace();
    }*/
  }
  plic_enable();
  
  loop{}
}
