pub struct CPU {
    // Memory
    pub memory: [u8; 4096],

    // General purpose registers
    pub gp: [u8; 16],
    // index register
    pub i: u16,
    // Program counter register
    pub pc: u16,

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
            memory: [0; 4096],
            gp: [0; 16],
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            operand: 0,
        }
    }
}
