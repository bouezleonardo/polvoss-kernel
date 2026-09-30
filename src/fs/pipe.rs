//! Pipe mechanism.
//!
//! The pipe is an inter process communication
//! mechanism that allows processes to send
//! messages to each other using the file I/O
//! interface (read and write).

use crate::riscv::memory_types::Addr;
use crate::proc::spin::*;

/// Pipe struct.
pub struct Pipe {

}

pub fn free_pipe(mut file: MutexGuard<Pipe>) {

}

pub fn 
read_pipe(pipe: MutexGuard<Pipe>, 
  dst: Addr, 
  len: usize) 
-> usize {
  0
}

pub fn 
write_pipe(pipe: MutexGuard<Pipe>, 
  src: Addr, 
  len: usize) 
-> usize {
  0
}
