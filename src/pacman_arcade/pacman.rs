use log::{debug, error, info, warn};
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

pub use crate::pacman_arcade::display::{Display, HEIGHT, WIDTH};
pub use crate::z80_rs::interconnect::Interconnect;
pub use crate::z80_rs::memory::MemoryRW;

pub struct Pacman {
    pub int_vector: u8,
    // IO port 0x00 to write interrupt vector for CPU
    pub int_enable: bool,
    // Vblank or CPU interrupt
    pub port_in: u8,
    pub port_out: u8,
    pub ctx: Interconnect,
    pub fb: Display,
    pub dip: Dip,
    pub in0: IN0,
    pub in1: IN1,
    pub c_lockout: bool,
    pub c_counter: bool,
}

#[derive(Default)]
pub struct Dip {
    coins_per_game: u8,
    lives_per_game: u8,
    bonus_extra_life: u8,
    difficulty: bool,
    ghost_names: bool,
}

#[derive(Default)]
pub struct IN0 {
    joy_up: bool,
    joy_left: bool,
    joy_right: bool,
    joy_down: bool,
}

#[derive(Default)]
pub struct IN1 {
    joy_up: bool,
    joy_left: bool,
    joy_right: bool,
    joy_down: bool,
}

#[derive(Debug)]
enum Map {
    SpriteRom,
    ColorRom,
    PaletteRom,
    TileRom,
    Rom,
    Ram,
}

impl Pacman {
    pub fn new() -> Self {
        Self {
            int_vector: 0,
            int_enable: false,
            port_in: 0,
            port_out: 0,
            ctx: Interconnect::default(),
            fb: Display::new(),
            dip: Dip::default(),
            in0: IN0::default(),
            in1: IN1::default(),
            c_lockout: false,
            c_counter: false,
        }
    }
    pub fn init(&mut self) {
        self.ctx.cpu.cpm_compat = false;
        self.ctx.cpu.flags.zf = true;
        self.ctx.cpu.reg.ix = 0xFFFF;
        self.ctx.cpu.reg.iy = 0xFFFF;
        info!("Initialized z80 core");
    }

    fn load(&mut self, file: &mut File, map: Map, offset: usize) {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Unable to read file");

        for i in 0..buf.len() {
            match map {
                Map::SpriteRom => self.fb.sprite_rom[i] = buf[i],
                Map::ColorRom => self.fb.color_rom[i] = buf[i],
                Map::TileRom => self.fb.tile_rom[i] = buf[i],
                Map::PaletteRom => self.fb.palette_rom[i] = buf[i],
                Map::Rom => self.ctx.cpu.memory.rom[i + offset] = buf[i],
                Map::Ram => self.ctx.cpu.memory.ram[i + offset] = buf[i],
            }
        }
    }
    pub fn load_rom(&mut self, rom: &Vec<String>) {
        let mut collection: Vec<&str> = Vec::new();

        // Skip the target directory and use provided args
        for i in rom.iter().skip(1) {
            collection.push(&i);
        }
        for y in collection.iter() {
            let path = Path::new(y);

            if path.is_dir() {
                let rom = path.read_dir().unwrap();
                for entry in rom {
                    let f = entry.unwrap();
                    let file = File::open(f.path().as_path());

                    match f.file_name().to_str() {
                        Some("82s123.7f") => self.load(&mut file.unwrap(), Map::ColorRom, 0),
                        Some("82s126.4a") => self.load(&mut file.unwrap(), Map::PaletteRom, 0),
                        Some("pacman.6e") => self.load(&mut file.unwrap(), Map::Rom, 0),
                        Some("pacman.6f") => self.load(&mut file.unwrap(), Map::Rom, 0x1000),
                        Some("pacman.6h") => self.load(&mut file.unwrap(), Map::Rom, 0x2000),
                        Some("pacman.6j") => self.load(&mut file.unwrap(), Map::Rom, 0x3000),
                        Some("pacman.5e") => self.load(&mut file.unwrap(), Map::TileRom, 0),
                        Some("pacman.5f") => self.load(&mut file.unwrap(), Map::SpriteRom, 0),
                        _ => {} // Do nothing for non matches
                    }
                }
                println!("Rom files found & loaded..");
            } else if path.is_file() || !path.is_dir() {
                eprintln!(
                    "Pacman roms not found, please check your rom directory or provided arguments"
                );
                panic!(format!("Attempted to load: {}", path.display()));
            }
        }
    }
    pub fn render_sprite(&mut self, x: usize, y: usize, mut index: usize) {
        for y in 0..16 {
            for x in 0..16 {
                self.fb.draw_sprite(x, y, index, index, 1);
                index = index.wrapping_add(1);
            }
        }
        self.fb.window.update_with_buffer(&self.fb.raster, WIDTH, HEIGHT);

    }


