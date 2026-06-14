// memory/frame_alloc.rs

//! Allocate and free physical memory frames.

use crate::config::constants::{RAM_SIZE,
                               PAGE_SIZE};

use super::kernel_layout::{skernel_addr,
                           ekernel_addr};

use crate::proc::spin::Mutex;

use super::types::PhysAddr;

/// Size of the frame bitmap. RAM_SIZE is divided
/// by PAGE_SIZE * 8 because the bits map the first 
/// addresses of the pages and the bits are inside
/// the 8-bit integers in the array.
const BITMAP_SIZE: usize = RAM_SIZE/(PAGE_SIZE*8);

/// Frame bitmap array that controls which frames
/// are used and which are free. For each entry
/// `0`: free, `1`: used.  
static BITMAP: Mutex<[u8;BITMAP_SIZE]> = 
               Mutex::new([0b11111111;BITMAP_SIZE]);

/// Last frame address
fn last_addr() -> usize {
  // Last physical address in RAM
  skernel_addr() + RAM_SIZE
}

/// First frame address
fn first_frame_addr() -> usize {
  // First multiple of PAGE_SIZE after ekernel
  (ekernel_addr()+PAGE_SIZE-1)/PAGE_SIZE * PAGE_SIZE
}

/// Get indexes to access the bitmap
/// # Arguments
///  - `addr`: bit address to read
/// # Return
/// i index for the bitmap array and j index for the bits inside
fn get_indexes(addr: usize) -> (usize, usize) {
  // The count starts from the first address in skernel
  let index: usize = (addr - skernel_addr())/PAGE_SIZE;
  
  // Get the position in BITMAP
  let i: usize = index / 8;
  
  // Get the position inside the values
  let j: usize = index % 8;
  
  if i >= BITMAP_SIZE {   
    panic!("[read_bit]: index out of bounds.");
  }
  
  (i, j)
}

/// Read a bit from the bitmap
/// # Arguments
///  - `addr`: bit address to read
/// # Return
/// The bit referenced by index
fn read_bit(addr: usize) -> u8 {
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
fn write_bit(addr: usize, bit: u8) {
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
pub fn kmalloc() -> Option<PhysAddr> {
  let mut addr: usize = first_frame_addr();
  let last_addr: usize = last_addr();
  
  while addr < last_addr {
    // Return the free frame pointer
    if read_bit(addr) == 0 {
      write_bit(addr, 1);
      return Some(PhysAddr::new(addr));
    }
    
    addr += PAGE_SIZE;
  }
  None
}

/// Free an used frame
/// # Arguments
///  - `ptr`: pointer to the page
pub fn kfree(ptr: PhysAddr) {
  let addr: usize = ptr.get_addr();
  
  if addr % PAGE_SIZE != 0 {
    panic!("[kfree]: address not multiple of PAGE_SIZE.");
  }
  
  if addr < ekernel_addr() || addr > last_addr() {
    panic!("[kfree]: address out of bounds.");
  }
  
  if read_bit(addr) == 0 {
    panic!("[kfree]: tried to free an unused frame.");
  }
  
  write_bit(addr, 0);
}

/// Set the usable memory as free and test memory access
pub fn init_frame_alloc() {  
  // First address after the end of the kernel that
  // is multiple of PAGE_SIZE.
  let mut addr: usize = first_frame_addr();
  
  let last_addr: usize = last_addr();
  
  let mut ptr: PhysAddr = PhysAddr::new(addr);
  // Mark the usable memory as free
  while addr < last_addr {
    // Dereference the pointer to check if it is really valid
    ptr.read::<u8>();
    kfree(ptr);
    addr += PAGE_SIZE;
    ptr = PhysAddr::new(addr);
  }
}
