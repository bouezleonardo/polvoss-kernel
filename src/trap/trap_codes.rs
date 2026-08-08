/// Exception codes
pub const INSTRUCTION_ADDRESS_MISALIGNED: usize = 0;
pub const INSTRUCTION_ACCESS_FAULT: usize = 1;
pub const ILLEGAL_INSTRUCTION: usize = 2;
pub const BREAKPOINT: usize = 3;
pub const LOAD_ADDRESS_MISALIGNED: usize = 4;
pub const LOAD_ACCESS_FAULT: usize = 5;
pub const STORE_AMO_ADDRESS_MISALIGNED: usize = 6;
pub const STORE_AMO_ACCESS_FAULT: usize = 7;
pub const ENVIRONMENT_CALL_FROM_U_MODE: usize = 8;
pub const ENVIRONMENT_CALL_FROM_S_MODE: usize = 9;
pub const INSTRUCTION_PAGE_FAULT: usize = 12;
pub const LOAD_PAGE_FAULT: usize = 13;
pub const STORE_AMO_PAGE_FAULT: usize = 15;
pub const SOFTWARE_CHECK: usize = 18;
pub const HARDWARE_ERROR: usize = 19;

pub const fn desc_exception(code: usize) -> &'static str {
  match code {
    INSTRUCTION_ADDRESS_MISALIGNED => "Instruction address misaligned",
    INSTRUCTION_ACCESS_FAULT => "Instruction access fault",
    ILLEGAL_INSTRUCTION => "Illegal instruction",
    BREAKPOINT => "Breakpoint",
    LOAD_ADDRESS_MISALIGNED => "Load address misaligned",
    LOAD_ACCESS_FAULT => "Load access fault",
    STORE_AMO_ADDRESS_MISALIGNED => "Store/AMO address misaligned",
    STORE_AMO_ACCESS_FAULT => "Store/AMO access fault",
    ENVIRONMENT_CALL_FROM_U_MODE => "Environment call from U-mode",
    ENVIRONMENT_CALL_FROM_S_MODE => "Environment call from S-mode",
    INSTRUCTION_PAGE_FAULT => "Instruction page fault",
    LOAD_PAGE_FAULT => "Load page fault",
    STORE_AMO_PAGE_FAULT => "Store/AMO page fault",
    SOFTWARE_CHECK => "Software check",
    HARDWARE_ERROR => "Hardware error",
    _ => "Unknown exception",
  }
}

/// Supervisor Interrupt codes
pub const SOFTWARE_INT: usize = 1;
pub const TIMER_INT: usize = 5;
pub const EXTERNAL_INT: usize = 9;
pub const COUNTER_OVERFLOW_INT: usize = 13;

pub const fn desc_interrupt(code: usize) -> &'static str {
  match code {
    SOFTWARE_INT => "Supervisor software interrupt",
    TIMER_INT => "Supervisor timer interrupt",
    EXTERNAL_INT => "Supervisor external interrupt",
    COUNTER_OVERFLOW_INT => "Counter overflow interrupt",
    _ => "Unknown interrupt",
  }
}
