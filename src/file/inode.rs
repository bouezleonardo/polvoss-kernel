//! Inode module.
//!
//! An inode is the control structure
//! that provides metadata for objects
//! like common files, pipes and devices.

// Inode types
const FILE: u8          = 1;
const PIPE: u8          = 2;
const DIRECTORY: u8     = 3;
const DEVICE: u8        = 4;
const SOCKET: u8        = 5; // Not used
const SYMBOLIC_LINK: u8 = 6;

/// The DiskInode is the representation
/// of the inode data stored on the disk
pub struct DiskInode {
  inum: usize,      // Inode number
  itype: u8,        // Inode type
  major: u8,        // What kind of device this is
  minor: u8,        // Which device this is
  first_sector: usize, // First sector of the inode
  file_size: usize,    // File size (bytes)
  link_count: usize,  // Number of hardlinks to this inode
}

/// The Inode resides in main memory
pub struct Inode {
  open_count: u64,  // Number of active references to this inode
  idisk: DiskInode, // Disk inode
}
