use super::bus::Bus;
use ggez;
use ggez::audio;
use ggez::audio::SoundSource;
use rand::{thread_rng, Rng};
use std::convert::TryInto;
use std::fmt;

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
    // The sound that plays when the dt runs out
    pub sound: Option<audio::Source>,
}

impl CPU {
    pub fn new(audio_file: Option<audio::Source>) -> CPU {
        CPU {
            gp: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            operand: 0,
            sound: audio_file,
        }
    }

    pub fn update_timers(&mut self) -> ggez::GameResult {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if let Some(ref mut sound) = self.sound {
                sound.play_later()?;
            }
            self.st -= 1;
        }

        Ok(())
    }

    pub fn process_operation(&mut self, bus: &mut Bus) {
        // Some cool bitwise manipulation to transform two u8s into a u16
        // see https://stackoverflow.com/questions/50243866/how-do-i-convert-two-u8-primitives-into-a-u16-primitive
        // TODO: This should be switched depending on endianness
        self.operand = ((bus.ram.read_byte_from_ram(self.pc) as u16) << 8)
            | bus.ram.read_byte_from_ram(self.pc + 1) as u16;

        // println!("Instruction: {:#X}", self.operand);

        // Extract common constructs using masks
        let nnn = self.operand & 0x0FFF;
        let kk = (self.operand & 0x00FF) as u8;
        let n = (self.operand & 0x000F) as u8;
        let x = ((self.operand & 0x0F00) >> 8) as usize;
        let y = ((self.operand & 0x00F0) >> 4) as usize;

        // println!("nnn: {:#X}", nnn);

        // Deconstruct the operation into nibbles
        let op_1 = (self.operand & 0xF000) >> 12;
        let op_2 = (self.operand & 0x0F00) >> 8;
        let op_3 = (self.operand & 0x00F0) >> 4;
        let op_4 = self.operand & 0x000F;

        // println!(
        //     "op1: {:#X},\nop2: {:#X},\nop3: {:#X},\nop4: {:#X}\n",
        //     op_1, op_2, op_3, op_4
        // );

        // Increment the counter
        self.pc += 2;

        // We match based on the instruction, we first take a look at the first part
        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear the screen
                bus.display.cls();
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
                if self.gp[x] == kk {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                // Increment the program counter by two 'words'
                // if Vx != kk
                if self.gp[x] != kk {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                // Increment the program counter by two 'words'
                // if Vx != Vy
                if self.gp[x] == self.gp[y] {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                // set the Vx register to kk
                self.gp[x] = kk;
            }
            (0x7, _, _, _) => {
                // add the Vx register and kk together
                self.gp[x] = self.gp[x].wrapping_add(kk);
            }
            (0x8, _, _, 0x0) => {
                // set the Vx register to Vy
                self.gp[x] = self.gp[y];
            }
            (0x8, _, _, 0x1) => {
                // OR the Vx and Vy
                self.gp[x] |= self.gp[y];
            }
            (0x8, _, _, 0x2) => {
                // AND the Vx and Vy
                self.gp[x] &= self.gp[y];
            }
            (0x8, _, _, 0x3) => {
                // XOR the Vx and Vy
                self.gp[x] ^= self.gp[y];
            }
            (0x8, _, _, 0x4) => {
                // Add the Vy register to Vx
                let new_value: u16 = self.gp[x] as u16 + self.gp[y] as u16;
                self.gp[0xF] = if new_value > 0xFF { 1 } else { 0 };
                // TODO: Test what is calculated here
                // It could also be done with "& 0xFF"
                self.gp[x] = (new_value & 0b1111_1111).try_into().unwrap_or_else(|e| {
                    panic!("failed to unwrap in 8__4: {:?}", e);
                });
            }
            (0x8, _, _, 0x5) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = if self.gp[x] > self.gp[y] { 1 } else { 0 };
                self.gp[x] = self.gp[x].wrapping_sub(self.gp[y]);
            }
            (0x8, _, _, 0x6) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = self.gp[x] & 0x1;
                self.gp[x] >>= 1;
            }
            (0x8, _, _, 0x7) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = if self.gp[x] < self.gp[y] { 1 } else { 0 };
                self.gp[x] = self.gp[y].wrapping_sub(self.gp[x]);
            }
            (0x8, _, _, 0xE) => {
                // Subtract the Vy from Vx
                self.gp[0xF] = self.gp[x] & 0x80;
                self.gp[x] <<= 1;
            }
            (0x9, _, _, 0x0) => {
                // Skip next instruction if Vx != Vy
                if self.gp[x] != self.gp[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                // Set I to nnn
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                // Jump to location nnn + V0
                self.pc = nnn + self.gp[0] as u16;
            }
            (0xC, _, _, _) => {
                // Set Vx = random byte AND kk
                self.gp[x] = thread_rng().gen_range(0, 255) & kk;
            }
            (0xD, _, _, _) => {
                // reset 0xF register
                self.gp[0xF] = 0;

                // Set a sprite in the graphics buffer
                // Get the sprite from memory
                let mut sprite = vec![0; (self.i..self.i + (n as u16)).len()];
                for idx in 0..sprite.len() {
                    sprite[idx] = bus.ram.read_byte_from_ram(self.i + (idx as u16));
                }

                // Draw it
                self.gp[0xF] =
                    if bus
                        .display
                        .draw(self.gp[x] as usize, self.gp[y] as usize, &mut sprite)
                    {
                        1
                    } else {
                        0
                    }
            }
            (0xE, _, 0x9, 0xE) => {
                // Check if key is down
                if bus.keypad.is_key_down(x) {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                // Check if key is up
                if !bus.keypad.is_key_down(x) {
                    self.pc += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => {
                // Set dt to Vx
                self.gp[x] = self.dt;
            }
            (0xF, _, 0x0, 0xA) => {
                // Check for key press
                // We keep performing the same action until a key is pressed
                self.pc -= 2;
                for (idx, key) in bus.keypad.keypad.iter().enumerate() {
                    if *key {
                        self.gp[x] = idx as u8;
                        self.pc += 2;
                    }
                }
            }
            (0xF, _, 0x1, 0x5) => {
                // Set dt to Vx
                self.dt = self.gp[x];
            }
            (0xF, _, 0x1, 0x8) => {
                // Set st to Vx
                self.st = self.gp[x];
            }
            (0xF, _, 0x1, 0xE) => {
                // Add I = I + Vx
                self.i += self.gp[x] as u16;
            }
            (0xF, _, 0x2, 0x9) => {
                // Set I = location of sprite (from Vx)
                // Sprites begin at 0x0
                self.i = (self.gp[x] as u16) * 5;
            }
            (0xF, _, 0x3, 0x3) => {
                // Store BCD representation of Vx in memory
                // at locations I+{0,1,2}
                bus.ram.write_byte_to_ram(self.i, self.gp[x] / 100);
                bus.ram
                    .write_byte_to_ram(self.i + 1, (self.gp[x] / 10) % 10);
                bus.ram.write_byte_to_ram(self.i + 2, self.gp[x] % 10);
            }
            (0xF, _, 0x5, 0x5) => {
                // Save V0 through Vx in memory starting at I
                for idx in 0..=x {
                    bus.ram
                        .write_byte_to_ram(self.i + (idx as u16), self.gp[idx]);
                }
            }
            (0xF, _, 0x6, 0x5) => {
                // Load V0 through Vx in memory starting at I
                for idx in 0..=x {
                    self.gp[idx] = bus.ram.read_byte_from_ram(self.i + (idx as u16));
                }
            }
            _ => panic!("unmatched instruction: {:#X}", self.operand),
        }
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\npc: {:#X}\n", self.pc)?;
        write!(f, "vx: ")?;
        for item in &self.gp {
            write!(f, "{:#X} ", *item)?;
        }
        write!(f, "\n")?;
        write!(f, "i: {:#X}\n", self.i)?;
        write!(f, "operand: {:#X}\n", self.operand)
    }
}
