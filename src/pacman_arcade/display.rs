use std::{fmt, thread};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;
use std::time::Duration;

use crate::minifb::{Scale, Window, WindowOptions};
use crate::pacman_arcade::pacman::Mapper;

pub const WIDTH: usize = 224;
pub const HEIGHT: usize = 256;

impl Mapper for Display {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x4000..=0x43FF => self.vram[addr as usize - 0x4000],
            0x4400..=0x47FF => self.vram[addr as usize - 0x4400],
            0x4800..=0x4BFF => self.vram[addr as usize - 0x4800],
            _ => unimplemented!("Read to address:{:04X}", addr),
        }
    }

    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x4000..=0x43FF => self.vram[addr as usize - 0x4000] = byte,
            0x4400..=0x47FF => self.vram[addr as usize - 0x4400] = byte,
            0x4800..=0x4FEF => self.vram[addr as usize - 0x4800] = byte,
            _ => unimplemented!(
                "{}",
                format!("Write address:{:02X} Byte:{:02X}", addr, byte)
            ),
        }
    }
}

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

impl Index<u16> for Display {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        &self.vram[index as usize]
    }
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
            raster: vec![0x00FF_FFFF; (WIDTH as usize * HEIGHT as usize) * SCALE],
            buf: vec![0; WIDTH as usize * HEIGHT as usize],
            tile_rom: vec![0; 0x1024],
            sprite_rom: vec![0; 0x90024],
            color_rom: vec![0; 0x1024],
            palette_rom: vec![0; 0x1024],
            vram: vec![0; WIDTH as usize * HEIGHT as usize * SCALE],
            vblank: false,
            window,
        }
    }

    // Converts RGB into 0RGB where the upper 8 bits are ignored
    // Needed for minifb as it takes a 32bit pixel buffer(0RBG format)
    fn u8_rgb(&self, r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn draw_tile(&mut self, tile_number: u8, x: u8, y: u8, pal_no: u8) {
        // 16 bytes per tile
        for byte_number in 0..16 {
            // X wraps around to 0 (at 7) for drawing the second half of the tile
            // let mut x = x + 7 - (byte_number % 8);
            let mut x = x.wrapping_add(7 - (byte_number % 8));
            let mut y = if byte_number >= 8 {
                y.wrapping_add(0)
            } else {
                y.wrapping_add(4)
            };
            let mut byte = self.tile_rom[tile_number as usize * 16 + byte_number as usize];
            let mut strip: Vec<u8> = self.decode_vertical_strip(byte);
            for pixel_number in 0..4 {
                self.draw_pixel(strip[3 - pixel_number], x, y, pal_no as usize);
                y = y.wrapping_add(1);
            }
        }
    }
    // Draws a sprite slice ( 8 columns, each column is 4  pixels tall)
    pub fn draw_slice(&mut self, x: usize, mut y: usize, mut index: usize, pal: usize) {
        let sprites = self.create_slice(index);

        for byte_number in 0..8 {
            // X wraps around to 0 (at 7) for drawing the second half of the tile
            let mut x = x.wrapping_add(7 - (byte_number % 8));
            let mut y = if byte_number >= 8 {
                y.wrapping_add(0)
            } else {
                y.wrapping_add(4)
            };
            for strip_no in 0..4 {
                // not sure about reversing the slice bytes here??
                // Reverse what is stored otherwise it's flipped horizontally
                // self.draw_pixel(sprites[&(7 -byte_number as u8)][3 - strip_no], x as u8, y as u8, 1 as usize);
                self.draw_pixel(sprites[&(7 -byte_number as u8)][3 - strip_no], x as u8, y as u8, 1 as usize);
                y = y.wrapping_add(1);
            }
        }
    }
    pub fn create_slice(&mut self, mut index: usize) -> HashMap<u8, Vec<u8>> {
        let mut hash: HashMap<u8, Vec<u8>> = HashMap::new();
        // Sprite[index] --> [sprite_byte[pixel_data]
        for strip in 0..8 {
            // A 8x4 pixel strip is 8 bytes long (which is why we need to index * 8 + strip offset)
            let sprite_byte = self.sprite_rom[index * 8 + strip as usize];
            hash.insert(strip, self.decode_vertical_strip(sprite_byte));
        }
        return hash;
    }

    pub fn draw_sprite(&mut self, x: usize, mut y: usize, mut sprite: usize, offset: usize, pal_no: usize) {
        /*  Sprite arrangement:
        Stored as 2bbp
        ---------------------------------
        *  | 5 | 1 | * (strip0, strip1)
        *  | 6 | 2 | * (strip2, strip3)
        *  | 7 | 3 | * (strip4, strip5)
        *  | 4 | 0 | * (strip6, strip7)
        ----------------------------- */


        self.draw_slice(x, y + 0, (sprite + offset) + 0 * 8, pal_no);
        self.draw_slice(x, y + 4, (sprite + offset) + 1 * 8, pal_no);
        self.draw_slice(x, y + 8, (sprite + offset) + 2 * 8, pal_no);
        self.draw_slice(x, y + 12, (sprite + offset) + 3 * 8, pal_no);

        self.draw_slice(x + 8, y + 0, (sprite + offset) + 4 * 8, pal_no);
        self.draw_slice(x + 8, y + 4, (sprite + offset) + 5 * 8, pal_no);
        self.draw_slice(x + 8, y + 8, (sprite + offset) + 6 * 8, pal_no);
        self.draw_slice(x + 8, y + 12, (sprite + offset) + 7 * 8, pal_no);

        // first bitplane
        //self.draw_slice(x + 8, y + 4, (sprite + offset) + 64, pal_no);
        //self.draw_slice(x + 8, y + 8, (sprite + offset) + 96, pal_no);
        //self.draw_slice(x + 8, y + 12, (sprite + offset), pal_no);
        //self.draw_slice(x + 8, y * 8, (sprite + offset) + 32, pal_no);

        //// // Second bit plane
        //self.draw_slice(x, y, (sprite + offset) + 160, pal_no);
        //self.draw_slice(x, y + 4, (sprite + offset)+ 192, pal_no);
        //self.draw_slice(x, y + 8, (sprite + offset) + 224, pal_no);
        //self.draw_slice(x, y + 12, (sprite + offset) + 128, pal_no);

    }

    // each byte of tile data stores 4 pixels on a bit plane
    // the respective pixels are composed of bits:  0, 4, 1,5, 2, 6, 3 and 7
    // pixel1: bit 0 and bit 4.
    pub fn decode_vertical_strip(&self, byte: u8) -> Vec<u8> {
        let pixel1 = ((byte >> 0) & 1) | (((byte >> 4) & 1) << 1);
        let pixel2 = ((byte >> 1) & 1) | (((byte >> 5) & 1) << 1);
        let pixel3 = ((byte >> 2) & 1) | (((byte >> 6) & 1) << 1);
        let pixel4 = ((byte >> 3) & 1) | (((byte >> 7) & 1) << 1);

        vec![pixel1, pixel2, pixel3, pixel4]
    }

    pub fn draw_pixel(&mut self, pixel_number: u8, x: u8, y: u8, offset: usize) {
        // pixel number being the 2bbp value from our vertical strip
        // let color = self.get_palette(self.read((0x4400 + pixel_number) as u16), pixel_number);
        let color = self.get_palette(1, pixel_number);
        let (r, g, b) = self.get_color(color);
        let v = self.u8_rgb(r, g, b);
        self.raster[WIDTH as usize * y as usize + x as usize] = v;
    }

    pub(crate) fn get_palette(&self, palette_number: u8, color_index: u8) -> u8 {
       // Color index is the byte // offset
        // self.read((palette_number as usize * 4 + color_index) as u16)
        self.palette_rom[palette_number as usize * 4 + color_index as usize]
    }

    // Gets the RGB color from color intensity values in color rom
    pub fn get_color(&self, index: u8) -> (u8, u8, u8) {
        use Color::*;
        use Intensity::*;
        let color = self.color_rom[index as usize];

        // Bit 0, 1, 2: Red
        let r = (color & 1) * Color::get(Red, Min)
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
