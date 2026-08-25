//! System call handling.
//!
//! This module contains mechanisms for handling
//! and dispatching system calls, i.e. system call
//! codes and handler function.

use crate::proc::spin::*;
use crate::proc::processing::current_proc_unwrap;
use crate::proc::control_types::Pcb;
use super::trap_types::Trapframe;
use super::syscall::*;
use crate::print;

// System call codes to invoke a system call
const SYS_EXIT: usize = 1;
const SYS_FORK: usize = 2;
const SYS_READ: usize = 3;
const SYS_WRITE: usize = 4;
const SYS_OPEN: usize = 5;
const SYS_CLOSE: usize = 6;
const SYS_WAITPID: usize = 7;
const SYS_EXECV: usize = 8;
const SYS_PIPE: usize = 9;
const SYS_DUP: usize = 10;
const SYS_KILL: usize = 11;
const SYS_PAUSE: usize = 12;
const SYS_GETPID: usize = 13;
const SYS_SLEEP: usize = 14;

/// System call handler reads a7 register from
/// the process trapframe to dispatch a system
/// call.
/// This is called from the trap handlers.
pub fn syscall(){
  // Current process on this CPU
  let mutex: &'static Mutex<Pcb> = current_proc_unwrap("syscall");
  // Lock process mutex
  let mut proc: MutexGuard<Pcb> = mutex.lock();
  // Get trapframe
  let mut tpf: Trapframe = proc.trapframe();
  
  let mut ret: usize = 0; // Syscall return code
  let pid: usize = proc.pid; // Process pid
  
  drop(proc); // Unlock proc Mutex
  
  // Call the system call function
  match tpf.a7 {
    SYS_EXIT    => sys_exit(),
    SYS_FORK    => ret = sys_fork(),
    SYS_READ    => ret = sys_read(),
    SYS_WRITE   => ret = sys_write(),
    SYS_OPEN    => ret = sys_open(),
    SYS_CLOSE   => ret = sys_close(),
    SYS_WAITPID => ret = sys_waitpid(),
    SYS_EXECV   => ret = sys_execv(),
    SYS_PIPE    => ret = sys_pipe(),
    SYS_DUP     => ret = sys_dup(),
    SYS_KILL    => ret = sys_kill(),
    SYS_GETPID  => ret = sys_getpid(),
    SYS_SLEEP   => ret = sys_sleep(),
    _ => print!("\n\rUnknown system call. PID: {}", pid),
  }
  
  // Place return in the process trapframe
  proc = mutex.lock();
  tpf = proc.trapframe();
  tpf.a0 = ret;
  proc.update_trapframe(tpf);
}
