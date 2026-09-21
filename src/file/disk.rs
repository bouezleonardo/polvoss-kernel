//! Disk driver module.
//!
//! This module is reponsible for reading and
//! writing to the disk

/// Size of block header (bytes)
const BLOCK_HEADER: usize = 2*4;

/// Block represents a disk block when its
/// loaded into main memory 
pub struct Block {
  bnum: u32,   // Block number (not stored on the disk)
  
  // Block header
  next: u32,  // Next block number
  prev: u32,  // Previous block number
  
  // Block data
  data: [u8;BLOCK_SIZE - BLOCK_HEADER], // Sector data
}
