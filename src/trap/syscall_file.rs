//! File related system calls.
//!
//! This module contains the functions
//! for system calls related to files.

use crate::fs::file::*;
use crate::proc::processing::current_proc_unwrap;
use crate::proc::control_types::Pcb;
use crate::proc::spin::*;
use crate::riscv::memory_types::Addr;
use crate::config::constants::{USTACK_SIZE, ARG_SIZE, PATH_SIZE, PAGE_SIZE, NUM_ARG};
use crate::memory::virtual_memory::{shrink_proc_image, strcopyin, copyout};
use crate::memory::memory_layout::USTACK;
use crate::proc::loader::{load, load_check};
use core::str::{Utf8Error, from_utf8};
use super::trap_types::Trapframe;
use crate::memory::virtual_memory::walkaddr;

/// Read n bytes from a file and put it into a
/// a buffer.
/// # Wrapper 
/// `ssize_t read(int fd, void *buf, size_t n)`
pub fn sys_read() -> usize {
  // Current process
  let proc: &'static Mutex<Pcb> = current_proc_unwrap("read");
  
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
  let proc: &'static Mutex<Pcb> = current_proc_unwrap("write");
  
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
  // Current process
  let proc: &'static Mutex<Pcb> = current_proc_unwrap("open");
  
  // Get arguments from Trapframe
  let tpf: Trapframe = proc.lock().trapframe();
  let path: Addr = Addr::new(tpf.a0 as u64);
  let flags: u32 = tpf.a1 as u32;
  
  // Buffer to read the path
  let buf: [u8;PATH_SIZE] = [0;PATH_SIZE];
  
  // Copy the path string into buf
  let mut str_size: usize = strcopyin(proc.lock().pagetable(), 
                                Addr::to_addr(&buf), 
                                path, 
                                PATH_SIZE);
  
  // Check if there is more than just NULL or \0                 
  if str_size <= 1 {
    // There was no valid path
    return usize::MAX;
  }
  
  // Get the a string slice from the buffer (without \0)
  let opt: Result<&str, Utf8Error> = from_utf8(&buf[0..str_size-1]);
  if !opt.is_ok() {
    return usize::MAX;
  }
  let path_str: &str = opt.unwrap();
 
  open_inode_file(path_str, flags)
}

/// Close a file descriptor.
/// # Wrapper 
/// `int close(int fd)`
pub fn sys_close() -> usize {
  // Current process
  let mut proc: MutexGuard<Pcb> = current_proc_unwrap("close").lock();
  
  // Get arguments from Trapframe
  let tpf: Trapframe = proc.trapframe();
  let fd: usize = tpf.a0;
  
  // Check this file descriptor in the process file table
  if proc.files[fd].is_none() {
    return usize::MAX;
  }
  
  // Decrement the file reference count and free memory if needed
  close_file(proc.files[fd].unwrap().lock());
  
  // Remove file from file table
  proc.files[fd] = None;
  
  0
}

