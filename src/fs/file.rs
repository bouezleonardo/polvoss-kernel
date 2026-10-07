//! File module.
//!
//! A file is an abstraction that allows
//! processes to perform operations in
//! devices, files on disk or pipes.

use crate::riscv::memory_types::Addr;
use crate::config::constants::NUM_FILE;
use crate::io::console::{console_read, console_write};
use crate::proc::processing::current_proc_unwrap;
use crate::config::constants::CONSOLE;
use crate::proc::spin::*;
use crate::proc::control_types::Pcb;
use super::inode::*;
use super::pipe::*;

// File flags
pub const O_RDONLY: u32   = 1 << 0; // Read-only
pub const O_WRONLY: u32   = 1 << 1; // Write-only
pub const O_RDWR: u32     = 1 << 2; // Read-write

// Open flags
const O_CREAT: u32    = 1 << 3; // Create

// File types
const ORD: u8     = 1;  // Ordinary file
const PIPE: u8    = 2; // Pipe
const DIR: u8     = 3; // Directory
const DEV: u8     = 4; // Device
const SOCKET: u8  = 5; // Socket (not used)
const SLINK: u8   = 6; // Soft-link

/// Operations on devices.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Device {
  pub read: fn(bool, Addr, usize)->usize,
  pub write: fn(bool, Addr, usize)->usize,
}

/// File struct.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct File {
  pub open_count: u64, // Number of active references to this file
  pub ftype: u8, // File type
  pub inode: Option<&'static Mutex<Inode>>, 
  pub pipe: Option<&'static Mutex<Pipe>>,
  pub dev: Option<Device>, // Device operations
  pub offset: usize,  // Offset inside the file
  pub flags: u32,     // Flags
}
impl File {
  pub const fn new() -> Self {
    Self{
      open_count: 0, ftype: 0, inode: None,
      pipe: None, dev: None, offset: 0, flags: 0,
    }
  }
}

/// File array.
static FILE: [Mutex<File>;NUM_FILE] = 
  [const{Mutex::new(File::new())};NUM_FILE];

/// Get a device given a device number.
fn get_device(num: u16) -> Option<Device> {
  match num {
    CONSOLE => {
      Some(Device {
        read: console_read, write: console_write,
      })
    },
    _ => None,
  }
}

/// Allocate a free File from the `FILE` array 
fn alloc_file() -> Option<&'static Mutex<File>> {
  for i in 0..NUM_FILE {
    let mut file: MutexGuard<File> = FILE[i].lock();
    
    if file.open_count == 0 {
      file.open_count = 1;
      return Some(&FILE[i]);
    }
  }
  None
}

/// Open a file for an inode.
/// # Arguments
/// - `path`: inode path in the file system
/// - `flags`: file flags
/// # Return
/// The file descriptor if successful, -1 otherwise.
/// usize::MAX converts to -1 in signed integer.
pub fn 
open_inode_file(path: &[u8], flags: u32) 
-> usize {
  // Check if the flags are valid
  if flags & O_RDONLY == 0 && 
    flags & O_WRONLY == 0 &&
    flags & O_RDWR == 0 ||
    flags & (O_RDONLY|O_WRONLY) == O_RDONLY|O_WRONLY {
    return usize::MAX;
  }
  
  // Get the current process
  let mut proc: MutexGuard<Pcb> = 
    current_proc_unwrap("open_inode_file").lock();
  
  // Try to read the inode for this path
  let inode_opt: Option<&'static Mutex<Inode>>;
  inode_opt = open_inode(path);
  
  if inode_opt.is_none() {
    return usize::MAX;
  }
  
  let file_opt: Option<&'static Mutex<File>> = 
    alloc_file();
  
  if file_opt.is_none() {
    return usize::MAX;
  }
  
  let mut file: MutexGuard<File> = 
    file_opt.unwrap().lock();
  
  file.inode = inode_opt;
  
  // Lock the inode mutex
  let mut inode: MutexGuard<Inode> = 
    inode_opt.unwrap().lock();
  
  // Check inode type
  if inode.itype != ORD &&
    inode.itype != DIR &&
    inode.itype != DEV &&
    inode.itype != SLINK {
    close_file(file);
    return usize::MAX;
  }
  file.ftype = inode.itype;
  
  // If the inode refers to a device
  if file.ftype == DEV {
    let mut num: u16 = (inode.major as u16)<<8;
    num += inode.minor as u16;
    file.dev = get_device(num);
    
    if file.dev.is_none() {
      close_file(file);
      return usize::MAX;
    }
  }
  
  // Set file flags
  file.flags = flags;
  
  // Add file to the process table
  let fd: usize = proc.add_file(file_opt.unwrap());
  
  // If the add was unsuccessful
  if fd == usize::MAX {
    close_file(file);
  }
  
  fd
}

