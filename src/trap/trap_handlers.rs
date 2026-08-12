//! Trap handlers for kernel and userspace.
//!
//! Trap handlers to treat interrupts and exceptions
//! from kernel and userspace.

use crate::riscv::supervisor_mode::*;
use crate::proc::processing::{cpu_id, current_proc};
use crate::proc::spin::*;
use crate::proc::sync::*;
use crate::config::constants::{TICK_TIME, MILISECOND};
use super::kernelvec::kernelvec;
use super::trap_codes::*;

/// Count the number of ticks
pub static TICKS: Mutex<u64> = Mutex::new(0);

/// Condition variable to syncronize processes
/// waiting for ticks
pub static TICKS_CVAR: Condvar = Condvar::new();

/// Write kernelvec address to stvec register
pub fn install_kernelvec() {
  write_stvec(kernelvec as *const() as usize);
}

/// Get interrupt bit and exception code
// Read section 12.1.8. of RISC-V privileged doc.
fn catch_cause() -> (usize, usize) {
  // Interrupt bit (last bit)
  let interrupt: usize = read_scause() >> 31;
  // Exception code bits (except last bit)
  let code: usize = read_scause() & 0x7fffffff;
  
  (interrupt, code)  
}

/// Interrupt handler for external devices
fn dev_intr() {

}

/// Interrupt handler for clock
fn clock_intr() {
  // Only CPU 0 should increment ticks to avoid
  // incrementing extra times
  if cpu_id() == 0 {
    // Increment ticks
    let mut ticks: MutexGuard<u64> = TICKS.lock();
    *ticks += 1;
    // Wakes up all sleeping processes on this condvar
    TICKS_CVAR.notify_all();
  }
  // Write next time to have a clock interrupt
  write_stimecmp(read_time()+TICK_TIME*MILISECOND);
}

/// Kernel trap handler
#[unsafe(no_mangle)] // Make it easy to call from assembly
pub extern "C" fn kerneltrap() {
  // Catch interrupt bit and code for the trap
  let (int, code): (usize, usize) = catch_cause();
  // PC saved when the trap occured
  let sepc: usize = read_sepc();
  // Operating status of the machine
  let sstatus: usize = read_sstatus();
  
  // Check if interrupts are still enabled
  if intr_enabled() {
    panic!("[trap_handlers]: kerneltrap interrupts enabled.");
  }
  // Check if the trap really came from S-mode
  if sstatus & SPP_S != 1 {
    panic!("[trap_handlers]: not from Supervisor mode.");
  }
  
  // Check if the trap is an exception or interrupt
  if int == 0 {
    // Panic if there is an exception in the kernel
    panic!("[trap_handlers]: kernel exception has occured.
          \n scause: {}\n sepc: {}\n stval: {}\n Desc: {}", 
          read_scause(), sepc, read_stval(), desc_exception(code));
  } else if int == 1 {
    if code == EXTERNAL_INT {
      dev_intr(); // Handle external device
    } else if code == TIMER_INT {
      clock_intr(); // Handle clock
      
      // If the CPU should be given to a process
      /*if current_proc().is_some() {
        yield(); // Call the scheduler
      }*/
    } else {
      panic!("[trap_handlers]: kerneltrap interrupt not handled.
            \n scause: {}\n sepc: {}\n stval: {}\n Desc: {}", 
            read_scause(), sepc, read_stval(), desc_interrupt(code));
    }
  } else {
    panic!("[trap_handlers]: invalid kerneltrap interrupt bit.
            \n scause: {}\n sepc: {}\n stval: {}\n Bit: {}", 
            read_scause(), sepc, read_stval(), int);
  }
  
  // Restore the sepc and sstatus in case they were
  // modified when yield was called
  write_sepc(sepc);
  write_sstatus(sstatus);
}
