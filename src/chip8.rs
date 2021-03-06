mod bus;
mod cpu;
pub mod display;
mod keypad;
mod memory;

use ggez;
use ggez::audio;
use ggez::event;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::Rect;

use bus::Bus;
use cpu::CPU;

pub struct Chip8 {
    cpu: CPU,
    bus: Bus,
}

impl Chip8 {
    pub fn new(audio_file: audio::Source) -> Chip8 {
        Chip8 {
            cpu: CPU::new(audio_file),
            bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, game_data: &mut [u8]) {
        // The first 512 bytes are reserved for the interpreter
        // After that the ROM is loaded
        for idx in 0..game_data.len() {
            self.bus
                .ram
                .write_byte_to_ram(cpu::PROGRAM_START + (idx as u16), game_data[idx]);
        }
    }
}

impl event::EventHandler for Chip8 {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        // Every time run a cpu operation
        self.cpu.process_operation(&mut self.bus);

        // Update the timers and sound the beep when necessary
        self.cpu.update_timers()?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for col in 0..display::WIDTH {
            for row in 0..display::HEIGHT {
                if self.bus.display.get_pixel(col, row) == 1 {
                    let color = [0.0, 1.0, 0.3, 1.0].into();
                    let rect = Rect::new_i32(
                        col as i32 * display::PIXEL_SIZE,
                        row as i32 * display::PIXEL_SIZE,
                        display::PIXEL_SIZE,
                        display::PIXEL_SIZE,
                    );
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        color,
                    )?;
                    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.bus.keypad.press_key(get_key(keycode));
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, _keycode: KeyCode, _keymods: KeyMods) {
        self.bus.keypad.release_keys();
    }
}

fn get_key(key: KeyCode) -> u8 {
    match key {
        KeyCode::Key1 => 0x1,
        KeyCode::Key2 => 0x2,
        KeyCode::Key3 => 0x3,
        KeyCode::Key4 => 0xC,

        KeyCode::Q => 0x4,
        KeyCode::W => 0x5,
        KeyCode::E => 0x6,
        KeyCode::R => 0xD,

        KeyCode::A => 0x7,
        KeyCode::S => 0x8,
        KeyCode::D => 0x9,
        KeyCode::F => 0xE,

        KeyCode::Z => 0xA,
        KeyCode::X => 0x0,
        KeyCode::C => 0xB,
        KeyCode::V => 0xF,
        _ => 0xF,
    }
}
