pub struct RAM {
    pub memory: [u8; 4096],
}

impl RAM {
    pub fn new() -> RAM {
        let mut ram = RAM { memory: [0; 4096] };

        // The interpreter has a few hardcoded sprites
        // representing the range of 0 to F (hexadecimal)
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        // Put those characters in the ram
        let mut idx = 0;
        for sprite in sprites.iter() {
            for ch in sprite {
                ram.memory[idx] = *ch;
                idx += 1;
            }
        }

        ram
    }

    pub fn read_byte_from_ram(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte_to_ram(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
