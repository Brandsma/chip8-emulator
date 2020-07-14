use super::display::Display;
use super::keypad::Keypad;
use super::memory::RAM;

pub struct Bus {
    ram: RAM,
    display: Display,
    keypad: Keypad,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: RAM::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn cls(&mut self) {
        self.display.cls();
    }

    pub fn read_byte_from_ram(&self, address: u16) -> u8 {
        self.ram.read_byte_from_ram(address)
    }

    pub fn write_byte_to_ram(&mut self, address: u16, value: u8) {
        self.ram.write_byte_to_ram(address, value);
    }
}
