//! Program loader.
//!
//! The program loader reads an ELF file and
//! prepares it's process image to execute.

use crate::riscv::memory_types::*;
use crate::fs::elf::*;
use crate::memory::virtual_memory::{grow_proc_image,
                                    walkaddr, copyout};
use crate::trap::trap_types::Trapframe;
use super::control_types::Pcb;
use super::spin::MutexGuard;
use super::init_test::init_addr;
use super::graphics_test::graphics_addr;

/// Load segments from the ELF file into
/// the pagetable memory. The pagetable must
/// have been already initialized by init_proc_image()
/// # Arguments
/// - `proc`: pagetable
/// - `path`: path of the ELF file
/// # Return
/// `true` if successful, `false` otherwise
pub fn 
load(path: &str, proc: &mut MutexGuard<Pcb>) 
-> bool {
  // FIXME: temporary solution before FS is working
  let opt: Option<Addr> = file_addr(path);
  if opt.is_none() {
    return false;
  }
  let file: Addr = opt.unwrap();
  
  // Read ELF header
  let ehdr: Elf32_Ehdr = file.read::<Elf32_Ehdr>();
  
  if !validate_elf_header(ehdr) {
    return false;
  }
  
  // Number and size of each program header
  let ph_num: usize = ehdr.e_phnum as usize;
  let ph_size: usize = ehdr.e_phentsize as usize;
  
  // First entry in the program header table
  let phdr_base: Addr = 
    file.clone() + ehdr.e_phoff as usize;

  let mut phdr_addr: Addr;
  
  // Walk through the program header table
  for i in 0..ph_num {
    phdr_addr = phdr_base.clone() + i*ph_size;  
      
    let phdr: Elf32_Phdr = 
    phdr_addr.read::<Elf32_Phdr>();
    
    if !validate_program_header(phdr) {
      continue;
    }
    
    // Address of the segment
    let seg_addr: Addr = 
      file.clone() + phdr.p_offset as usize;
    
    // Grow the process image with the virtual addresses
    let newbrk: Addr = 
      Addr::new(phdr.p_vaddr as u64 + phdr.p_memsz as u64);
    
    // Check if growing process image is needed
    if proc.brk <= newbrk {
      let perm: u8 = elf_to_pte_perm(phdr.p_flags);
      if !grow_proc_image(proc, newbrk, PTE_U|perm) {
        return false;
      }
    }
    
    let mut success: bool;
    success = copyout(proc.pagetable(), 
                 Addr::new(phdr.p_vaddr as u64), 
                 seg_addr, 
                 phdr.p_filesz as usize);
    
    if !success {
      return false;
    }
  }
  
  // Prepare trapframe
  let mut tpf: Trapframe = proc.trapframe();
  
  // Return to the program entry point after the kernel
  tpf.epc = ehdr.e_entry as usize;
  
  proc.write_trapframe(tpf);
  
  true
}

/// FIXME: this is a temporary solution
fn file_addr(path: &str) -> Option<Addr> {
  if path == "/init" {
    return Some(init_addr());
  } else if path == "/graphics" {
    return Some(graphics_addr());
  }
  None
}

