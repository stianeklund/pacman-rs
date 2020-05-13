use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};
use std::path::Path;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:?}", val)
    }
}

impl fmt::UpperHex for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:04X}", val)
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        &mut self.memory[index as usize]
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        &self.memory[index as usize]
    }
}
pub trait MemoryRW {
    fn read8(&self, addr: u16) -> u8;
    fn read16(&self, addr: u16) -> u16 {
        (self.read8(addr.wrapping_add(1)) as u16) << 8 | self.read8(addr) as u16
    }
    fn read_hb(&self, addr: u16) -> u8 {
        self.read8(addr.wrapping_add(2))
    }
    fn read_lb(&self, addr: u16) -> u8 {
        self.read8(addr.wrapping_add(1))
    }
    fn read(&self, addr: u16) -> u16;
    fn read_word(&self, addr: u16) -> u16;
    fn write_word(&mut self, addr: u16, word: u16);
    fn write_byte(&mut self, addr: u16, byte: u8);
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: vec![0; 0x1_0000],
        }
    }
    pub fn peek(&self, v: usize) -> u8 {
        self.memory[v]
    }
    pub fn load_bin(&mut self, rom: &Vec<String>) {
        let mut buf = Vec::new();
        let mut collection: Vec<&str> = Vec::new();

        for i in rom.iter().skip(1) {
            collection.push(&i);
        }
        for f in collection.iter() {
            let path = Path::new(f);
            let mut file = File::open(&path).unwrap();
            file.read_to_end(&mut buf).expect("Failed to read binary");

            self.memory[..buf.len()].clone_from_slice(&buf[..]);
            println!("Loaded: {:?} Bytes: {:?}", path, buf.len());
        }
    }

    pub fn load_tests(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = File::open(&path).expect("Couldn't load binary");
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).expect("Failed to read binary");
        // Tests are loaded at 0x0100
        self.memory[0x0100..(buf.len() + 0x0100)].clone_from_slice(&buf[..]);
        println!("Test loaded: {:?} Bytes: {:?}\n", path, buf.len());
    }
}
