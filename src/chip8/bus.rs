use super::display::Display;
use super::keypad::Keypad;
use super::memory::RAM;

pub struct Bus {
    pub ram: RAM,
    pub display: Display,
    pub keypad: Keypad,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: RAM::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }
}
