//! Program loader.
//!
//! The program loader reads an ELF file and
//! prepares it's process image to execute.

use crate::riscv::memory_types::*;
use crate::file::elf::*;

/// Load segments from the ELF file into
/// the pagetable memory. The pagetable must
/// have been already initialized by init_proc_image()
/// # Arguments
/// - `pgt`: pagetable
/// - `file`: ELF file
/// # Return
/// `true` if successful, `false` otherwise
pub fn 
load(pgt: PageTable, file: Addr) 
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
    
    // Address of the segment
    let seg_addr: Addr = file.clone() + phdr.p_offset as usize;
    
    // Read the
    /*for b in 
    
    
    pub p_vaddr: Elf32_Addr, // Virtual address where the segment starts
    pub p_filesz: Elf32_Word, // Size of the file image of the segment
    pub p_memsz: Elf32_Word, // Size of the memory image of the segment
    pub p_flags:  Elf32_Word, // Flags for segment permissions (R/W/E)*/
  }
  
  true
}
