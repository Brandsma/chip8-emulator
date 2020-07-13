use super::memory::RAM;

// The program will always start at 0x200 (512)
// Since the rest is reserved for the interpreter
pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    // General purpose registers
    pub gp: [u8; 16],
    // Program counter register
    pub pc: u16,
    // index register
    pub i: u16,

    // Stack
    pub stack: [u16; 16],
    // Stack pointer register
    pub sp: u16,

    // Delay timer register
    pub dt: u8,
    // Sound timer register
    pub st: u8,

    // The current operation: operand
    pub operand: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            gp: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            operand: 0,
        }
    }

    pub fn process_operation(&mut self, memory: &mut RAM) {}
}
