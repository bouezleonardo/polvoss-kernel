

use super::control_types::*;
use crate::proc::spin::Mutex;
use crate::config::constants::{NUM_PROC, NUM_CPU};
use crate::riscv::memory_types::Addr;
use crate::riscv::supervisor_mode::{read_tp};

/// Array of Pcb struct Mutexes for each process
static PCB: [Mutex<Pcb>; NUM_PROC] = [const{Mutex::new(Pcb::new())}; NUM_PROC];

/// Array of Cpu structs for each CPU
static mut CPU: [Cpu; NUM_CPU] = [const{Cpu::new()}; NUM_CPU];

/// Next Process ID available
static NEXT_PID: Mutex<usize> = Mutex::new(0);

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
pub fn this_cpu() -> Addr {
  let id: usize = cpu_id();
  let mut addr: Addr = Addr::new(0);
  // Get the address of the CPU struct
  unsafe {addr.to_addr::<Cpu>(&CPU[id]);}
  
  addr
}

/// Get the current process PCB in this cpu
pub fn this_proc() -> Option<Addr> {
  //push_off();
  let cpu: Cpu = this_cpu().read::<Cpu>();
  let proc: Option<Addr> = cpu.proc;  
  //pop_off();
  
  proc
}

