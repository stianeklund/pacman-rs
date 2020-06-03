use std::fmt;

use minifb::{Scale, Window, WindowOptions};

pub const WIDTH: u32 = 224;
pub const HEIGHT: u32 = 288;

pub struct Display {
    pub raster: Vec<u32>,
    pub tile_rom: Vec<u8>,
    pub sprite_rom: Vec<u8>,
    pub color_rom: Vec<u8>,
    pub palette_rom: Vec<u8>,
    pub vram: Vec<u8>,
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
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        window.set_position(400, 400);
        Display {
            // TODO: Is there a better way to handle resize / different scaling?
            raster: vec![0x00FF_FFFF; WIDTH as usize * HEIGHT as usize * SCALE],
            tile_rom: vec![0; 0x1000],
            sprite_rom: vec![0; 0x1000],
            color_rom: vec![0; 0x1000],
            palette_rom: vec![0; 0x1000],
            vram: vec![0; WIDTH as usize * HEIGHT as usize * SCALE],
            vblank: false,
            window,
        }
    }

    pub fn draw_tile(&mut self) {
        // Each tile is 8 by 8 pixels stored as 2bpp, i.e 16 bits per tile.
        // Whole rom holds 256 tiles.
        // Tile ram location: 0x4000..=0x43FFF
        let tiles = vec![258 * 8 * 8];
        for (i, byte) in self.tile_rom.iter().enumerate() {}
    }

    fn draw_palette(&mut self) {
        for (i, byte) in self.palette_rom.iter().enumerate() {
            println!("{}, {}", i, byte);
        }
    }

    pub fn draw_sprite(&mut self) {
        // pacman.5f stores 16x16 sprites as 2bpp. 64 sprites stored in rom.
        // Each byte represents 4 pixels in a columned (stored in the same way as tiles).
        // Each 8 bytes draw a strp of 8x4 pixels.
        // Palette ram location: 0x4000..=0x47FF
        let sprites = vec![64 * 16 * 16];

        for (i, byte) in self.sprite_rom.iter().enumerate() {
            for x in 0..4 {}
        }
    }

    pub fn draw_pixel(&mut self) {
        // Iterate over VRAM
        for (i, byte) in (self.vram).iter().enumerate() {
            let y = i as isize * 8 / HEIGHT as isize;

            for shift in 0..(7 + 1) {
                let x = ((i * 8) % HEIGHT as usize + shift as usize) as isize;

                let pixel = if byte.wrapping_shr(shift) & 1 == 0 {
                    0xFF00_0000 // Alpha
                } else {
                    0xFFFF_FFFF // Black
                };
                self.raster[(WIDTH as usize * x as usize + y as usize)] = pixel;
            }
        }
    }
}
