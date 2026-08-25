//! System call functions.
//!
//! This module contains the functions
//! for system calls.

use crate::config::constants::NUM_PROC;
use crate::proc::processing::current_proc_unwrap;
use crate::proc::spin::Mutex;
use crate::proc::sync::*;

/// Syncronize of kill() and pause()
static KILL_CVAR: Condvar = Condvar::new();
/// Which PIDs received a signal from kill() 
static KILLED: Mutex<[usize;NUM_PROC]> = 
Mutex::new([usize::MAX;NUM_PROC]);

/// Syncronize of exit() and waitpid()
static EXIT_CVAR: Condvar = Condvar::new();
/// Which PIDs called exit()
static EXITED: Mutex<[usize;NUM_PROC]> = 
Mutex::new([usize::MAX;NUM_PROC]);

/// Syncronize processes waiting for ticks
pub static TICKS_CVAR: Condvar = Condvar::new();
/// Count the number of ticks
pub static TICKS: Mutex<u64> = Mutex::new(0);

// Each system call function has a comment
// indicating which library wrapper function
// invokes it in user mode. 

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

/// Read n bytes from a file and put it into a
/// a buffer.
/// # Wrapper 
/// `ssize_t read(int fd, void *buf, size_t n)`
pub fn sys_read() -> usize {
  0
}

/// Write n bytes from a buffer to a file.
/// # Wrapper
/// `ssize_t write(int fd, const void *buf, size_t n)` 
pub fn sys_write() -> usize {
  0
}

/// Open and possibly create a file or device.
/// # Wrapper
/// `int open(const char *file, int flags);` 
pub fn sys_open() -> usize {
  0
}

/// Close a file descriptor.
/// # Wrapper 
/// `int close(int fd)`
pub fn sys_close() -> usize {
  0
}

/// Wait for a child process termination.
/// # Wrapper
/// `pid_t waitpid(pid_t pid, int *status)`
pub fn sys_waitpid() -> usize {
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

/// Send a signal to a process.
/// # Wrapper
/// `int kill(pid_t pid, int sig)`
pub fn sys_kill() -> usize {
  0
}

/// Wait for a signal.
/// # Wrapper 
/// `int pause(void)`
pub fn sys_pause() -> usize {
  0
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
  // Get the argument for sleep()
  
  
  
  let target_tick: usize = 0;

  0
}
