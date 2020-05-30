use std::borrow::Borrow;

use crate::interconnect::Interconnect;
use crate::memory::Memory;
use crate::pacman::display;
use crate::pacman::display::Display;

pub struct Pacman {
    int_vector: u8,
    int_enable: bool,
    port_in: u8,
    port_out: u8,
    ram: Vec<u8>,
    ctx: Interconnect,
    fb: Display,
    dip: Dip,
    in0: IN0,
    in1: IN1,
    c_lockout: bool,
    c_counter: bool,
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

impl Pacman {
    fn new() -> Self {
        Self {
            int_vector: 0,
            int_enable: false,
            port_in: 0,
            port_out: 0,
            ram: Vec::with_capacity(0x1000),
            ctx: Interconnect::new(),
            fb: Display::new(),
            dip: Dip::default(),
            in0: IN0::default(),
            in1: IN1::default(),
            c_lockout: false,
            c_counter: false,
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
            0x0000..=0x3FFF => self.ctx.cpu.memory.ram[addr as usize],
            0x4000..=0x4FFF => self.ram[addr as usize],
            0x5000 => self.int_enable as u8,
            0x5006 => self.ctx.cpu.memory.ram[addr as usize],
            0x5040..=0x507F => unimplemented!(), // self.in0 // Joystick and start buttons
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x4000..=0x4FFF => self.ram[addr as usize] = byte,
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
            _ => unimplemented!(),
        }
    }
}
