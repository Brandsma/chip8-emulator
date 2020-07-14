use super::bus::Bus;
use rand::{thread_rng, Rng};

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

    pub fn process_operation(&mut self, bus: &mut Bus) {
        // Some cool bitwise manipulation to transform two u8s into a u16
        // see https://stackoverflow.com/questions/50243866/how-do-i-convert-two-u8-primitives-into-a-u16-primitive
        // TODO: This should be switched depending on endianness
        self.operand = ((bus.read_byte_from_ram(self.pc) as u16) << 8)
            | bus.read_byte_from_ram(self.pc + 1) as u16;

        println!("Instruction: {:#X}", self.operand);

        // Extract common constructs using masks
        let nnn = self.operand & 0x0FFF;
        let kk = (self.operand & 0x00FF) as u8;
        let n = (self.operand & 0x000F) as u8;
        let x = ((self.operand & 0x0F00) >> 8) as usize;
        let y = ((self.operand & 0x00F0) >> 4) as usize;

        println!("nnn: {:#X}", nnn);

        // Deconstruct the operation into nibbles
        let op_1 = (self.operand & 0xF000) >> 12;
        let op_2 = (self.operand & 0x0F00) >> 8;
        let op_3 = (self.operand & 0x00F0) >> 4;
        let op_4 = self.operand & 0x000F;

        println!(
            "op1: {:#X},\nop2: {:#X},\nop3: {:#X},\nop4: {:#X}\n",
            op_1, op_2, op_3, op_4
        );

        // Increment the counter
        self.pc += 2;

        // We match based on the instruction, we first take a look at the first part
        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear the screen
                bus.cls();
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // Return from a subroutine
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            (0x1, _, _, _) => {
                // Jump to location nnn
                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                // Call subroutine at nnn
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                if self.sp > 15 {
                    panic!(
                        "stack pointer set too high after operation: {:#X}",
                        self.operand
                    );
                }
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                // Increment the program counter by two 'words'
                // if Vx == kk
                if (self.gp[x] == kk) {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                // Increment the program counter by two 'words'
                // if Vx != kk
                if (self.gp[x] != kk) {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                // Increment the program counter by two 'words'
                // if Vx != Vy
                if (self.gp[x] == self.gp[y]) {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                // set the Vx register to kk
                self.gp[x] = kk;
            }
            (0x7, _, _, _) => {
                // add the Vx register and kk together
                self.gp[x] += kk;
            }
            (0x8, _, _, 0x0) => {
                // set the Vx register to Vy
                self.gp[x] = self.gp[y];
            }
            (0x8, _, _, 0x1) => {
                // OR the Vx and Vy
                self.gp[x] = self.gp[x] | self.gp[y];
            }
            (0x8, _, _, 0x2) => {
                // AND the Vx and Vy
                self.gp[x] = self.gp[x] & self.gp[y];
            }
            (0x8, _, _, 0x3) => {
                // XOR the Vx and Vy
                self.gp[x] = self.gp[x] ^ self.gp[y];
            }
            (0x8, _, _, 0x4) => {
                // Add the Vy register to Vx
                let new_value = self.gp[x] + self.gp[y];
                self.gp[0xF] = if (new_value > 0xFF) { 1 } else { 0 };
                // TODO: Test what is calculated here
                // It could also be done with "& 0xFF"
                self.gp[x] = new_value & 0b1111_1111;
            }
            (0x8, _, _, 0x5) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = if self.gp[x] > self.gp[y] { 1 } else { 0 };
                self.gp[x] -= self.gp[y];
            }
            (0x8, _, _, 0x6) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = self.gp[x] & 0x1;
                self.gp[x] >>= 1;
            }
            (0x8, _, _, 0x7) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = if self.gp[x] < self.gp[y] { 1 } else { 0 };
                self.gp[x] = self.gp[y] - self.gp[x];
            }
            (0x8, _, _, 0xE) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = self.gp[x] & 0x80;
                self.gp[x] <<= 1;
            }
            (0x9, _, _, 0x0) => {
                if (self.gp[x] != self.gp[y]) {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = nnn + self.gp[0] as u16;
            }
            (0xB, _, _, _) => self.gp[x] = thread_rng().gen_range(0, 255) & kk,
            _ => panic!("unmatched instruction: {:#X}", self.operand),
        }
    }
}
