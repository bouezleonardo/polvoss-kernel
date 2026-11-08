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
    
    // Start init process
    print!("Starting init process...");
    start_init_proc();
    println!(" Done");
    
    // Welcome message
    boot_msg();
  } else {
    plic_enable();
  }
  
  // Call scheduler
  scheduler();
}

fn boot_msg() {
  clear();
  
  print!(":::::::::   ::::::::  :::    :::     :::  ::::::::   ::::::::   ::::::::  \n");
  print!(":+:    :+: :+:    :+: :+:    :+:     :+: :+:    :+: :+:    :+: :+:    :+: \n");
  print!("+:+    +:+ +:+    +:+ +:+    +:+     +:+ +:+    +:+ +:+        +:+        \n");
  print!("+#++:++#+  +#+    +:+ +#+    +#+     +:+ +#+    +:+ +#++:++#++ +#++:++#++ \n");
  print!("+#+        +#+    +#+ +#+     +#+   +#+  +#+    +#+        +#+        +#+ \n");
  print!("#+#        #+#    #+# #+#      #+#+#+#   #+#    #+# #+#    #+# #+#    #+# \n");
  print!("###         ########  ########## ###      ########   ########   ######## \n");
  print!("\n");
  print!("=========================================================================\n");
  print!("            Privilegeless Open Learnable OS Simulator - v1.0.0\n");
  print!("=========================================================================\n");
  
  print!("\n+=================|SYSTEM MEMORY|==================+");
  print!("\n");
  print!("|  Total memory:        {} KB", RAM_SIZE/1024);
  print!("\n");
  print!("|  Kernel memory used:  {} KB", used_memory_count()/1024);
  print!("\n");
  print!("|  Free memory:         {} KB", (RAM_SIZE-used_memory_count())/1024);
  print!("\n");
  print!("+==================================================+\n");
}
