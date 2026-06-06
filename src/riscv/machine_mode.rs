// riscv/machine_mode.rs

//! Machine mode inline assembly functions.
//!
//! The following functions execute inline assembly
//! code for Machine mode.

use core::arch::asm;

/**************|MSTATUS REGISTER|****************/

// The Machine Status Register (mstatus) register 
// contains information about a CPU's operating
// state. Read section 3.1.6. of RISC-V privileged
// doc.

/// MPP Machine mode code
pub const MPP_M: usize = 3 << 11;

/// MPP Supervisor mode code
pub const MPP_S: usize = 1 << 11;

/// MPP User mode code
pub const MPP_U: usize = 0 << 11;

/// Read mstatus register
pub fn read_mstatus() -> usize {
  let mut mstatus: usize;
  
  // csrr reads mstatus into t0 register 
  unsafe{ asm!("csrr t0, mstatus", out("t0") mstatus); }
  
  mstatus
}

/// Write to mstatus register
pub fn write_mstatus(mstatus: usize) {

  // csrw writes t0 into mstatus
  unsafe{ asm!("csrw mstatus, t0", in("t0") mstatus); }
}

/**************|MEDELEG REGISTER|****************/

// The medeleg and medelegh registers control the
// delegation of exeptions to Supervisor mode.
// Read section 3.1.8. of RISC-V privileged doc.

/// Write to medeleg register
pub fn write_medeleg(medeleg: usize) {

  // csrw writes t0 into medeleg
  unsafe{ asm!("csrw medeleg, t0", in("t0") medeleg); }
}

/// Write to medelegh (represents the higher order bits of medeleg)
pub fn write_medelegh(medelegh: usize) {

  // csrw writes t0 into medelegh
  unsafe{ asm!("csrw medelegh, t0", in("t0") medelegh); }
}

/**************|MIDELEG REGISTER|****************/

// The mideleg register controls the delegation
// of interrupts to Supervisor mode. Read section
// 3.1.8. of RISC-V privileged doc.

/// Write to mideleg register
pub fn write_mideleg(mideleg: usize) {

  // csrw writes t0 into mideleg
  unsafe{ asm!("csrw mideleg, t0", in("t0") mideleg); }
}

/*******************|MRET|***********************/

// The mret instruction returns from Machine mode 
// and switches to the mode specified in MPP. It
// returns to the address saved in mepc. Read
// section 3.3.2. of RISC-V privileged doc.

/// Return from Machine mode
pub fn mret() -> ! {
  unsafe{ asm!("mret", options(noreturn)); }
}

/// Write to mepc register
pub fn write_mepc(addr: *const()) {
  // csrw writes t0 into mepc
  unsafe{ asm!("csrw mepc, t0", in("t0") addr); }
}