/// Load a file and execute it with arguments.
/// # Wrapper
/// `int execv(const char *path, char const *argv[])`
pub fn sys_execv() -> usize {
  // Current process
  let mut proc: MutexGuard<Pcb> = current_proc_unwrap("execv").lock();
    
  // Get arguments from Trapframe
  let mut tpf: Trapframe = proc.trapframe();
  let path: Addr = Addr::new(tpf.a0 as u64);
  let argv: Addr = Addr::new(tpf.a1 as u64);
  
  // Buffer to read the path
  let buf: [u8;PATH_SIZE] = [0;PATH_SIZE];
  
  // Copy the path string into buf
  let mut str_size: usize = strcopyin(proc.pagetable(), 
                                Addr::to_addr(&buf), 
                                path, 
                                PATH_SIZE);
  
  // Check if there is more than just NULL or \0                 
  if str_size <= 1 {
    // There was no valid path
    return usize::MAX;
  }
  
  // Get the a string slice from the buffer (without \0)
  let opt: Result<&str, Utf8Error> = from_utf8(&buf[0..str_size-1]);
  if !opt.is_ok() {
    return usize::MAX;
  }
  let path_str: &str = opt.unwrap();
  
  // Check if this path is able to be loaded
  if !load_check(path_str) {
    return usize::MAX;
  }
  
  // Reset the trapframe
  tpf = Trapframe::new();          // 0 initialized trapframe
  tpf.sp = USTACK + PAGE_SIZE;     // Top of the stack
  tpf.sp -= tpf.sp % 16;           // Must be 16 byte aligned
  
  // Initial Stack memory layout with arguments:
  // | string\0 string\0 string\0 *ptr *ptr *ptr   ....... |
  //        arguments              pointer array   ^sp 
  
  // This buffer holds the pointers to arguments.
  // Like argv[0], argv[1] ...
  let mut arg_pointers: [usize; NUM_ARG] = [0; NUM_ARG];
  
  // Arguments
  let mut args: [u8; ARG_SIZE*NUM_ARG] = [0;ARG_SIZE*NUM_ARG];
  
  // Argument count
  let mut argc: usize = 0;
  
  // Total argument size
  let mut total_arg_size: usize = 0;
  
  // Size of pointer array
  let mut ptr_array_size: usize = 0;
  
  // Check if the arguments are not NULL
  if argv.as_integer() != 0 {
    // FIXME: this is not working
    
    // Get the physical of address argv
    let argv_pa: Option<Addr> = walkaddr(proc.pagetable(), argv);
    
    if argv_pa.is_none() {
      return usize::MAX;
    }
   
    // Base of argument array (argv[0])
    let base: Addr = Addr::new(argv_pa.clone().unwrap().read::<usize>() as u64);
    
    // Read the first argument string
    str_size = strcopyin(proc.pagetable(), 
                        Addr::to_addr(&args), 
                        base.clone(), 
                        ARG_SIZE);
    
    // Invalid string (no null terminator)
    if str_size == 0 {
      return usize::MAX;
    }

    // The first argument is in the stack top
    arg_pointers[argc] = tpf.sp;
    
    // Read all strings
    while argc < NUM_ARG && total_arg_size < args.len() && str_size > 1 {       
      argc += 1;
      total_arg_size += str_size;
      
      // Calculate pointer  
      arg_pointers[argc] = arg_pointers[argc-1] + str_size;
      
      // Get the string the pointer refers to
      str_size = strcopyin(proc.pagetable(), 
              Addr::to_addr(&args) + total_arg_size, // Arg page pointer 
              base.clone()+argc*(usize::BITS/8) as usize, // argv pointer 
              ARG_SIZE);
      
      // Invalid string (no null terminator)
      if str_size == 0 {
        return usize::MAX;
      }
    }
    
    // Size of pointer array
    ptr_array_size = argc * (usize::BITS/8) as usize;
    
    // Check if the arguments and the pointer array 
    // fit into the stack
    if total_arg_size + ptr_array_size < USTACK_SIZE * PAGE_SIZE {
      return usize::MAX;
    }

    // argv will point to the first pointer after the arguments
    tpf.a1 = tpf.sp + total_arg_size;
  }
  
  // Shrink process memory to 0
  // This frees all user memory, but preserves the user stack,
  // trapframe, files and the pagetable.
  if !shrink_proc_image(&mut proc, Addr::new(0)) {
    return usize::MAX;
  }
  
  // Load the process image
  if !load(path_str, &mut proc) {
    return usize::MAX;
  }
  
  // Entry point after load
  tpf.epc = proc.trapframe().epc;  
  
  // Check if there were arguments
  // This is done here instead of earlier to avoid breaking
  // the process' memory before being sure everything went
  // alright
  if tpf.a1 != 0 {
    // Write arguments to the stack
    if !copyout(proc.pagetable(), 
                Addr::new(tpf.sp as u64), 
                Addr::to_addr(&args), 
                total_arg_size) {
      return usize::MAX;
    }
    
    // Write pointer array to the stack
    if !copyout(proc.pagetable(), 
                Addr::new(tpf.a1 as u64), 
                Addr::to_addr(&arg_pointers), 
                ptr_array_size) {
      return usize::MAX;
    }
    // Adjust sp to be after the pointer array
    tpf.sp = tpf.a1 + ptr_array_size;
    tpf.sp -= tpf.sp % 16;
  }
  
  // Update the trapframe
  proc.write_trapframe(tpf);
  
  // When this returns, the syscall handler will put argc
  // in the a0 register for _start to pass to main. The a1
  // is already set as the argv pointer in the trapframe
  argc
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
