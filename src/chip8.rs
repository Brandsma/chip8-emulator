mod cpu;
mod display;
mod keypad;
mod memory;

use cpu::CPU;
use display::Display;
use keypad::Keypad;
use memory::RAM;

use std::vec::Vec;

pub struct Chip8 {
    cpu: CPU,
    ram: RAM,
    display: Display,
    keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: CPU::new(),
            ram: RAM::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn load_rom(&mut self, game_data: &Vec<u8>) {
        // The first 512 bytes are reserved for the interpreter
        // After that the ROM is loaded
        let offset = 0x200;

        for idx in 0..game_data.len() {
            self.ram.write_byte((offset + idx) as u16, game_data[idx]);
        }
    }
}
