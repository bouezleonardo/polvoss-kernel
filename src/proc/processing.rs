
use super::control_types::*;
use super::spin::*;
use crate::trap::trap_types::*;
use crate::trap::syscall_proc::{sys_exit};
use crate::memory::virtual_memory::{copyin};
use crate::memory::frame_alloc::*;
use crate::config::constants::{NUM_PROC, NUM_CPU};
use crate::riscv::memory_types::{Addr, PageTable};
use crate::riscv::supervisor_mode::{read_tp, intr_enabled};
use crate::riscv::context_switch::*;

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

// All the following CPU functions must be called 
// with interrupts disabled to avoid a process 
// changing CPUs while holding the previous CPU's data

/// Get the current CPU's process.
pub fn current_proc() -> Option<&'static Mutex<Pcb>> {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].proc}
}
/// Get the current CPU's process PCB mutex.
/// Receives a string informing which module called
/// in the case of a panic.
pub fn current_proc_unwrap(loc: &str) -> &'static Mutex<Pcb> {
  let opt: Option<&'static Mutex<Pcb>> = current_proc();
  if opt.is_none() {
    panic!("[{}]: no process running.", loc);
  }
  // Get the mutex
  opt.unwrap()
}
/// Set the current CPU's process.
pub fn set_current_proc(proc: Option<&'static Mutex<Pcb>>) {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].proc = proc;}
}
/// Get the current CPU's context.
pub fn cpu_context() -> Context {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].ctx}
}
/// Set the current CPU's context.
pub fn set_cpu_context(ctx: Context) {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].ctx = ctx;}
}
/// Get the current CPU's number of of nested mutex locks.
pub fn cpu_lock_count() -> usize {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].lock_count}
}
/// Set the current CPU's number of nested mutex locks.
pub fn set_cpu_lock_count(lock_count: usize) {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].lock_count = lock_count;}
}
/// Get whether interrupts were enabled before locks.
pub fn cpu_intena() -> bool {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].intena}
}
/// Set whether interrupts were enabled before locks.
pub fn set_cpu_intena(intena: bool) {
  let id = cpu_id();
  assert!(!intr_enabled(), "[cpus]: interrupts enabled.");
  unsafe {CPU[id].intena = intena;}
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
    let opt: Option<&'static Mutex<Pcb>> = current_proc();
    
    // Get the current process PCB
    let proc: &'static Mutex<Pcb> = opt.expect("[proc]: either_copyin.");
    
    // Copy from the user process using its pagetable
    return copyin(proc.lock().pagetable.clone(), dst, src, len);
  }
  
  // Copy len bytes from src to dst
  dst.copy::<u8>(src, len);
  
  true
}

pub fn scheduler() -> ! {
  loop{}
}

/// Call scheduler from a process context. Drops
/// the process mutex guard before calling the
/// scheduler. The caller should change the process
/// state from 'Running' before calling.
/// # Arguments
/// - `proc`: process mutex guard to be dropped
pub fn call_scheduler(mut proc: MutexGuard<Pcb>) {
  if proc.state == ProcState::Running {
    panic!("[proc]: process state is still 'Running'.");
  }
  if intr_enabled() {
    panic!("[proc]: interrupts enabled before scheduler.");
  }
  if cpu_lock_count() != 1 {
    // The process should hold only 1 mutex (its own guard)
    // before calling the scheduler 
    panic!("[proc]: noff different than 1 before scheduler.");
  }
  
  // Get process context
  let proc_ctx: *mut Context = &mut proc.ctx as *mut Context;
  let cpu_ctx: *mut Context;
  unsafe{
    cpu_ctx = &mut CPU[cpu_id()].ctx as *mut Context;
  }
  
  // Save the intena
  let intena: bool = cpu_intena();
  
  // Drops mutex guard
  drop(proc);
  
  // Context switch from proc to the scheduler
  switch(proc_ctx, cpu_ctx);
  
  // Process coming back from scheduler, restore intena
  set_cpu_intena(intena);
}

/// Get the PCB of a child of the current process that 
/// has the specified PID.
/// # Arguments
/// - `pid`: child process' PID
/// # Return
/// Option containing the child that has the PID, 
/// None otherwise 
pub fn current_proc_child(pid: usize) 
-> Option<&'static Mutex<Pcb>>{
  // Current process
  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("proc");
  
  let mut parent: Option<&'static Mutex<Pcb>>;
  
  // Search the PCB array
  for i in 0..NUM_PROC {
    // Get the child's parent
    parent = PCB[i].lock().parent;
    
    // If the current process is the parent
    if parent.is_some() &&
       core::ptr::eq(proc, parent.unwrap()) {
      return Some(&PCB[i]);
    }
  }
  None
}

/// Check if a process is terminated
/// # Arguments
/// - `proc`: process' PCB guard
/// # Return
/// `true` if terminated, `false` otherwise
pub fn terminated(proc: &MutexGuard<Pcb>) -> bool{
  // Check if the process received a termination signal
  if proc.kill_signal == SIGKILL {
    return true;
  }
  false
}

/// Free process address space and Trapframe page
/// # Arguments
/// - `proc`: process' PCB guard 
pub fn free_memory(proc: &mut MutexGuard<Pcb>) {
  // Free Trapframe page
  if proc.trapframe.is_some() {
    // Using kfree with clone
    kfree(proc.trapframe.clone().unwrap());
    proc.trapframe = None;
  }
}

/// Free process PCB
/// # Arguments
/// - `proc`: process' PCB guard 
pub fn free_pcb(proc: &mut MutexGuard<Pcb>) {
  **proc = Pcb::new();
}

/// Exit the current process running on kernel 
/// mode by setting the Trapframe and calling
/// sys_exit syscall as if it were in user mode.
/// # Arguments
/// - `proc`: process' PCB guard
pub fn kexit(status: i32) -> ! {
  let mutex: &'static Mutex<Pcb> = 
  current_proc_unwrap("kexit");
  let mut proc: MutexGuard<Pcb> = mutex.lock();
  
  // Set the argument for status
  let mut tpf: Trapframe = proc.trapframe();
  tpf.a0 = status as usize;
  proc.write_trapframe(tpf);
  
  // Release lock
  drop(proc);
  
  // Call syscall function
  sys_exit();
}
