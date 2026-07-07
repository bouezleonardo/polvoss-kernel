//! Allocate and free physical memory frames.

use crate::config::constants::{RAM_SIZE,
                               PAGE_SIZE};

use super::memory_layout::{first_addr,
                           last_addr,
                           skernel_addr};

use crate::proc::spin::{Mutex,
                        MutexGuard};

use crate::riscv::memory_types::Addr;

/// Size of the frame bitmap. RAM_SIZE is divided
/// by PAGE_SIZE * 8 because the bits map the first 
/// addresses of the pages and the bits are inside
/// the 8-bit integers in the array.
const BITMAP_SIZE: usize = RAM_SIZE/(PAGE_SIZE*8);

/// Frame bitmap array that controls which frames
/// are used and which are free. For each entry
/// `0`: free, `1`: used.  
static BITMAP: Mutex<[u8;BITMAP_SIZE]> = 
               Mutex::new([0b00000000;BITMAP_SIZE]);

/// Last allocated frame address.
/// May help when searching for a new frame
/// to allocate
static LATEST: Mutex<u64> = Mutex::new(0);

/// Get indexes to access the bitmap
/// # Arguments
///  - `addr`: bit address to read
/// # Return
/// i index for the bitmap array and j index for the bits inside
fn get_indexes(addr: u64) -> (usize, usize) {
  // The count starts from the first address in skernel
  let index: usize = 
  ((addr - skernel_addr())/PAGE_SIZE as u64) as usize;
  
  // Get the position in BITMAP
  let i: usize = index / 8;
  
  // Get the position inside the values
  let j: usize = index % 8;
  
  if i >= BITMAP_SIZE {   
    panic!("[get_indexes]: index out of bounds.");
  }
  
  (i, j)
}

/// Read a bit from the bitmap
/// # Arguments
///  - `addr`: bit address to read
/// # Return
/// The bit referenced by index
fn read_bit(addr: u64) -> u8 {
  let (i, j): (usize, usize) = get_indexes(addr);
  
  //intr_off();
  
  let bitmap = BITMAP.lock();
  
  // Shift the bits to the right to get the least 
  // significant bit
  let bit: u8 = bitmap[i] >> j & 1;
  
  //intr_on();
  
  bit
}

/// Write a bit to the bitmap
/// # Arguments
///  - `addr`: bit address to write to
///  - `bit`: new bit value
fn write_bit(addr: u64, bit: u8) {
  let (i, j): (usize, usize) = get_indexes(addr);
  
  // Mask is used to reset the j bit to 0
  let mask: u8 = !(1 << j);
  
  //intr_off();
  let mut bitmap = BITMAP.lock();
  
  // Reset bit j in the i position
  bitmap[i] &= mask;
  
  // Set bit to new value
  bitmap[i] |= bit << j;

  //intr_on();
}

/// Get a free frame pointer 
/// # Return
/// Option containing the pointer or None
pub fn kmalloc() -> Option<Addr> { 
  //intr_off();
  
  // Return value
  let mut ret: Option<Addr> = None;
  
  // Last freed or allocated frame addr
  let mut latest: MutexGuard<u64> = LATEST.lock();
  
  // Last physical address
  let last_addr: u64 = last_addr();
  
  // Check if the page after latest is free
  let next: u64 = *latest+PAGE_SIZE as u64;
  if next < last_addr && read_bit(next) == 0 {
    *latest = next;
    write_bit(next, 1);
    ret = Some(Addr::new(next));
  }
  
  // If the page after latest was not free
  if ret.is_none() {
      let mut addr: u64 = first_addr();
      
      while addr < last_addr {
        // Return the free frame pointer
        if read_bit(addr) == 0 {
          *latest = addr;
          write_bit(addr, 1);
          ret = Some(Addr::new(addr));
          break;
        }
        
        addr += PAGE_SIZE as u64;
      }
  }
  
  //intr_on();
  ret
}

/// Free an used frame
/// # Arguments
///  - `ptr`: pointer to the page
pub fn kfree(ptr: Addr) {
  let addr: u64 = ptr.as_integer();
  
  if !addr.is_multiple_of(PAGE_SIZE as u64) {
    panic!("[kfree]: address not multiple of PAGE_SIZE.");
  }
  
  if addr < first_addr() || addr > last_addr() {
    panic!("[kfree]: address out of bounds.");
  }
  
  if read_bit(addr) == 0 {
    panic!("[kfree]: tried to free an unused frame.");
  }
  
  write_bit(addr, 0);
}

/// Set the last freed or allocated frame
/// to the first usable frame address
pub fn init_frame_alloc() {
  // Last freed or allocated frame addr
  let mut latest: MutexGuard<u64> = LATEST.lock();
  
  // First address after the end of the kernel that
  // is multiple of PAGE_SIZE.
  *latest = first_addr();
}
