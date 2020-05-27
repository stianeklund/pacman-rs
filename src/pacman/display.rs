use std::fmt;

use minifb::{Scale, Window, WindowOptions};

use crate::memory::Memory;

pub const WIDTH: u32 = 224;
pub const HEIGHT: u32 = 288;

pub struct Display {
    pub raster: Vec<u32>,
    pub vblank: bool,
    pub window: Window,
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:?}", val)
    }
}

impl fmt::UpperHex for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:02X}", val)
    }
}

const SCALE: usize = 8;

impl Display {
    pub fn new() -> Self {
        let mut window = Window::new(
            "Pacman-rs",
            WIDTH as usize,
            HEIGHT as usize,
            WindowOptions {
                resize: true,
                scale: Scale::X4,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        window.set_position(400, 400);
        Display {
            // TODO: Is there a better way to handle resize / different scaling?
            raster: vec![0x00FF_FFFF; WIDTH as usize * HEIGHT as usize * SCALE],
            vblank: false,
            window,
        }
    }

    pub fn draw_pixel(&mut self, memory: &Memory) {
        let memory = &memory.ram;

        // Iterate over VRAM
        for (i, byte) in (memory[0x04000..0x4FFF]).iter().enumerate() {
            let y = i as isize * 8 / 224;

            for shift in 0..(7 + 1) {
                let x = ((i * 8) % 224 as usize + shift as usize) as isize;

                let pixel = if byte.wrapping_shr(shift) & 1 == 0 {
                    0xFF00_0000 // Alpha
                } else {
                    0xFFFF_FFFF // Black
                };
                self.raster[WIDTH as usize * x as usize + y as usize] = pixel;
            }
        }
    }
}
