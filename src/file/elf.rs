//! Executable and Linkable Format (ELF).
//!
//! The Executable and Linkable Format (ELF) 
//! is the standard format for object files in 
//! UNIX-like systems. It contains information
//! about a program's sections and entry point 
//! and how to load these sections' into segments
//! in memory.

// Read the ELF spec. for detailed information.

use crate::config::constants::{PAGE_SIZE};

/********************|TYPES|********************/

/// Unsigned program address 
type Elf32_Addr = u32;
/// Unsigned medium integer
type Elf32_Half = u16;
/// Unsigned file offset
type Elf32_Off = u32;
/// Signed large integer
type Elf32_Sword = i32;
/// Unsigned large integer
type Elf32_Word = u32;

/*****************|ELF HEADER|*****************/

/// Size of the ELF identification array 
const EI_NIDENT: usize = 16;

/// ELF file header. This is used as a "road map"
/// describing the file's organization.
#[derive(Clone, Copy)]
#[repr(C)] // Ensure struct follows C memory layout
pub struct Elf32_Ehdr {
  pub e_ident:    [u8;EI_NIDENT], // Magic number and other info
  pub e_type:     Elf32_Half, // Object file type
  pub e_machine:  Elf32_Half, // Architecture
  pub e_version:  Elf32_Word, // Object file version
  pub e_entry:    Elf32_Addr, // Entry point virtual address
  pub e_phoff:     Elf32_Off,   // Program header table file offset
  pub e_shoff:     Elf32_Off,   // Section header table file offset
  pub e_flags:     Elf32_Word, // Processor-specific flags
  pub e_ehsize:   Elf32_Half, // ELF header size in bytes
  pub e_phentsize:Elf32_Half, // Program header table entry size
  pub e_phnum:    Elf32_Half, // Program header table entry count
  pub e_shentsize:Elf32_Half, // Section header table entry size
  pub e_shnum:    Elf32_Half, // Section header table entry count
  pub e_shstrndx: Elf32_Half, // Section header string table index
}

// e_ident offsets
const EI_MAG0: usize    = 0; // 0x7F
const EI_MAG1: usize    = 1; // 'E'
const EI_MAG2: usize    = 2; // 'L'
const EI_MAG3: usize    = 3; // 'F'
const EI_CLASS: usize   = 4; // Class (32 bit/64 bit)
const EI_DATA: usize    = 5; // Byte Order
const EI_VERSION: usize	= 6; // ELF Version
const EI_PAD: usize	    = 7; // Padding

// The first 4 bytes of e_ident identify the file 
// as an ELF object using the magic number
const ELFMAG0: u8     = 0x7f; // e_ident[EI_MAG0]
const ELFMAG1: u8     = b'E'; // e_ident[EI_MAG1]
const ELFMAG2: u8     = b'L'; // e_ident[EI_MAG2] 
const ELFMAG3: u8     = b'F'; // e_ident[EI_MAG3]
const ELFCLASS32: u8  = 1;    // e_ident[EI_CLASS] value for 32 bit
const ELFDATA2LSB: u8 = 1;    // e_ident[EI_DATA] value for LSB
const EV_CURRENT: u8  = 1;    // e_ident[EI_VERSION] value for version

// e_type values
const ET_EXEC: Elf32_Half	= 2; // Absolute addresses executable File

/// RISC-V e_machine value
const EM_RISCV: Elf32_Half = 0xF3;

/// Check whether or not the ELF is valid
/// # Arguments
/// - `hdr`: ELF header of the file
/// # Return
/// `true` if the file is valid, `false` otherwise
pub fn 
validate_elf_header(hdr: Elf32_Ehdr) 
-> bool {
  // Check magic number
  if hdr.e_ident[EI_MAG0] != ELFMAG0 ||
     hdr.e_ident[EI_MAG1] != ELFMAG1 ||
     hdr.e_ident[EI_MAG2] != ELFMAG2 ||
     hdr.e_ident[EI_MAG3] != ELFMAG3 {
    return false;  
  }
  // Check OS support for this ELF file
  if hdr.e_ident[EI_CLASS]   != ELFCLASS32 ||
     hdr.e_ident[EI_DATA]    != ELFDATA2LSB || 
     hdr.e_ident[EI_VERSION] != EV_CURRENT ||
     hdr.e_machine           != EM_RISCV {
    return false;
  }
  // Check if the type is valid
  if hdr.e_type != ET_EXEC {
    return false;
  }
  true
}

/***************|PROGRAM HEADER|***************/

/// Program header. The program header describes
/// segments and information for program execution
#[derive(Copy, Clone)]
#[repr(C)] // Ensure struct follows C memory layout
pub struct Elf32_Phdr {
  pub p_type:  Elf32_Word, // Type of segment
  pub p_offset: Elf32_Off,   // Segment's offset from the beginning of the file
  pub p_vaddr: Elf32_Addr, // Virtual address where the segment starts
  pub p_paddr: Elf32_Addr, // Physical address where the segment starts (optional)
  pub p_filesz: Elf32_Word, // Size of the file image of the segment
  pub p_memsz: Elf32_Word, // Size of the memory image of the segment
  pub p_flags:  Elf32_Word, // Flags for segment permissions (R/W/E)
  pub p_align: Elf32_Word, // Alignment requirement for p_vaddr and p_offset
}

// p_type values
const PT_LOAD: Elf32_Word = 1; // Loadable segment

// p_flags values
const PT_X: Elf32_Word = 0x1; // Execute permission
const PT_W: Elf32_Word = 0x2; // Write permission
const PT_R: Elf32_Word = 0x4; // Read permission

/// Check whether or not the ELF segment indicated by
/// the program header is supported
/// # Arguments
/// - `hdr`: ELF program header
/// # Return
/// `true` if the file is valid, `false` otherwise
pub fn 
validate_program_header(hdr: Elf32_Phdr) 
-> bool {
  // Check the type
  if hdr.p_type != PT_LOAD {
    return false;
  }
  const PSZ: u32 = PAGE_SIZE as u32;
  
  // Check the alignment
  if hdr.p_align != PSZ ||
     !hdr.p_vaddr.is_multiple_of(PSZ) || 
     !hdr.p_offset.is_multiple_of(PSZ) {
    return false;
  }
  
  // Check sizes
  if hdr.p_memsz == 0 ||
     hdr.p_memsz < hdr.p_filesz{
    return false;
  }
  
  // Check permissions
  if hdr.p_flags != PT_X &&
     hdr.p_flags != PT_W &&
     hdr.p_flags != PT_R {
    return false;
  }
  
  true
}
