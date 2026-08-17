// boot/hard_config.rs

//! Configure hardware for the kernel startup.
//!
//! Delegate all interrupts and exceptions to Supervisor
//! mode, enable interrupts, grant access to all physical
//! memory to Supervisor mode and switch from Machine to
//! Supervisor mode calling the start function.

use crate::riscv::machine_mode::*;
use crate::config::constants::{MILISECOND, TICK_TIME};                         
use crate::riscv::supervisor_mode::{SIE_STIE, SIE_SEIE, SIE_SSIE,
                                   read_sie, write_sie, write_tp, 
                                   read_time, write_stimecmp};

/// Configure clock
fn clock_init() {
  // Enable timer interrupts
  write_mie(read_mie() | MIE_STIE);
  
  // Allow Supervisor mode to use stimecmp
  write_menvcfgh(read_menvcfgh() | (1 << 31)); 
  write_mcounteren(read_mcounteren() | 2);
  
  // First clock interrupt
  write_stimecmp(read_time()+MILISECOND*TICK_TIME);
}

/// Hardware configuration function
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
  sie |= SIE_SSIE;   // Enable software interrupts
  write_sie(sie);
  
  // Allow S mode access all physical memory
  // Read, Write, Execute and TOR matching
  write_pmpcfg0(0xf); 
  
  // All addresses such that 0 <= addr < 0xffffffff
  // are valid
  write_pmpaddr0(0xffffffff);
  
  // Set mepc to the start funtion address
  write_mepc(super::start::start as *const () as usize);

  // Save CPU id in the tp register because
  // mhartid will become inaccessible in S mode
  write_tp(read_mhartid());
  
  // Initialize clock
  clock_init();
  
  // Return from M mode to switch to S mode
  // Will return to the address in mepc
  mret();
}
