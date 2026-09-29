//! Disk driver module.
//!
//! This module is reponsible for managing
//! the disk.

// MMIO layout and values come from:
// https://docs.oasis-open.org/virtio/virtio/v1.3/csd01/virtio-v1.3-csd01.html

const MAGIC_VALUE: u32 = 0x74726976;
const VERSION: u32 = 0x2;
const DISK_DEVICE_ID: u32 = 2;

const VIRTIO_BLK_T_IN           0 
const VIRTIO_BLK_T_OUT          1 
const VIRTIO_BLK_T_FLUSH        4 
const VIRTIO_BLK_T_GET_ID       8 
const VIRTIO_BLK_T_GET_LIFETIME 10 
const VIRTIO_BLK_T_DISCARD      11 
const VIRTIO_BLK_T_WRITE_ZEROES 13 
const VIRTIO_BLK_T_SECURE_ERASE   14


