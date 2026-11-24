//! Pipe mechanism.
//!
//! The pipe is an inter process communication
//! mechanism that allows processes to send
//! messages to each other using the file I/O
//! interface (read and write).

use crate::memory::memory_types::{Addr, PageTable};
use crate::memory::frame_alloc::{kmalloc, kfree};
use crate::config::constants::{NUM_FILE, PAGE_SIZE};
use crate::proc::spin::*;
use crate::proc::sync::*;
use crate::proc::processing::current_proc_unwrap;
use crate::memory::virtual_memory::{copyout, copyin};

/// Pipe struct.
/// Each pipe is a circular buffer. A page is dinamically allocated and freed
/// to avoid wasting too much memory.
pub struct Pipe {
  read_open: usize,   // Amount of read ends open
  write_open: usize,  // Amount of write ends open
  r_offset: usize,    // Read offset
  w_offset: usize,    // Write offset
  buf: Option<Addr>, // Page where data will be transfered
}
impl Pipe {
  pub const fn new() -> Self {
    Self{
      read_open: 0, write_open: 0, r_offset: 0, w_offset: 0, buf: None,
    }
  }
}

/// Array of pipes
static PIPE: [Mutex<Pipe>; NUM_FILE] = [const{Mutex::new(Pipe::new())};NUM_FILE];

/// Syncronize processes waiting to read from pipes
pub static PIPE_CVAR: Condvar = Condvar::new();

/// Allocate a free Pipe from the `PIPE` array 
pub fn open_pipe() -> Option<&'static Mutex<Pipe>> {
  // Search for an unallocated pipe
  for i in 0..NUM_FILE {
    let mut pipe: MutexGuard<Pipe> = PIPE[i].lock();
    
    if pipe.read_open == 0 && pipe.write_open == 0 {
      // Allocate a page for the buffer
      let buf: Option<Addr> = kmalloc();
      
      buf.as_ref()?;
      
      pipe.read_open = 1;
      pipe.write_open = 1;
      pipe.buf = buf;
      
      return Some(&PIPE[i]);
    }
  }
  None
}

/// Read the pipe's data and save it into 
/// a buffer a in user destination.
/// # Arguments
/// - `pipe_mtx`: pipe to read
/// - `dst`: destination address
/// - `len`: amount of bytes to read
pub fn 
read_pipe(pipe_mtx: &'static Mutex<Pipe>, dst: Addr, len: usize) 
-> usize {
  let mut pipe: MutexGuard<Pipe> = pipe_mtx.lock();
  
  if pipe.buf.is_none() {
    return usize::MAX;
  }

  // Current process' pagetable
  let pgt: PageTable = current_proc_unwrap("read_pipe").lock().pagetable();
  let buf: Addr = pipe.buf.clone().unwrap();
  
  // Copy each byte from the circular buffer
  for i in 0..len {
    // If there is nothing more to read
    while pipe.r_offset >= pipe.w_offset {
      // Wait for more data if p[1] is still open
      if pipe.write_open > 0 {
        pipe = PIPE_CVAR.wait(pipe_mtx, pipe);
      } else {
        return i;
      }
    }
  
    if !copyout(pgt.clone(), dst.clone()+i, buf.clone() + pipe.r_offset % PAGE_SIZE, 1) {
      return i; // Amount of bytes read to this point
    }
    pipe.r_offset += 1;
  }
  
  let pa: Addr = crate::memory::virtual_memory::va_to_pa(pgt.clone(), dst.clone()).unwrap();
  
   crate::print!("\n DST");
  for j in 0..100 {
    crate::print!(" {} ", (pa.clone()+j).read::<u8>());
  }
  
  len
}

pub fn 
write_pipe(pipe_mtx: &'static Mutex<Pipe>, src: Addr, len: usize) 
-> usize {
  let mut pipe: MutexGuard<Pipe> = pipe_mtx.lock();
  
  if pipe.buf.is_none() {
    return usize::MAX;
  }

  // Current process' pagetable
  let pgt: PageTable = current_proc_unwrap("write_pipe").lock().pagetable();
  let buf: Addr = pipe.buf.clone().unwrap();
  
  // Copy each byte from the circular buffer
  for i in 0..len {
    if !copyin(pgt.clone(), buf.clone() + pipe.w_offset % PAGE_SIZE, src.clone()+i, 1) {
      PIPE_CVAR.notify_all();
      return i; // Amount of bytes read to this point
    }
    pipe.w_offset += 1;
  }
  
  PIPE_CVAR.notify_all();
  len
}

/// Increment the pipe's read_count
pub fn dup_read_pipe(mut pipe: MutexGuard<Pipe>) { 
  pipe.read_open += 1;
}

/// Increment the pipe's write_count
pub fn dup_write_pipe(mut pipe: MutexGuard<Pipe>) { 
  pipe.write_open += 1;
}

/// Free the read end of a pipe
pub fn close_read_pipe(mut pipe: MutexGuard<Pipe>) {
  pipe.read_open -= 1;

  // Free buffer page
  if pipe.read_open == 0 && pipe.buf.is_some() {
    kfree(pipe.buf.clone().unwrap());
  }
  
  // Cleanup pipe
  if pipe.read_open == 0 && pipe.write_open == 0 {
    *pipe = Pipe::new();
  }
}

/// Free the write end of a pipe
pub fn close_write_pipe(mut pipe: MutexGuard<Pipe>) {
  pipe.write_open -= 1;

  // Check if this pipe should be freed
  if pipe.read_open == 0 && pipe.write_open == 0 {
    *pipe = Pipe::new();
  }
  
  // Wake up processes waiting to read
  PIPE_CVAR.notify_all();
}
