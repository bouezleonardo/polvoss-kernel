//! Program loader.
//!
//! The program loader reads an ELF file and
//! prepares it's process image to execute.

use crate::riscv::memory_types::*;
use crate::file::elf::*;

// Load segments from the ELF file into
// the pagetable 
pub fn 
load_segments(pgt: PageTable, file: Addr) 
-> bool {
  // Read ELF header
  let ehdr: Elf32_Ehdr = file.read::<Elf32_Ehdr>();
  
  if !validate_elf_header(ehdr) {
    return false;
  }
  
  // Number and size of each program header
  let ph_num: usize = ehdr.e_phnum as usize;
  let ph_size: usize = ehdr.e_phentsize as usize;
  
  // First entry in the program header table
  let mut phdr_addr: Addr = 
    file.clone() + ehdr.e_phoff as usize;
  
  // Walk through the program header table
  for i in 0..ph_num {
    phdr_addr += i*ph_size;
    
    let phdr: Elf32_Phdr = phdr_addr.read::<Elf32_Phdr>();
    
    if !validate_program_header(phdr) {
      return false;
    }
    
    
  }
  
  true
}
