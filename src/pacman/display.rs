use std::fmt;

use minifb::{Scale, Window, WindowOptions};

pub const WIDTH: u32 = 224;
pub const HEIGHT: u32 = 288;

pub struct Display {
    pub raster: Vec<u32>,
    pub buf: Vec<u8>,
    pub tile_rom: Vec<u8>,
    pub sprite_rom: Vec<u8>,
    pub color_rom: Vec<u8>,
    pub palette_rom: Vec<u8>,
    pub vram: Vec<u8>,
    pub vblank: bool,
    pub window: Window,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

pub enum Intensity {
    Min,
    Max,
    Normal,
}

// As described per: https://www.lomont.org/software/games/pacman/PacmanEmulation.pdf
impl Color {
    fn get(c: Color, i: Intensity) -> u8 {
        match c {
            Color::Red | Color::Green => match i {
                Intensity::Min => 0x21,
                Intensity::Max => 0x97,
                Intensity::Normal => 0x47,
            },
            Color::Blue => match i {
                Intensity::Min => 0x51,
                Intensity::Max => 0xAE,
                Intensity::Normal => 0x51,
            },
        }
    }
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
            buf: vec![0; WIDTH as usize * HEIGHT as usize * SCALE],
            tile_rom: vec![0; 0x2048],
            sprite_rom: vec![0; 0x1024],
            color_rom: vec![0; 0x1024],
            palette_rom: vec![0; 0x1024],
            vram: vec![0; WIDTH as usize * HEIGHT as usize * SCALE],
            vblank: false,
            window,
        }
    }

    // Converts 8 bit RGB into RGBA 32 bit
    fn as_u32_le(&self, array: &Vec<u8>) -> u32 {
        ((array[0] as u32) << 24)
            | ((array[1] as u32) << 16)
            | ((array[2] as u32) << 8)
            | ((array[3] as u32) << 0)
    }

    // Each tile is 8 by 8 pixels stored as 2bpp, i.e 16 bits per tile.
    // Drawing a tile uses a byte in tile ram to select one of the 256 - 4 color tiles to display.
    // It also uses a byte in palette ram to select the palette to be used to color the tile.
    // Each byte stores 4 pixels, the least significant bit of each pixel is stored in
    // the lower four bits of the byte, and the most significant bit of each pixel is stored
    // in the higher 4 bits of the byte.
    //
    // Byte layout of tile rom
    // Bit  Description
    // 0    bit 0 pixel 1
    // 1    bit 0 pixel 2
    // 2    bit 0 pixel 3
    // 3    bit 0 pixel 4
    // 4    bit 1 pixel 1
    // 5    bit 1 pixel 2
    // 6    bit 1 pixel 3
    // 7    bit 1 pixel 4
    // Color data is stored at $4400 & up.

    // Fetches one tile out of tile rom
    pub fn draw_tile(&mut self, tile: u8) {
        for byte_number in 0..16 {
            // Make sure X wraps around to 0 for drawing the second line of tile strips
            let mut x: u8 = (byte_number % 8);
            let mut y: u8 = if byte_number >= 8 { 4 } else { 0 };
            let byte = self.tile_rom[tile as usize * 16 + byte_number as usize];
            let strip = self.decode_vertical_strip(byte);

            for pixel_number in 0..4 {
                self.draw_pixel(strip[pixel_number], x, y);
                y = y + 1;
            }
        }
    }

    // Takes a byte of tile data and returns one vertical strip (4 pixels)
    pub fn decode_vertical_strip(&self, byte: u8) -> Vec<u8> {
        let pixel1 = (byte & 1) | (((byte >> 3) & 1) << 1);
        let pixel2 = ((byte >> 1) & 1) | ((byte >> 4) & 1);
        let pixel3 = ((byte >> 2) & 1) | ((byte >> 5) & 1);
        let pixel4 = ((byte >> 3) & 1) | ((byte >> 6) & 1);

        vec![pixel1, pixel2, pixel3, pixel4]
    }

    pub fn draw_pixel(&mut self, color: u8, x: u8, y: u8) {
        if color == 0 {
            self.raster[WIDTH as usize * y as usize + x as usize] = 0xFF00_0000;
        } else {
            self.raster[WIDTH as usize * y as usize + x as usize] = 0xFFFF_FFFF;
        }
    }

    /*pub fn draw_pixel(&mut self) {
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
    }*/
}
