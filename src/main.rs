mod chip8;

use chip8::Chip8;
use std::fs::File;
use std::io::Read;

fn main() {
    // Read the game into memory
    // TODO: Remove hardcoded file path
    let mut file = File::open("games/INVADERS.ch8").unwrap();
    let mut rom_data = Vec::<u8>::new();
    assert!(file.read_to_end(&mut rom_data).is_ok());

    // Initialize chip8 VM
    let mut chip8 = Chip8::new();

    // Load the game into the RAM
    chip8.load_rom(&rom_data);

    /* Main Game Loop */
    loop {
        chip8.run_operation();
    }
}
