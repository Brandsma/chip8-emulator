pub struct Display {
    // Graphics display is 64 by 32 pixels
    // A pixel is white or black
    // We can put it in a one dimensional array
    pub gfx: [u8; (64 * 32)],
}

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [0; (64 * 32)],
        }
    }
}
