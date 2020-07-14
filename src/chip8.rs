mod bus;
mod cpu;
mod display;
mod keypad;
mod memory;

use bus::Bus;
use cpu::CPU;
use std::vec::Vec;

pub struct Chip8 {
    cpu: CPU,
    bus: Bus,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: CPU::new(),
            bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, game_data: &Vec<u8>) {
        // The first 512 bytes are reserved for the interpreter
        // After that the ROM is loaded
        for idx in 0..game_data.len() {
            self.bus
                .write_byte_to_ram(cpu::PROGRAM_START + (idx as u16), game_data[idx]);
        }
    }

    // start has the main 'game loop' of the chip8
    pub fn start(&mut self) {
        loop {
            self.cpu.process_operation(&mut self.bus);
        }
    }
}
