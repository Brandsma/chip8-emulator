pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const PIXEL_SIZE: i32 = 20;

pub struct Display {
    // Graphics display is 64 by 32 pixels
    // A pixel is white or black
    // We can put it in a one dimensional array
    pub gfx: [u8; (WIDTH * HEIGHT)],
}

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [0; (WIDTH * HEIGHT)],
        }
    }

    // Clear screen
    pub fn cls(&mut self) {
        self.gfx.iter_mut().for_each(|x| *x = 0);
    }

    pub fn get_display_buffer(&mut self) -> [u8; (WIDTH * HEIGHT)] {
        self.gfx
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        self.gfx[x + y * WIDTH]
    }

    fn set_pixel(&mut self, x: usize, y: usize, val: u8) {
        self.gfx[x + y * WIDTH] = val as u8;
    }

    // Draw a sprite to the screen at the given coordinates
    pub fn draw(&mut self, x: usize, y: usize, sprite: &mut [u8]) -> bool {
        let mut has_collided = false;

        // Go over all the rows of the sprite
        for x_idx in 0..sprite.len() {
            let cur_row = sprite[x_idx];
            // Each row has 8 bits worth of data
            for y_idx in 0..8 {
                // Iterate over each bit in the row and mask
                // that specific bit out
                let new_sprite_value = cur_row >> (7 - y_idx) & 0x01;

                // Only change things if the new sprite value is 1
                if new_sprite_value != 1 {
                    continue;
                }

                // Get the location of this sprite bit in memory
                let x_location = (x + x_idx) % WIDTH;
                let y_location = (y + y_idx) % HEIGHT;

                // Check if that pixel is currently filled
                let old_sprite_value = self.get_pixel(x_location, y_location);
                has_collided = old_sprite_value == 1;

                // Set the pixel and XOR it with the current value
                self.set_pixel(x_location, y_location, new_sprite_value ^ old_sprite_value);
            }
        }

        has_collided
    }
}
