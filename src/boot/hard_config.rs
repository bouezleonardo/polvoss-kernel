// boot/hard_config.rs

//! Configure hardware interrupts and mode of operation.
//!
//! Delegate all interrupts and exception to Supervisor
//! mode, enable interrupts and switch from Machine to
//! Supervisor mode calling the start function.

use crate::riscv::machine_mode::{MPP_M, 
                                 MPP_S,
                                 read_mstatus,
                                 write_mstatus,
                                 write_mideleg,
                                 write_medeleg,
                                 write_pmpcfg0,
                                 write_pmpaddr0,
                                 write_mepc,
                                 mret
                                 };
                                 
use crate::riscv::supervisor_mode::{SIE_STIE, 
                                 SIE_SEIE, 
                                 read_sie, 
                                 write_sie};

/// Configure hardware interrupts and mode of operation
pub fn hard_config() -> ! {
  
  // Switch from Machine to Supervisor mode
  let mut mstatus: usize = read_mstatus();
  mstatus &= !MPP_M; // Overwrite previous MPP
  mstatus |= MPP_S;  // Set MPP to Supervisor
  write_mstatus(mstatus);
  
  // Delegate all interrupts and exeptions to S mode
  write_mideleg(0xffff); // All interrupts
  write_medeleg(0xffff); // All exceptions
  
  // Enable interrupts in S mode
  let mut sie: usize = read_sie();
  sie |= SIE_STIE;   // Enable timer interrupts
  sie |= SIE_SEIE;   // Enable external interrupts
  write_sie(sie);
  
  // Allow S mode access all physical memory
  // Read, Write, Execute and TOR matching
  write_pmpcfg0(0xf); 
  
  // All addresses such that 0 <= addr < 0xffffffff
  // are valid
  write_pmpaddr0(0xffffffff);
  
  // Set mepc to the start funtion address
  write_mepc(super::start::start as *const ());

  // Return from M mode to switch to S mode
  // Will return to the address in mepc
  mret();
}
