// boot/start.rs

//! Startup the kernel after hardware configuration.
//!
//! Startup the higher kernel funtionality. This includes
//! initializing the virtual memory, trap vector, init process 
//! and scheduling.

use crate::{print, println};
use crate::io::console::{clear, init_console};
use crate::proc::processing::{cpu_id, start_init_proc};
use crate::trap::plic::{init_plic, plic_enable};
use crate::trap::trap_handlers::{install_kernelvec};
use crate::memory::frame_alloc::{init_frame_alloc, used_memory_count};
use crate::riscv::supervisor_mode::{intr_on};
use crate::memory::virtual_memory::{init_virtual_memory, use_virtual_memory};
use crate::proc::scheduler::*;
use crate::config::constants::RAM_SIZE;

/// Startup the higher kernel funtionality
pub fn start() -> ! {
  if cpu_id() == 0 {
    clear();
    
    // Frame allocation
    print!("Initializing memory allocation...");
    init_frame_alloc();
    println!(" Done");
    
    // Virtual memory
    print!("Initializing virtual memory...");
    init_virtual_memory(); 
    use_virtual_memory();
    println!(" Done");
    
    // Traps
    print!("Installing kernel trap vector on CPU 0...");
    install_kernelvec();
    println!(" Done");
    
    // PLIC
    print!("Initializing PLIC...");
    init_plic();
    plic_enable();
    println!(" Done");
    
    // Console
    print!("Initializing the console...");
    init_console();
    println!(" Done");
    
    // Enable interrupts
    print!("Enabling interrupts on CPU 0...");
    intr_on();
    println!(" Done");
    
    print!("\nHello, POLVOSS!");
    print!("\nFree memory: {} kb", RAM_SIZE/1024);
    print!("\nKernel used memory: {} kb", used_memory_count()/1024);
    
    // Start init process
    start_init_proc();
  } else {
    plic_enable();
  }
  
  // Call scheduler
  scheduler();
}
