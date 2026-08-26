//! External devices mechanisms.
//!
//! This module includes external devices 
//! mechanisms such as the device interrupt 
//! handler.

use crate::config::constants::UART0_IRQ;
use crate::io::uart::uart_intr;
use super::plic::*;

/// Interrupt handler for external devices
pub fn dev_intr() {
  // Claim the interrupt from the PLIC
  let irq: u32 = plic_claim();
  
  // If the interrupt was UART
  if irq == UART0_IRQ {
    uart_intr();
  } else {
    panic!("[trap_handlers]: unknown device interrupt.
            \n\r IRQ: {}", irq);
  }
  
  // Signal the handling is completed
  plic_complete(irq);
}
