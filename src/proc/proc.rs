

use super::control_types::*;
use super::spin::Mutex;
use crate::memory::virtual_memory::{copyin};
use crate::config::constants::{NUM_PROC, NUM_CPU};
use crate::riscv::memory_types::{Addr, PageTable};
use crate::riscv::supervisor_mode::{read_tp};

/// Array of Pcb struct Mutexes for each process
static PCB: [Mutex<Pcb>; NUM_PROC] = [const{Mutex::new(Pcb::new())}; NUM_PROC];

/// Array of Cpu structs for each CPU
static mut CPU: [Cpu; NUM_CPU] = [const{Cpu::new()}; NUM_CPU];

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
pub fn current_cpu() -> Addr {
  let id: usize = cpu_id();

  // Get the address of the CPU struct
  unsafe {Addr::to_addr(&CPU[id])}
}

/// Get the current PCB address in this cpu
pub fn current_proc() -> Option<Addr> {
  //push_off();
  let cpu: Cpu = current_cpu().read::<Cpu>();
  let proc: Option<Addr> = cpu.proc;  
  //pop_off();
  
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
    // Get the current process PCB address
    let proc: Addr = current_proc().expect("[proc]: either_copyin.");
    
    // Copy from the user process using its pagetable
    return copyin(proc.read::<Pcb>().pagetable, dst, src, len);
  }
  
  // Copy len bytes from src to dst
  dst.copy::<u8>(src, len);
  
  true
}

