// mmio/monitor.rs

//! Monitor driver.
//!
//! The monitor driver is responsible for
//! writing to the monitor's memory region.

use crate::config::constants::{MONITOR_BASE,
                               MONITOR_WIDTH,
                               MONITOR_HEIGHT};

use crate::riscv::utils::write_byte;

pub fn putc(chr: u8){
  write_byte(chr, MONITOR_BASE);
}
