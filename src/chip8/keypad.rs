pub struct Keypad {
    // There are 16 different keys on the keypad
    pub keypad: [u8; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keypad: [0; 16] }
    }

    pub fn press_key(&mut self, key_location: u16) {
        self.keypad = [0; 16];
        self.keypad[key_location as usize] = 1;
    }
}
