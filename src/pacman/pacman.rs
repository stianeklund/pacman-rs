use std::convert::TryInto;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use crate::interconnect::Interconnect;
use crate::memory::MemoryRW;
use crate::pacman::display::{Display, HEIGHT, WIDTH};

pub struct Pacman {
    pub int_vector: u8,
    pub int_enable: bool, // Vblank or CPU interrupt
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
            ctx: Interconnect::new(),
            fb: Display::new(),
            dip: Dip::default(),
            in0: IN0::default(),
            in1: IN1::default(),
            c_lockout: false,
            c_counter: false,
        }
    }
    pub fn init(&mut self) {
        self.ctx.cpu.flags.zf = true;
        self.ctx.cpu.reg.ix = 0xFFFF;
        self.ctx.cpu.reg.iy = 0xFFFF;
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

    pub fn render_sprites(&mut self) {
        let mut sprite = 0;
        for mut y in 0..8 {
            for mut x in 0..8 {
                self.fb.draw_sprite(sprite, x * 8, y * 8);
                sprite = sprite.wrapping_add(1);
            }
        }
    }
    // Render the whole tile map
    pub(crate) fn render_tiles(&mut self) {
        let mut tile = 0;
        for mut y in 0..16 {
            for mut x in 0..16 {
                self.fb.draw_tile(tile, x * 8, y * 8);
                tile = tile.wrapping_add(1);
            }
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
            0x4000..=0x47FF => self.fb.tile_rom[addr as usize - 0x4000],
            0x4800..=0x4FEF => self.fb.vram[addr as usize - 0x4800],
            0x4FF0..=0x4FFF => self.fb.sprite_rom[addr as usize - 0x4ff0],
            0x5000 => self.int_enable as u8,
            0x5006 => self.ctx.cpu.memory.rom[addr as usize],
            0x5040..=0x507F => unimplemented!(), // self.in0 // Joystick and start buttons
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x4000..=0x43FF => self.fb.vram[addr as usize] = byte,
            0x4400..=0x47FF => self.fb.vram[addr as usize] = byte,
            0x4800..=0x4FEF => self.fb.vram[addr as usize] = byte,

            0x5000 => {
                self.int_enable = true;
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
