//! File related system calls.
//!
//! This module contains the functions
//! for system calls related to files.

use crate::fs::file::*;
use crate::proc::processing::current_proc_unwrap;
use crate::proc::control_types::Pcb;
use crate::proc::spin::*;
use crate::riscv::memory_types::Addr;
use super::trap_types::Trapframe;

/// Read n bytes from a file and put it into a
/// a buffer.
/// # Wrapper 
/// `ssize_t read(int fd, void *buf, size_t n)`
pub fn sys_read() -> usize {
  // Current process
  let proc: &'static Mutex<Pcb> = 
    current_proc_unwrap("waitpid");
  
  // Get arguments from Trapframe
  let tpf: Trapframe = proc.lock().trapframe();
  let fd: usize = tpf.a0;
  let buf: Addr = Addr::new(tpf.a1 as u64);
  let len: usize = tpf.a2;
  
  read_file(fd, true, buf, len)
}

/// Write n bytes from a buffer to a file.
/// # Wrapper
/// `ssize_t write(int fd, const void *buf, size_t n)` 
pub fn sys_write() -> usize {
  // Current process
  let proc: &'static Mutex<Pcb> = 
    current_proc_unwrap("waitpid");
  
  // Get arguments from Trapframe
  let tpf: Trapframe = proc.lock().trapframe();
  let fd: usize = tpf.a0;
  let buf: Addr = Addr::new(tpf.a1 as u64);
  let len: usize = tpf.a2;
  
  write_file(fd, true, buf, len)
}

/// Open and possibly create a file or device.
/// # Wrapper
/// `int open(const char *path, int flags);` 
pub fn sys_open() -> usize {
  // FIXME: testing
  open_inode_file(&[0;1], O_RDWR)
}

/// Close a file descriptor.
/// # Wrapper 
/// `int close(int fd)`
pub fn sys_close() -> usize {
  0
}

/// Load a file and execute it with arguments.
/// # Wrapper
/// `int execv(const char *path, char *const argv[])`
pub fn sys_execv() -> usize {
  0
}

/// Create pipe.
/// # Wrapper
/// `int pipe(int p[2]);`
pub fn sys_pipe() -> usize {
  0
}

/// Return a new file descriptor referring to 
/// the a file.
/// # Wrapper
/// `int dup(int fd)`
pub fn sys_dup() -> usize {
  0
}
