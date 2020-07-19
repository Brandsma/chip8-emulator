mod chip8;

use chip8::Chip8;
use std::fs::File;
use std::io::Read;

use std::env;

use ggez;
use ggez::audio;
use ggez::event;

fn main() -> ggez::GameResult {
    // Get the game the player wants to play
    let args: Vec<String> = env::args().collect();
    let game = &args[1];
    println!("{}", game);

    // Determine the dimensions of the window
    let width = chip8::display::PIXEL_SIZE as f32 * chip8::display::WIDTH as f32;
    let height = chip8::display::PIXEL_SIZE as f32 * chip8::display::HEIGHT as f32;

    let (ctx, event_loop) =
        &mut ggez::ContextBuilder::new(&("CHIP-8 ".to_owned() + &args[1]), "Abe")
            .window_setup(
                ggez::conf::WindowSetup::default().title(&("CHIP-8: ".to_owned() + &args[1])),
            )
            .window_mode(ggez::conf::WindowMode::default().dimensions(width, height))
            .build()?;

    // Read the game into memory
    // TODO: Remove hardcoded file path
    let mut file = File::open(&args[1]).unwrap();
    let mut rom_data = Vec::<u8>::new();
    assert!(file.read_to_end(&mut rom_data).is_ok());

    let audio_file = audio::Source::new(ctx, "/beep.ogg");

    // Initialize chip8 VM
    let mut chip8 = Chip8::new(audio_file.unwrap_or_else(|e| {
        panic!(
            "Something went wrong with unwrapping the audiofile: {:?}",
            e
        );
    }));

    // Load the game into the RAM
    chip8.load_rom(&mut rom_data);

    // Start the chip8 machine
    event::run(ctx, event_loop, &mut chip8)
}
