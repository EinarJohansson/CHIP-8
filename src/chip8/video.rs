use crate::{HEIGHT, WIDTH};

pub struct Video {
    pub gfx: Vec<u32>,
}

const PIXELS: usize = WIDTH * HEIGHT;

impl Video {
    pub fn new() -> Self {
        Self {
            gfx:  vec![0; PIXELS],
        }
    }

    pub fn clear_screen(&mut self) {
        self.gfx = vec![0; PIXELS];
    }

    pub fn xor(&mut self, index: usize) {
        self.gfx[index] ^= 255;
    }

    pub fn get_index(&self, x_coordinate: usize, y_coordinate: usize) -> usize {
        x_coordinate + 64 * y_coordinate
    }

    pub fn get_pixel(&self, index: usize) -> u8 {
        self.gfx[index] as u8
    }
}