    // Render the whole tile map
    pub(crate) fn render_tiles(&mut self) {
        let mut tile: u8 = 0;
        /*// VRAM screen offset
        let bottom = 0x4000;
        let middle = bottom + 64;
        let top = middle + 0x380;*/

        // AND the address with with 0x3F to wrap at 64 bytes (size of palette rom)
        let pal_no = self.read(0x4400);
        for y in 0..16 {
            for x in 0..16 {
                self.fb.draw_tile(tile as u8, x * 8, y * 8, 0);
                tile = tile.wrapping_add(1);
            }
            self.fb.window.update_with_buffer(&self.fb.raster, WIDTH, HEIGHT).ok();
        }
    }
    pub fn draw_screen(&mut self) {
        // Iterate over VRAM
        for i in 0x4300..=0x4FFF {
            for shift in 0..8 as isize {
                // println!("Read i:{:04x}", self.read(i));
                for y in 0..=HEIGHT as isize {
                    let x = i % (WIDTH as usize + shift as usize + y as usize);
                    self.fb.draw_sprite(x as usize, y as usize + shift as usize, 1 as usize, 1, 1);
                    // self.fb.draw_tile(i as u8, y as u8, x as u8, 1);
                }
                self.fb.window.update_with_buffer(&self.fb.raster, WIDTH, HEIGHT).ok();
            }
        }
    }
    //To to draw tile 4000, you mask with 1F to get 0 for reversed X,
    // then mask with 20 (and shift right 5) to get 0 again for Y (within that block).
    // Then reverse the X (1F-X) to flip the order, add 34 to Y
    // (because this block is on row 34 of the screen), multiply by the 8x8 tile size, & draw the tile.
    // Then repeat for tile 4001

    // Bottom:
    // First 64 addresses draw two rows, right to left, top to bottom
    // Note Sprites do not have any address encoding unlike tiles.
    // Each row 32 tiles wide.
    // Each fruit takes up 4 tiles
    pub fn draw_fruits(&mut self) {
        // Get the tile index at address 0x4004;
        let pal_no = 1;
        for i in 0x4000..=0x4FFF {

            // self.fb.vblank = true;

            let addr = self.fb.sprite_rom[i] * 64;
            let mut x = (i & 0x1F) * 8;
            let mut y = ((i & 0x20) >> 5) * 8 + 34;
            for row in 0..HEIGHT {
                self.fb.draw_slice(x, y, i, 1);
            }
            self.fb.window.update_with_buffer(&self.fb.raster, WIDTH, HEIGHT).ok();

        }
    }
}

// Mapper trait for the Pacman hardware
pub trait Mapper {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);
}

impl Mapper for Pacman {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.ctx.cpu.memory.rom[addr as usize],
            0x4000..=0x43FF => {
                debug!("Read to VRAM {:04x}", addr);
                self.fb.tile_rom[addr as usize - 0x4000]
                // self.fb.vram[addr as usize - 0x4000]
            }
            0x4400..=0x47FF => {
                debug!("Read to VRAM {:04x}", addr);
                self.fb.palette_rom[addr as usize - 0x4400]
                // self.fb.vram[addr as usize - 0x4400]
            },
            0x4800..=0x4BFF => {
                debug!("Read to VRAM {:04x}", addr);
                self.ctx.cpu.memory.ram[addr as usize - 0x4800]
            },
            0x4C00..=0x4FFF => {
                debug!("Read to WRAM {:04x}", addr);
                self.ctx.cpu.memory.ram[addr as usize - 0x4C00]
            },
            0x5000 => {
                info!("Read to IO interrupt");
                self.int_enable as u8
            },
            0x5040..=0x507F => unimplemented!(), // self.in0 // Joystick and start buttons
            _ => unimplemented!("Read to address:{:04X}", addr),
        }
    }

    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x4000..=0x43FF => {
                println!("Write to VRAM: {:04x}, byte: {:02x}", addr, byte);
                debug!("Write to VRAM: {:04x}, byte: {:02x}", addr, byte);
                self.fb.vram[addr as usize - 0x4000] = byte
            }
            0x4400..=0x47FF => {
                println!("Write to VRAM: {:04x}, byte: {:02x}", addr, byte);
                debug!("Write to VRAM: {:04x}, byte: {:02x}", addr, byte);
                self.fb.vram[addr as usize - 0x4400] = byte
            }
            0x4800..=0x4FEF => {
                println!("Write to VRAM: {:04x}, byte: {:02x}", addr, byte);
                debug!("Write to VRAM: {:04x}, byte: {:02x}", addr, byte);
                self.fb.vram[addr as usize - 0x4800] = byte
            }
            0x4C00..=0x4FFF => {
                println!("Write to WRAM: {:04x}, byte: {:02x}", addr, byte);
                debug!("Write to WRAM: {:04x}, byte: {:02x}", addr, byte);
                self.ctx.cpu.memory.ram[addr as usize - 0x4C00] = byte
            }

            0x5000 => {
                debug!("Enabling interrupts: {:04x}, byte: {:02x}", addr, byte);
                self.int_enable = true;
                self.ctx.cpu.int.int = byte & 0x01 != 0;
                self.ctx.cpu.int.irq = (byte & 0x01) != 0;
            }
            0x5001 => println!("Stubbed: Sound enable:{}", byte & 0x01 != 0),
            0x5002 => println!("Stubbed: Aux board enable write."),
            0x5004 => println!("Player 1 start lamp:{}", byte & 0x01 != 0),
            0x5005 => println!("Player 2 start lamp:{}", byte & 0x01 != 0),
            0x5006 => {
                println!("Coin lockout:{}", (byte & 0x01) != 0);
                self.ctx.cpu.memory.ram[addr as usize] = byte;
                if (byte & 0x01) != 0 {
                    self.c_lockout = true;
                }
            }
            0x5007 => {
                println!("Coin lockout:{}", (byte & 0x01) != 0);
                self.ctx.cpu.memory.ram[addr as usize] = byte;
                if (byte & 0x01) != 0 {
                    self.c_counter = true;
                }
            }
            _ => unimplemented!(
                "{}",
                format!("Write address:{:02X} Byte:{:02X}", addr, byte)
            ),
        }
    }
}
