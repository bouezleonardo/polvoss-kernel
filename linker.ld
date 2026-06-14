/* Specify the architecture*/
OUTPUT_ARCH("riscv")

/* Symbol of the entry point of the kernel */
ENTRY(_entry)

/* Specify the sections of the final kernel image
where the sections of the object files will be put */
SECTIONS 
{
  /* Start address offset */
  . = 0x80000000;
  
  PROVIDE(skernel = .); /* Save address where the kernel starts */
  
  /* Program code */
  .text :
  {
    *(.text.entry) /* First section */
    *(.text)
    *(.text.*)
    
    PROVIDE(etext = .); /* Save address where .text ends */
  }
  
  /* Read-only data i.e. constants*/
  .rodata :
  {
    *(.rodata)
    *(.rodata.*)
  }
  
  /* Read-write initialized data */
  .data :
  {
    *(.data)
    *(.data.*)
  }
  
  /* Read-write uninitialized or 
  zero initialized data */
  .bss :
  {
    *(COMMON)
    *(.bss)
    *(.bss.*)
  }
  
  PROVIDE(ekernel = .); /* Save address where kernel ends */
}
