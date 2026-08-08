

use super::control_types::*;
use super::spin::*;
use crate::memory::virtual_memory::{copyin};
use crate::config::constants::{NUM_PROC, NUM_CPU};
use crate::riscv::memory_types::{Addr, PageTable};
use crate::riscv::supervisor_mode::{read_tp};

/// Array of Pcb struct Mutexes for each process
static PCB: [Mutex<Pcb>; NUM_PROC] = [const{Mutex::new(Pcb::new())}; NUM_PROC];

/// Array of Cpu structs for each CPU
static CPU: [Mutex<Cpu>; NUM_CPU] = [const{Mutex::new(Cpu::new())}; NUM_CPU];

/// Next Process ID available
static NEXT_PID: Mutex<usize> = Mutex::new(1);

/// Get the current CPU ID.
/// Must be called with interrupts disabled
/// to avoid a process changing CPUs while
/// holding the previous CPU's data
pub fn cpu_id() -> usize {
  // The ID is preserved in tp register 
  // before entering S mode
  read_tp()
}

/// Get the current CPU struct.
/// Must be called with interrupts disabled
/// to avoid a process changing CPUs while
/// holding the previous CPU's data
pub fn current_cpu() -> &'static Mutex<Cpu> {
  let id: usize = cpu_id();

  // Get the CPU reference
  &CPU[id]
}

/// Get the current PCB in this cpu
pub fn current_proc() -> Option<&'static Mutex<Pcb>> {
  let cpu: MutexGuard<Cpu> = current_cpu().lock();
  let proc: Option<&'static Mutex<Pcb>> = cpu.proc;
  
  proc
}

/// Copy bytes to a kernel destination from a source address
/// either in kernel or userspace.
/// # Arguments
/// - `dst`: destination address
/// - `usr_src`: `true` if the source address is from a user
/// - `src`: source address
/// - `len`: length in bytes of the output
/// # Return
/// `true` if the copy is successful, `false` otherwise.
pub fn 
either_copyin(dst: Addr, usr_src: bool, src: Addr, len: usize) 
-> bool {
  // If it is an user address
  if usr_src {
    // Get the current process PCB
    let proc: &'static Mutex<Pcb>;
    proc = current_proc().expect("[proc]: either_copyin.");
    
    // Copy from the user process using its pagetable
    return copyin(proc.lock().pagetable.clone(), dst, src, len);
  }
  
  // Copy len bytes from src to dst
  dst.copy::<u8>(src, len);
  
  true
}

pub fn sleep(chan: &Mutex<u64>) {
   //let proc_addr: Addr = current_proc()
}
