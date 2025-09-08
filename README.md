# CHIP-8 Emulator

A simple and functional CHIP-8 emulator written in Rust using the ggez game framework.

## What is CHIP-8?

[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) is a simple virtual machine designed in the mid-1970s for running games on 8-bit microcomputers. It features:
- 64x32 pixel monochrome display
- 16 general-purpose registers
- 4KB of RAM
- Simple instruction set perfect for classic games like Pong, Breakout, and Tetris

## Features

- Full CHIP-8 instruction set implementation
- Graphics rendering with scalable display
- Audio support 
- Keyboard input mapping
- Loads standard .ch8 ROM files

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd chip8-emulator
```

Run the emulator with a ROM file:

```bash
cargo run games/PONG.ch8
```

Or with the release build:
```bash
cargo run --release games/BREAKOUT.ch8
```

### Available Games

The `games/` folder contains various CHIP-8 ROMs including:
- `PONG.ch8` - Classic Pong game
- `BREAKOUT.ch8` - Breakout/Brick Breaker
- `TETRIS.ch8` - Tetris variant
- `15PUZZLE.ch8` - Sliding puzzle game
- And many more!

### Controls

Game controls vary by ROM, but typically use:
- Numbers 1-4 for player controls
- QWER keys for directional input
- Consult individual game instructions for specific controls

## Troubleshooting

**Build issues on ARM64/Apple Silicon:**
- Currently mostly tested for x86_64 systems
- Some audio dependencies as well as libclang may have compatibility issues on ARM64

**No audio:**
The only audio is a simple beep sound.
The emulator automatically copies resources during build. If issues persist:
```bash
cp -r resources target/debug/
```

## Architecture

- `src/main.rs` - Entry point and window management
- `src/chip8.rs` - Main emulator coordination
- `src/chip8/cpu.rs` - CPU implementation and instruction processing
- `src/chip8/memory.rs` - RAM management
- `src/chip8/display.rs` - Graphics rendering
- `src/chip8/keypad.rs` - Input handling

## Resources & References

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [How to Write an Emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

## License

This project is open source. See individual ROM files for their respective licenses.
