pub struct Keypad {
    // There are 16 different keys on the keypad
    pub keypad: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keypad: [false; 16],
        }
    }

    pub fn press_key(&mut self, key: u8) {
        self.keypad[key as usize] = true;
    }

    pub fn release_keys(&mut self) {
        self.keypad.iter_mut().for_each(|x| *x = false);
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.keypad[key]
    }
}
