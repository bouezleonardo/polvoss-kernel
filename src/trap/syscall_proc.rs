//! Process related system calls.
//!
//! This module contains the functions
//! for system calls related to processes.

use crate::config::constants::{NUM_PROC, TICK_TIME};
use crate::memory::virtual_memory::{copyout};
use crate::proc::processing::{current_proc_unwrap,
                              current_proc_child,
                              call_scheduler, free_memory,
                              free_pcb, find_proc};
use crate::proc::control_types::{Pcb, ProcState, SIGKILL};
use crate::proc::spin::*;
use crate::proc::sync::*;
use crate::riscv::memory_types::{Addr};
use super::clock::{TICKS, TICKS_CVAR};
use super::trap_types::Trapframe;

/// Syncronize kill() and pause()
static KILL_CVAR: Condvar = Condvar::new();

/// Syncronize exit() and waitpid()
static EXIT_CVAR: Condvar = Condvar::new();

/*****************|AUXILIARY|******************/

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

/***************|SYSTEM CALLS|*****************/

// Each system call function has a comment
// indicating which library wrapper function
// invokes it in user mode.

// Read the RISC-V calling convention spec
// for information about function arguments.

/// Terminate a process.
/// # Wrapper
/// `void exit(int status)`
pub fn sys_exit() -> ! {
  let mutex: &'static Mutex<Pcb> = 
  current_proc_unwrap("exit");
  let mut proc: MutexGuard<Pcb> = mutex.lock();
  
  // Get status from trapframe argument
  proc.exit_status = proc.trapframe().a0 as i32;
  proc.state = ProcState::Zombie;
  
  // Free process' address space and Trapframe
  free_memory(&mut proc);
  
  // Notify all processes waiting
  EXIT_CVAR.notify_all();
  
  // Call scheduler
  call_scheduler(proc);
  
  panic!("[exit]: Zombie process still alive.");
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
  // Guard for child PCB
  let mut guard: MutexGuard<Pcb>;
  
  // Option for the child process
  let opt: Option<&'static Mutex<Pcb>>;
  
  // Current process
  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("waitpid");
  
  // Get arguments from Trapframe
  let tpf: Trapframe = proc.lock().trapframe();
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
  guard = child.lock();
  
  // Address of the child PCB's exit_status
  let exit: Addr = Addr::to_addr(&guard.exit_status);
  
  // While child is not Zombie
  while guard.state != ProcState::Zombie {
    // Wait until a exit() notifies waiting processes
    guard = EXIT_CVAR.wait(child, guard);
  }
  
  // Copy exit_status from child's PCB to the address
  // of the *status argument. Check if the stat address
  // is not 0 (NULL).
  if stat.as_integer() != 0 &&
  !copyout(proc.lock().pagetable.clone(), stat, exit, 4) {
    // If the copy is unsuccessful
    return usize::MAX;
  }
  
  // Free child's PCB
  free_pcb(&mut guard);
  
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
  // Current process
  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("kill");
  
  // Process that will receive the signal
  let mut target: MutexGuard<Pcb>;
  
  // Get arguments from Trapframe
  let tpf: Trapframe = proc.lock().trapframe();
  let pid: usize = tpf.a0;
  let sig: i32 = tpf.a1 as i32;
  
  // Search for the process with the pid
  let opt: Option<&'static Mutex<Pcb>> = 
  find_proc(pid);
  
  if opt.is_none() {
    return usize::MAX;
  }
  
  // Place the signal in the target PCB
  target = opt.unwrap().lock();
  target.kill_signal = sig;
  
  // Notify all processes waiting on pause()
  KILL_CVAR.notify_all();
  
  0
}

/// Wait for a signal to this PID.
/// # Wrapper 
/// `int pause(void)`
pub fn sys_pause() -> usize {
  // Guard for the PCB
  let mut guard: MutexGuard<Pcb>;
  // Process mutex
  let proc: &'static Mutex<Pcb> = 
  current_proc_unwrap("pause");
  
  // Wait while the kill_signal is the default value
  guard = proc.lock();
  while guard.kill_signal == i32::MAX {
    // Wait until a kill() notifies waiting processes
    // Process waits on its own PCB
    guard = KILL_CVAR.wait_self(proc, guard);
  }
  
  // Other signals may be added, only SIGKILL for now
  if guard.kill_signal == SIGKILL {
    drop(guard);
    kexit(SIGKILL);
  } else {
    // Clear kill_signal field
    guard.kill_signal = i32::MAX;
  }
  
  // This is equivalent to -1 when casting to i32
  usize::MAX
}

/// Get process PID.
/// # Wrapper
/// `pid_t getpid(void)`
pub fn sys_getpid() -> usize {
  current_proc_unwrap("getpid").lock().pid
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
