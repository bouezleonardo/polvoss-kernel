//! PLIC control mechanisms.
//!
//! The riscv Platform Level Interrupt Controller 
//! (PLIC) is the hardware that manages external
//! devices interrupts. This is used to distinguish
//! between keyboard and disk interrupts. Read the 
//! chapters indicated of the RISC-V Platform-Level
//! Interrupt Controller  Specification.

use crate::config::constants::{PLIC, UART0, UART0_IRQ};
use crate::proc::processing::{cpu_id};

/// Enable interrupts from a device for a CPU.
/// Read chapter 6.
fn plic_enable_irq(cpu: u64, irq: u32) {
  unsafe {
    // Set the IRQ bit to 1
    ((PLIC + 0x2080 + cpu*0x100) as *mut u32).write_volatile(1<<irq);
  }
}

/// Set interrupt priority for keyboard and disk
pub fn init_plic() {
  // If the priority is 0, there will be no interrupts
  unsafe {
    // Write to the UART priority register
    ((PLIC + UART0_IRQ as u64 * 4) as *mut u32).write_volatile(1);
  }
}

/// Initialize PLIC interrupts for the current CPU
pub fn plic_enable() {
  // Each CPU has its own set of addresses 
  let cpu: u64 = cpu_id() as u64;
  
  // Enable UART0 interrupts
  plic_enable_irq(cpu, UART0_IRQ);
  
  unsafe {
    // Set the minimum priority level that is handled
    // by the CPU. Read chapter 7.
    ((PLIC + 0x201000 + cpu*0x2000) as *mut u32).write_volatile(0);
  }
}

/// Claim the interrupt from the PLIC. Read chapter 8.
/// # Return
/// The IRQ (id) of the interrupt tr
pub fn plic_claim() -> u32 { 
  let cpu: u64 = cpu_id() as u64;
  
  unsafe {
    // The read acknowledges the interrupt
    ((PLIC + 0x201004 + cpu*0x2000) as *const u32).read_volatile()
  }
}

/// Signal the interrupt was handled. Read chapter 9.
/// # Arguments
/// - `irq`: IRQ that was handled
pub fn plic_complete(irq: u32) { 
  let cpu: u64 = cpu_id() as u64;
  
  unsafe {
    // The write signals the interrupt was handled
    ((PLIC + 0x201004 + cpu*0x2000) as *mut u32).write_volatile(irq);
  }
}

