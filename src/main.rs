mod chip8;
use chip8::{Chip8, video::WIDTH, video::HEIGHT, video::SCALE};

use minifb::{Window, WindowOptions};

fn main() {
    // Creating a chip-8 emulator
    let mut chip: Chip8 = Chip8::new();

    // Path to a game
    let path = std::path::Path::new("games/ibm.ch8");

    // Load the game into memory
    chip.load(path);

    // A window to draw on
    let mut window = Window::new(
        "CHIP-8",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Continue while window is not closed
    while window.is_open() {
        // Execute one cycle
        chip.cycle();

        // Update the graphics
        window
            .update_with_buffer(&chip.video.gfx, WIDTH, HEIGHT)
            .unwrap();
    }
}
