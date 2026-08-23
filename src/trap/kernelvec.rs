//! Kernel trap vector.
//!
//! The function defined in this module must
//! be pointed to by the Supervisor Trap Vector 
//! Base Address (stvec) register to treat traps
//! from supervisor mode in the kernel. It uses
//! the current kernel stack. The kernelvec function
//! saves the callee registers.

use core::arch::global_asm;

// Using global assembly to allow aligning the 
// kernelvec function address to be used in the
// stvec. This is done to make the first two bits
// of the address always be zero. Read section 12.1.2. 
// of the RISC-V privileged doc.
global_asm!(r#"
  .globl kernelvec
  .align 2         # Align the function addr in 4 bytes 
  
  kernelvec:
    # Grow the stack to store the registers
    addi sp, sp, -128
    
    # Save the caller-saved registers in the stack
    # Every general purpose register is 4 bytes in RV32I
    sw ra, 0(sp)
    # Ignore sp
    sw gp, 8(sp)
    sw tp, 12(sp)
    sw t0, 16(sp)
    sw t1, 20(sp)
    sw t2, 24(sp)
    # Ignore s0-s1
    sw a0, 36(sp)
    sw a1, 40(sp)
    sw a2, 44(sp)
    sw a3, 48(sp)
    sw a4, 52(sp)
    sw a5, 56(sp)
    sw a6, 60(sp)
    sw a7, 64(sp)
    # Ignore s2-s11
    sw t3, 108(sp)
    sw t4, 112(sp)
    sw t5, 116(sp)
    sw t6, 120(sp)
    
    # Call Rust trap handler
    call kerneltrap
    
    # Restore the caller-saved registers in the stack
    lw ra, 0(sp)
    # Ignore sp
    lw gp, 8(sp)
    # tp stores the hartid that may have changed
    lw t0, 16(sp)
    lw t1, 20(sp)
    lw t2, 24(sp)
    # Ignore s0-s1
    lw a0, 36(sp)
    lw a1, 40(sp)
    lw a2, 44(sp)
    lw a3, 48(sp)
    lw a4, 52(sp)
    lw a5, 56(sp)
    lw a6, 60(sp)
    lw a7, 64(sp)
    # Ignore s2-s11
    lw t3, 108(sp)
    lw t4, 112(sp)
    lw t5, 116(sp)
    lw t6, 120(sp)
    
    # Shrink the stack
    addi sp, sp, 128
    
    # Return to the instruction before the trap
    sret
"#);

unsafe extern "C" {
  /// Accessible kernelvec symbol for rust code
  pub fn kernelvec();
}
