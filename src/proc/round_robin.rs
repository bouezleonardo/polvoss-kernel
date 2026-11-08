//! Round-robin scheduling algorithm

use crate::config::constants::NUM_PROC;
use super::spin::*;
use super::control_types::{Pcb, ProcState};
use super::processing::PCB;

/// Last selected index in the PCB array
static mut START: usize = 0;

/// Round-robin function. This is called in by the
/// scheduler.
/// # Return
/// Option containing the next process' guard and
/// mutex, or `None` if there is no process to be
/// executed
pub fn round_robin() 
-> Option<(MutexGuard<'static, Pcb>, &'static Mutex<Pcb>)> 
{
  let mut guard: MutexGuard<'static, Pcb>;
  unsafe {
    for i in START..NUM_PROC {
      // Lock the process mutex in the PCB array
      guard = PCB[i].lock();
      
      START = i+1;
      if(START == NUM_PROC) {
        START = 0;
      }
      
      // The first Ready process is returned to the
      // scheduler to be dispatched
      if guard.state == ProcState::Ready {
        return Some((guard, &PCB[i]));
      }
    }
  }
  None
}
