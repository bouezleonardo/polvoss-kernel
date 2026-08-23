//! User trap vector.
//!
//! The function defined in this module must
//! be pointed to by the Supervisor Trap Vector 
//! Base Address (stvec) register to treat traps
//! from user mode. It uses the kernel stack of the
//! calling process saved on the trapframe.

use core::arch::global_asm;

// Using global assembly to allow aligning the 
// uservec function address to be used in the
// stvec. This is done to make the first two bits
// of the address always be zero. Read section 12.1.2. 
// of the RISC-V privileged doc.
global_asm!(r#"
  .globl uservec
  .align 2         # Align the function addr in 4 bytes 
  
  uservec:
    # sscratch is used as an auxiliary to save a0
    # Read section 12.1.6. of RISC-V privileged doc.
    csrw sscratch, a0
    
    # Using a0 as pointer to TRAPFRAME
    li a0, TRAPFRAME
    
    # The next instructions follow the layout of the 
    # Trapframe struct
    
    # Save the caller-saved registers in the trapframe
    # Every general purpose register is 4 bytes in RV32
    sw ra, 16(a0)
    sw sp, 20(a0)
    sw gp, 24(a0)
    sw tp, 28(a0)
    sw t0, 32(a0)
    sw t1, 36(a0)
    sw t2, 40(a0)
    sw s0, 44(a0)
    sw s1, 48(a0)
    # Ignore a0
    sw a1, 56(a0)
    sw a2, 60(a0)
    sw a3, 64(a0)
    sw a4, 68(a0)
    sw a5, 72(a0)
    sw a6, 76(a0)
    sw a7, 80(a0)
    sw s2, 84(a0)
    sw s3, 88(a0)
    sw s4, 92(a0)
    sw s5, 96(a0)
    sw s6, 100(a0)
    sw s7, 104(a0)
    sw s8, 108(a0)
    sw s9, 112(a0)
    sw s10, 116(a0)
    sw s11, 120(a0)
    sw t3, 124(a0)
    sw t4, 128(a0)
    sw t5, 132(a0)
    sw t6, 136(a0)
    
    # Save a0 value on trapframe using t0
    csrr t0, sscratch
    sw t0, 52(a0)
    
    # Load kernel page table address
    lw t0, 0(a0)
    
    # Load process kernel stack pointer
    lw sp, 4(a0)
    
    # Load hartid
    lw tp, 8(a0)
    
    # Save sepc in the trapframe. sepc holds the   
    # address of the instruction that was trapped
    # Read section 12.1.7. of RISC-V privileged doc.
    csrr t1, sepc
    sw t1, 12(a0)
    
    # Wait for previous memory operations to complete
    sfence.vma zero, zero 
     
    # Switch from user to kernel page table
    csrw satp, t0
    
    # Flush TLB
    sfence.vma zero, zero 
    
    # Call Rust trap handler
    call usertrap
    
    # After the trap is handled, usertrap() returns
    # to uservec here
    
    # usertrap() return value is the user page table
    # and it goes to a0
    
    # Restore user page table
    sfence.vma zero, zero 
    csrw satp, a0
    sfence.vma zero, zero 
    
    # Using a0 as pointer to TRAPFRAME
    li a0, TRAPFRAME
    
    # Load the caller-saved registers in the trapframe
    # Every general purpose register is 4 bytes in RV32
    lw ra, 16(a0)
    lw sp, 20(a0)
    lw gp, 24(a0)
    lw tp, 28(a0)
    lw t0, 32(a0)
    lw t1, 36(a0)
    lw t2, 40(a0)
    lw s0, 44(a0)
    lw s1, 48(a0)
    # Ignore a0
    lw a1, 56(a0)
    lw a2, 60(a0)
    lw a3, 64(a0)
    lw a4, 68(a0)
    lw a5, 72(a0)
    lw a6, 76(a0)
    lw a7, 80(a0)
    lw s2, 84(a0)
    lw s3, 88(a0)
    lw s4, 92(a0)
    lw s5, 96(a0)
    lw s6, 100(a0)
    lw s7, 104(a0)
    lw s8, 108(a0)
    lw s9, 112(a0)
    lw s10, 116(a0)
    lw s11, 120(a0)
    lw t3, 124(a0)
    lw t4, 128(a0)
    lw t5, 132(a0)
    lw t6, 136(a0)
    
    # Restore a0
    lw a0, 52(a0)
    
    # Return to the next instruction after the trap
    # in user mode
    sret
"#);

unsafe extern "C" {
  /// Accessible uservec symbol for Rust code
  pub fn uservec();
}
