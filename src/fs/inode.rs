//! Inode module.
//!
//! An inode is the control structure
//! that provides metadata for objects
//! like common files, pipes and devices.

use crate::config::constants::NUM_FILE;
use crate::proc::spin::*;
use crate::riscv::memory_types::Addr;

/// The DiskInode is the representation
/// of the inode data stored on the disk
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DiskInode {
  pub inum: usize,      // Inode number
  pub itype: u8,        // Inode type
  pub major: u8,        // What kind of device this is
  pub minor: u8,        // Which device this is
  pub first_sector: usize, // First sector of the inode
  pub file_size: usize,    // File size (bytes)
  pub link_count: usize,  // Number of hardlinks to this inode
}

/// The Inode resides in main memory
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Inode {
  pub open_count: usize,  // Number of active references to this inode
  
  // This is a copy from DiskInode
  pub inum: usize,      // Inode number
  pub itype: u8,        // Inode type
  pub major: u8,        // What kind of device this is
  pub minor: u8,        // Which device this is
  pub first_sector: usize, // First sector of the inode
  pub file_size: usize,    // File size (bytes)
  pub link_count: usize,  // Number of hardlinks to this inode
}
impl Inode {
  pub const fn new() -> Self {
    Self {
      open_count: 0, inum: 0,      
      itype: 0, major: 0,        
      minor: 0, first_sector: 0, 
      file_size: 0, link_count: 0,  
    }
  }
}

/// Inode array
static INODE: [Mutex<Inode>;NUM_FILE] = 
  [const{Mutex::new(Inode::new())};NUM_FILE];

/// Allocate a free File from the `FILE` array 
fn alloc_inode() -> Option<&'static Mutex<Inode>> {
  for i in 0..NUM_FILE {
    let mut inode: MutexGuard<Inode> = INODE[i].lock();
    
    if inode.open_count == 0 {
      inode.open_count = 1;
      return Some(&INODE[i]);
    }
  }
  None
}

/// Free an inode. This decrements the reference count
/// of the inode. When the count reaches 0, the inode
/// struct is considered as free to allocate.
pub fn free_inode(mut inode: MutexGuard<Inode>) {
  if inode.open_count == 0 {
    panic!("[free_inode]: tried to free an unreferenced inode.");
  }
  inode.open_count -= 1;
  
  if inode.open_count == 0 {
    *inode = Inode::new();
  }
}

/// Open an Inode. This reads the inode number
/// to check if the inode already is loaded in the
/// `INODE` array. If its not, then the inode is
/// loaded.
/// FIXME: this is not ready yet, just testing the console
pub fn 
open_inode(path: &str) 
-> Option<&'static Mutex<Inode>> {
  
  let inode: Option<&'static Mutex<Inode>> = 
    alloc_inode();
  
  if inode.is_none() {
    return None;
  }
  
  // Inode for the console
  *(inode.unwrap().lock()) = Inode {
    open_count: 1, inum: 0,      
    itype: 4, major: 0,        
    minor: 1, first_sector: 0, 
    file_size: 0, link_count: 0,
  };
  
  inode
}

/// Read the inode's data and save it into 
/// a buffer either in user or kernel destination.
/// # Arguments
/// - `inode`: inode to read
/// - `usr_dst`: if the destination is in userspace
/// - `dst`: destination address
/// - `len`: amount of bytes to read
pub fn 
read_inode(inode: MutexGuard<Inode>, 
  usr_dst: bool, 
  dst: Addr, 
  len: usize)
-> usize {
  0
}

/// Write to the inode's data from a buffer 
/// in user or kernel space.
/// # Arguments
/// - `inode`: inode to read
/// - `usr_src`: if the source is in userspace
/// - `src`: source address
/// - `len`: amount of bytes to read
pub fn 
write_inode(inode: MutexGuard<Inode>, 
  usr_src: bool, 
  src: Addr, 
  len: usize)
-> usize {
  0
}
