//! Trap handlers for kernel and userspace.
//!
//! Trap handlers to treat interrupts and exceptions
//! from kernel and userspace.

use crate::riscv::supervisor_mode::*;
use crate::riscv::memory_types::{satp_format};
use crate::proc::processing::{cpu_id, current_proc,
                              current_proc_unwrap,
                              terminated};
use crate::proc::control_types::*;
use crate::proc::spin::*;
use crate::proc::sync::*;
use crate::memory::memory_layout::{USERVEC};
use super::kernelvec::kernelvec;
use super::uservec::uservec;
use super::trap_codes::*;
use super::trap_types::*;
use super::syscall_handler::syscall;
use super::syscall_proc::kexit;
use super::clock::clock_intr;
use super::device::dev_intr;

/// Write kernelvec address to stvec register
pub fn install_kernelvec() {
  write_stvec(kernelvec as *const() as usize);
}

/// Write uservec address to stvec register
pub fn install_uservec() {
  write_stvec(USERVEC);
}

/// Generate a software interrupt for testing
pub fn generate_interrupt() {
  unsafe {
    core::arch::asm!(
      "csrs sip, {0}",
      in(reg) (1 << 1)
    );
  }
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

/// User trap handler.
/// # Return
/// Address of the user page table
#[unsafe(no_mangle)] // Make it easy to call from assembly
pub extern "C" fn usertrap() -> usize {
  // Catch interrupt bit and code for the trap
  let (int, code): (usize, usize) = catch_cause();
  // Operating status of the machine
  let sstatus: usize = read_sstatus();
  // Process mutex
  let mutex: &'static Mutex<Pcb> = 
  current_proc_unwrap("trap_handlers");
  // Process guard
  let mut proc: MutexGuard<Pcb>;
  // Process trapframe
  let mut tpf: Trapframe;
  
  // Use kernelvec to handle traps while running
  // inside the kernel
  install_kernelvec();
  
  // Check if interrupts are still enabled
  if intr_enabled() {
    panic!("[trap_handlers]: usertrap interrupts enabled.");
  }
  // Check if the trap really came from U-mode
  if sstatus & SPP_U != SPP_U {
    panic!("[trap_handlers]: not from User mode.
    \n\r scause: {}\n\r sepc: {:#x}\n\r stval: {}\n\r sstatus: {}", 
    read_scause(), read_sepc(), read_stval(), sstatus);
  }
  
  // Check if the trap is an exception or interrupt
  if int == 0 {
    proc = mutex.lock();
    if code == ENVIRONMENT_CALL_FROM_U_MODE {
      // Check if the process terminated
      if terminated(&proc) {
        drop(proc);
        kexit(SIGKILL);
      }
          
      // Update PC to the instruction after ecall
      tpf = proc.trapframe();
      tpf.epc += 4;
      proc.write_trapframe(tpf);
      
      // Unlock mutex and turn on interrupts
      drop(proc);
      intr_on();
      
      syscall(); // Handle system call
    } else {
      panic!("[trap_handlers]: usertrap exception not handled.
      \n\r scause: {}\n\r sepc: {:#x}\n\r stval: {}\n\r PID: {}
      \n\r Desc: {}", read_scause(), read_sepc(), read_stval(), 
      proc.pid, desc_exception(code));
    }
  } else if int == 1 {
    if code == EXTERNAL_INT {
      dev_intr(); // Handle external device
    } else if code == TIMER_INT {
      clock_intr(); // Handle clock
      
      // Call the scheduler
      //yield(); 
    } else {
      proc = mutex.lock();
      panic!("[trap_handlers]: usertrap interrupt not handled.
      \n\r scause: {}\n\r sepc: {:#x}\n\r stval: {}\n\r PID: {}
      \n\r Desc: {}", read_scause(), read_sepc(), read_stval(), 
      proc.pid, desc_interrupt(code));
    }
  }
  // Disable interrupts for return
  intr_off();
  
  // Process PCB 
  proc = mutex.lock();
  
  // Check if the process terminated
  if terminated(&proc) {
    drop(proc);
    kexit(SIGKILL);
  }
  
  // Set sepc to the process program counter
  tpf = proc.trapframe();
  write_sepc(tpf.epc);
  
  // Restore sstatus in case it was modified
  // if yield was called
  write_sstatus(sstatus);
  
  // Restore user trap handler
  install_uservec();
  
  // Return process page table to userret
  satp_format(proc.pagetable().as_integer())
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
  if sstatus & SPP_S != SPP_S {
    panic!("[trap_handlers]: not from Supervisor mode.
    \n\r scause: {}\n\r sepc: {:#x}\n\r stval: {}\n\r sstatus: {}", 
    read_scause(), sepc, read_stval(), sstatus);
  }
  
  // Check if the trap is an exception or interrupt
  if int == 0 {
    // Panic if there is an exception in the kernel
    panic!("[trap_handlers]: kernel exception has occured.
    \n\r scause: {}\n\r sepc: {:#x}\n\r stval: {}\n\r Desc: {}", 
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
      \n\r scause: {}\n\r sepc: {:#x}\n\r stval: {}\n\r Desc: {}", 
      read_scause(), sepc, read_stval(), desc_interrupt(code));
    }
  }
  
  // Restore the sepc and sstatus in case they were
  // modified when yield was called
  write_sepc(sepc);
  write_sstatus(sstatus);
}
