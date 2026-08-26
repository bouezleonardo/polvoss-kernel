//! Process related system calls.
//!
//! This module contains the functions
//! for system calls related to processes.

use crate::config::constants::{NUM_PROC, TICK_TIME};
use crate::memory::virtual_memory::{copyout};
use crate::proc::processing::{current_proc_unwrap,
                              current_proc_child};
use crate::proc::control_types::Pcb;
use crate::proc::spin::*;
use crate::proc::sync::*;
use crate::riscv::memory_types::{Addr};
use super::clock::{TICKS, TICKS_CVAR};
use super::trap_types::Trapframe;

/// Type alias for a process array
type ProcArray = [Option<&'static Mutex<Pcb>>;NUM_PROC];

/// Syncronize of kill() and pause()
static KILL_CVAR: Condvar = Condvar::new();
/// Which processes received a signal from kill() 
static KILLED: Mutex<ProcArray> = 
Mutex::new([None; NUM_PROC]);

/// Syncronize of exit() and waitpid()
static EXIT_CVAR: Condvar = Condvar::new();
/// Which processes called exit()
static EXITED: Mutex<ProcArray> = 
Mutex::new([None; NUM_PROC]);

/************|AUXILIARY FUNCTIONS|**************/

/// Search for a PID inside an array protected
/// by a mutex guard and replace it
fn 
find(guard: &mut MutexGuard<ProcArray>, 
pid: usize) -> bool {
  let mut proc: &'static Mutex<Pcb>;
  for i in 0..guard.len() {
    if guard[i].is_none() {
      continue;
    }
    
    proc = guard[i].unwrap();
    
    if proc.lock().pid == pid {
      guard[i] = None;
      return true;
    }
  }
  false
}

/***************|SYSTEM CALLS|*****************/

// Each system call function has a comment
// indicating which library wrapper function
// invokes it in user mode.

// Read the RISC-V calling convention spec
// for information about function arguments.

/// Terminate a process.
/// # Wrapper
/// `void exit(int status)`
pub fn sys_exit() {
   
}

/// Create a new process that is a copy of
/// the original's address space.
/// # Wrapper 
/// `pid_t fork(void)`
pub fn sys_fork() -> usize {
  0
}


/// Wait for a child process termination.
/// # Wrapper
/// `pid_t waitpid(pid_t pid, int *status)`
pub fn sys_waitpid() -> usize {
  // Guard for EXITED
  let mut guard: MutexGuard<ProcArray>;
  
  // Option for the child process
  let opt: Option<&'static Mutex<Pcb>>;
  
  // Current process
  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("waitpid");
  
  // Trapframe
  let tpf: Trapframe = proc.lock().trapframe();
  
  // Get arguments
  let pid: usize = tpf.a0;
  let stat: Addr = Addr::new(tpf.a1 as u64);
  
  // Get the child process with this PID
  opt = current_proc_child(pid);
  
  // Check if this process has a child with this PID
  if opt.is_none() {
    // Equivalent to -1 when casting
    return usize::MAX;
  }
  
  // Get child process
  let child: &'static Mutex<Pcb> = opt.unwrap();
  
  // Address of the child PCB's exit_status
  let exit: Addr = 
  Addr::to_addr(&child.lock().exit_status);
  
  // While PID not found
  guard = EXITED.lock();
  while !find(&mut guard, pid) {
    // Wait until a exit() notifies waiting processes
    guard = EXIT_CVAR.wait(&EXITED, guard);
  }
  
  // Copy exit_status from child's PCB to the address
  // of the *status argument.
  if !copyout(proc.lock().pagetable.clone(), stat, exit, 4) {
    // If the copy is unsuccessful
    return usize::MAX;
  }
  pid
}

/// Load a file and execute it with arguments.
/// # Wrapper
/// `int execv(const char *path, char *const argv[])`
pub fn sys_execv() -> usize {
  0
}

/// Send a signal to a process.
/// # Wrapper
/// `int kill(pid_t pid, int sig)`
pub fn sys_kill() -> usize {
  0
}

/// Wait for a signal to this PID.
/// # Wrapper 
/// `int pause(void)`
pub fn sys_pause() -> usize {
  // Guard for the KILLED array
  let mut guard: MutexGuard<ProcArray>;

  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("pause");
  
  let pid: usize = proc.lock().pid;
  
  // While PID not found
  guard = KILLED.lock();
  while !find(&mut guard, pid) {
    // Wait until a kill() notifies waiting processes
    guard = KILL_CVAR.wait(&KILLED, guard);
  }
  
  // This is equivalent to -1 when casting to i32
  usize::MAX
}

/// Get process PID.
/// # Wrapper
/// `pid_t getpid(void)`
pub fn sys_getpid() -> usize {
  current_proc_unwrap("[getpid]").lock().pid
}

/// Sleep for the specified number of ticks.
/// # Wrapper
/// `unsigned sleep(unsigned ticks)`
pub fn sys_sleep() -> usize {  
  // Get the current TICKS count
  let mut guard: MutexGuard<u64>;
  
  // Current process PCB
  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("sleep");
  
  let tpf: Trapframe = proc.lock().trapframe();
  
  // Get the argument for sleep()
  let mut ticks: u64 = 
  tpf.a0 as u64+(tpf.a1 as u64)<<32;
  
  // Get the current TICKS count
  guard = TICKS.lock();
  ticks += *guard;
  
  // Wait until TICKS reaches the specified value 
  while *guard < ticks {
    guard = TICKS_CVAR.wait(&TICKS, guard);
  }
  
  0
}

/// Return the system uptime in (ms).
/// # Wrapper
/// `unsigned uptime(void)`
pub fn sys_uptime() -> usize {
  (*TICKS.lock() * TICK_TIME) as usize
}
