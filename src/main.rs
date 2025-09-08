mod chip8;

use chip8::Chip8;
use std::fs::File;
use std::io::Read;

use std::env;

use ggez;
use ggez::audio;
use ggez::event;
use ggez::GameResult;

fn main() -> GameResult {
    // Get the game the player wants to play
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rom>", args[0]);
        eprintln!("Example: {} games/PONG.ch8", args[0]);
        std::process::exit(1);
    }
    
    let game = &args[1];
    println!("Loading game: {}", game);

    // Determine the dimensions of the window
    let width = chip8::display::PIXEL_SIZE as f32 * chip8::display::WIDTH as f32;
    let height = chip8::display::PIXEL_SIZE as f32 * chip8::display::HEIGHT as f32;

    let (mut ctx, event_loop) =
        ggez::ContextBuilder::new(&("CHIP-8 ".to_owned() + &args[1]), "Abe")
            .window_setup(
                ggez::conf::WindowSetup::default().title(&("CHIP-8: ".to_owned() + &args[1])),
            )
            .window_mode(ggez::conf::WindowMode::default().dimensions(width, height))
            .build()?;

    // Read the game into memory
    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening ROM file '{}': {}", args[1], e);
            std::process::exit(1);
        }
    };
    
    let mut rom_data = Vec::<u8>::new();
    if let Err(e) = file.read_to_end(&mut rom_data) {
        eprintln!("Error reading ROM file '{}': {}", args[1], e);
        std::process::exit(1);
    }

    // Get the audio file from the resources folder
    let audio_file = match audio::Source::new(&mut ctx, "/beep.wav") {
        Ok(audio) => {
            println!("Audio loaded successfully");
            Some(audio)
        },
        Err(e) => {
            eprintln!("Warning: Could not load audio file '/beep.wav': {}", e);
            eprintln!("Continuing without audio...");
            None
        }
    };

    // Initialize chip8 VM
    let mut chip8 = Chip8::new(audio_file);

    // Load the game into the RAM
    chip8.load_rom(&mut rom_data);

    // Start the chip8 machine
    event::run(ctx, event_loop, chip8)
}
