// boot/entry.rs

//! Kernel entry point.
//!
//! Initializes a stack for each hart and transfers control
//! to the Rust kernel code.

use core::arch::naked_asm;
use crate::config::constants::{NUM_CPU, PAGE_SIZE};

/*
At this point, nothing is ready yet. The stack pointer 
is not set up, interrupts and paging are disabled. Thus 
the kernel must setup a stack for each CPU hart for the
booting process.
*/

// Each hart will get access to a stack with the page size
static mut STACK0: [u8;PAGE_SIZE * NUM_CPU] = [0;PAGE_SIZE * NUM_CPU];

/// Entry function of the kernel
#[unsafe(naked)] // Dont add aditional assembly
#[unsafe(no_mangle)] // Disable name mangling
pub extern "C" fn _entry() -> ! {
  // Each stack will begin at stack0 + ((mhartid + 1) * PAGE_SIZE)
  naked_asm!(
    "la sp, {stack0}",   // Load the first address of STACK0
    "csrr t0, mhartid",  // t0 = mhartid
    "addi t0, t0, 1",    // t0 = t0 + 1
    "li t1, {page_size}",// t1 = page_size
    "li t2, 0",          // t2 = 0
    "j mul",             // t2 = t1 * t0
    "configure:",        // Configure label to return from mul
    "add sp, sp, t2",    // sp = sp + t2
    "call {hard_config}",// Call hardware configuration function
    
    "spin:",             // Spin if there is nothing more to do
    "j spin",

    "mul:",              // Multiply label
    "add t2, t2, t1",    // t2 = t2 + t1
    "addi t0, t0, -1",   // t0 = t0 + (-1) 
    "bnez t0, mul",      // if (t0 != 0) {goto mul}
    "j configure",       // Jump back to call hard_config

    stack0 = sym STACK0,
    page_size = const PAGE_SIZE,
    hard_config = sym super::hard_config::hard_config
  );
}