pub fn open_pipe_file(flags: u32) -> bool {
  true
}

pub fn 
read_file(fd: usize, usr_dst: bool, dst: Addr, len: usize) 
-> usize {
  // Check file descriptor
  if fd >= NUM_FILE {
    return usize::MAX;
  }
  
  // Get the current process
  let mut proc: MutexGuard<Pcb> = 
    current_proc_unwrap("read_file").lock();
  
  if proc.files[fd].is_none() {
    return usize::MAX;
  }
  
  // Get the file
  let mut file: MutexGuard<File> = 
    proc.files[fd].unwrap().lock();
  
  // Don't need process anymore
  drop(proc);
  
  // Check file flags
  if file.flags & O_RDONLY == 0 &&
     file.flags & O_RDWR == 0 {
    return usize::MAX;   
  }
  
  // Number of bytes read
  let mut bytes: usize = 0;
  
  // Check file type and read accordingly
  match file.ftype {
    ORD|DIR|SLINK => {
      if file.inode.is_some() { 
        bytes = read_inode(file.inode.unwrap().lock(), 
                           usr_dst,
                           dst, 
                           len);
      }
    },
    PIPE => { 
      if file.pipe.is_some() {
        bytes = read_pipe(file.pipe.unwrap().lock(), dst, len);
      }
    },
    DEV => {
      if file.dev.is_some() {
        let dev: Device = file.dev.unwrap();
        drop(file);
        bytes = (dev.read)(usr_dst, dst, len);
      }
    },
    _ => return usize::MAX,
  }
  bytes
}

pub fn 
write_file(fd: usize, usr_src: bool, src: Addr, len: usize) 
-> usize {
  // Check file descriptor
  if fd >= NUM_FILE {
    return usize::MAX;
  }
  
  // Get the current process
  let mut proc: MutexGuard<Pcb> = 
    current_proc_unwrap("write_file").lock();
  
  if proc.files[fd].is_none() {
    return usize::MAX;
  }
  
  // Get the file
  let mut file: MutexGuard<File> = 
    proc.files[fd].unwrap().lock();
  
  // Don't need process anymore
  drop(proc);
  
  // Check file flags
  if file.flags & O_WRONLY == 0 &&
     file.flags & O_RDWR == 0 {
    return usize::MAX;   
  }
  
  // Number of bytes read
  let mut bytes: usize = 0;
  
  // Check file type and read accordingly
  match file.ftype {
    ORD|DIR|SLINK => {
      if file.inode.is_some() {
        bytes = write_inode(file.inode.unwrap().lock(), 
                           usr_src,
                           src, 
                           len);
      }
    },
    PIPE => { 
      if file.pipe.is_some() {
        bytes = write_pipe(file.pipe.unwrap().lock(), src, len);
      }
    },
    DEV => {
      if file.dev.is_some() {
        bytes = (file.dev.unwrap().write)(usr_src, src, len);
      }
    },
    _ => return usize::MAX,
  }
  bytes
}

/// Close a file. This decrements the reference count
/// of the file. If the count reaches 0, the Inode or
/// Pipe associated with this file is also freed.
pub fn close_file(mut file: MutexGuard<File>) {
  if file.open_count == 0 {
    panic!("[close_file]: tried to close an unreferenced file.");
  }
  file.open_count -= 1;
  
  // Free inode or pipe
  if file.open_count == 0 {
    if file.inode.is_some() {
      free_inode(file.inode.unwrap().lock());
    } else if file.pipe.is_some() {
      free_pipe(file.pipe.unwrap().lock());
    }
    // Cleanup
    *file = File::new();
  }
}
