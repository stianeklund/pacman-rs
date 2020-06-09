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
        return match c {
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
        };
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
            tile_rom: vec![0; 0x1024],
            sprite_rom: vec![0; 0x1024],
            color_rom: vec![0; 0x1024],
            palette_rom: vec![0; 0x1024],
            vram: vec![0; WIDTH as usize * HEIGHT as usize * SCALE],
            vblank: false,
            window,
        }
    }

    fn u8_rgb(&self, r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn draw_tile(&mut self, tile: u8, x: u8, y: u8) {
        for byte_number in 0..16 {
            // X wraps around to 0 (at 7) for drawing the second half of the tile
            let x = x + (byte_number % 8);
            let mut y: u8 = if byte_number >= 8 { y + 4 } else { y + 0 };
            // 16 & byte number internal tile coordinates?
            let byte = self.tile_rom[tile as usize * 16 + byte_number as usize];
            let strip = self.decode_vertical_strip(byte);

            for pixel_number in 0..4 {
                self.draw_pixel(strip[pixel_number], x, y);
                y = y + 1
            }
        }
    }
    pub fn draw_sprite(&mut self, sprite: u8, x: u8, y: u8) {
        for byte_number in 0..64 {
            // X wraps around to 0 (at 7) for drawing the second half of the tile
            let x = x + (byte_number % 8);
            let mut y: u8 = if byte_number >= 8 { y + 4 } else { y + 0 };
            // 16 & byte number internal tile coordinates?
            let byte = self.sprite_rom[sprite as usize * 16 + byte_number as usize];
            let strip = self.decode_vertical_strip(byte);
            for pixel_number in 0..4 {
                self.draw_pixel(strip[pixel_number], x, y);
                y = y + 1
            }
        }
    }

    // Takes a byte of tile data and returns one vertical strip (4 pixels)
    pub fn decode_vertical_strip(&self, byte: u8) -> Vec<u8> {
        let pixel1 = ((byte >> 0) & 1) | (((byte >> 4) & 1) << 1);
        let pixel2 = ((byte >> 1) & 1) | (((byte >> 5) & 1) << 1);
        let pixel3 = ((byte >> 2) & 1) | (((byte >> 6) & 1) << 1);
        let pixel4 = ((byte >> 3) & 1) | (((byte >> 7) & 1) << 1);

        vec![pixel1, pixel2, pixel3, pixel4]
    }

    pub fn draw_pixel(&mut self, pixel_number: u8, x: u8, y: u8) {
        // Pixel number being the 2bbp value from our vertical strip

        let color = self.get_palette(1, pixel_number);
        let (r, g, b) = self.get_color(color);
        let v = self.u8_rgb(r, g, b);
        self.raster[WIDTH as usize * y as usize + x as usize] = v;
    }

    fn get_palette(&self, palette_number: u8, color_index: u8) -> u8 {
        self.palette_rom[palette_number as usize * 4 + color_index as usize]
    }

    // Gets the RGB color from color intensity values in color rom
    pub fn get_color(&self, color_number: u8) -> (u8, u8, u8) {
        use Color::*;
        use Intensity::*;
        let color = self.color_rom[color_number as usize];

        // Bit 0, 1, 2: Red
        let r = ((color >> 0) & 1) * Color::get(Red, Min)
            + ((color >> 1) & 1) * Color::get(Red, Normal)
            + ((color >> 2) & 1) * Color::get(Red, Max);
        // Bit 3, 4, 5: Green
        let g = ((color >> 3) & 1) * Color::get(Green, Min)
            + ((color >> 4) & 1) * Color::get(Green, Normal)
            + ((color >> 5) & 1) * Color::get(Green, Max);
        // Bit 6, 7: Blue
        let b =
            ((color >> 6) & 1) * Color::get(Blue, Min) + ((color >> 7) & 1) * Color::get(Blue, Max);

        (r, g, b)
    }
}
