pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = WIDTH * HEIGHT;
pub const PIXEL_SIZE: usize = 20;

pub struct Display {
    // Graphics gfx is 64 by 32 pixels
    // A pixel is white or black
    // We can put it in a one dimensional array
    pub gfx: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [0; WIDTH * HEIGHT],
        }
    }

    // Clear screen
    pub fn cls(&mut self) {
        self.gfx.iter_mut().for_each(|x| *x = 0);
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        self.gfx[x + y * WIDTH]
    }

    // Draw a sprite to the screen at the given coordinates
    pub fn draw(&mut self, x: usize, y: usize, sprite: &mut [u8]) -> bool {
        let mut has_collided = false;

        let line = y * WIDTH;
        let mut values = vec![0 as u8; 8];

        for i in 0..sprite.len() {
            // Each byte in a sprite draws on one line.
            let offset = line + WIDTH * i;

            // Organize the bits from the current sprite byte into a slice.
            for j in 0..values.len() {
                let bit = (sprite[i] >> j) & 0x01;
                values[8 - 1 - j] = bit;
            }

            // Loop through the bits in the current byte and set the gfx
            // values based on them.
            for j in 0..values.len() {
                let value = values[j];
                let pos: usize = x + j;
                let mut index: usize;

                // Draw a pixel in the sprite onto the gfx. If the pixel x
                // position is greater than the width of the gfx, the sprite
                // wraps around the gfx.
                if pos >= WIDTH {
                    // Wrap around to the left side to draw.
                    index = offset + pos - WIDTH;
                } else {
                    // Draw at the current offset.
                    index = offset + pos;
                }

                if index >= DISPLAY_SIZE {
                    index -= DISPLAY_SIZE;
                }

                if index < DISPLAY_SIZE {
                    // Save the previous state of the pixel before setting it
                    // for has_collided detection.
                    let prev = self.gfx[index];

                    // Draw the bit to the gfx.
                    self.gfx[index] = value ^ prev;

                    // Check the previous state of the pixel and check if it
                    // was erased, if so then there was a sprite has_collided.
                    if prev == 1 && self.gfx[index] == 0 {
                        has_collided = true;
                    }
                }
            }
        }

        has_collided
    }
}
