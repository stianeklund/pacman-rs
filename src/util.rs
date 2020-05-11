use crate::cpu::{CPU, Registers};
use std::fmt;
use std::fmt::{Debug, Formatter, Pointer, Result, Write, Display};
use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::interconnect::Interconnect;

pub trait ReadWrite {
    fn read_byte(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr.wrapping_add(1)) as u16) << 8 | self.read_byte(addr) as u16
    }
    fn read_hb(&self, addr: u16) -> u8 {
        self.read_byte(addr.wrapping_add(2))
    }
    fn read_lb(&self, addr: u16) -> u8 {
        self.read_byte(addr.wrapping_add(1))
    }
    fn read(&self, addr: u16) -> u16;
    fn write_word(&mut self, addr: u16, word: u16);
    fn write_byte(&mut self, addr: u16, byte: u8);

}

impl Display for Registers {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.debug_struct("Registers")
            .field("PC", &format_args!("{:04x}", self.prev_pc))
            .field("A", &format_args!("{:02x}", self.a))
            .field("BC", &format_args!("{:02x},{:02x}", self.b, self.c))
            .field("DE", &format_args!("{:02x},{:02x}", self.d, self.e))
            .field("HL", &format_args!("{:02x},{:02x}", self.h, self.l))
            .field("SP", &format_args!("{:04x}", self.sp))
            .finish()
    }
}

// TODO Improve printing here if possible.
// Ideally if we can utilize `Instruction` and print the immediate value etc.
impl Display for CPU {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt::Alignment::Left;
        write!(fmt, "Instruction");
        write!(fmt,  "{:w$}", &self.current_instruction, w=12);
        // write!(fmt, "{}", if self.memory.read);
        write!(fmt, "Opcode:\t");
        write!(fmt, "{:>04x}\t", self.opcode);
        write!(fmt, "PC:{:>04x}\t", self.reg.prev_pc);
        write!(fmt, "A:{:>02x}\t", self.reg.a);
        write!(fmt, "BC:{:>02x}{:02x}\t", self.reg.a, self.reg.b);
        write!(fmt, "DE:{:>02x}{:02x}\t", self.reg.d, self.reg.e);
        write!(fmt, "HL:{:>02x}{:02x}\t", self.reg.h, self.reg.l);
        write!(fmt, "IX:{:>04x}\t", self.reg.ix);
        write!(fmt, "IY:{:>04x}\t", self.reg.iy);
        write!(fmt, "SP:{:>04x}", self.reg.sp);
        write!(fmt, "S:{} ", self.flags.sign as u8);
        write!(fmt, "Z:{} ", self.flags.zero as u8);
        write!(fmt, "P:{} ", self.flags.parity as u8);
        write!(fmt, "C:{} ", self.flags.carry as u8);
        write!(fmt, "AC:{} ", self.flags.half_carry as u8);
        write!(fmt, "I:{}", self.flags.interrupt as u8)


        /*fmt.debug_struct("Cpu")
            .field("Instruction", &self.current_instruction)
            .field("Opcode", &format_args!("{:02x}", self.opcode))
            .field("PC", &format_args!("{:04x}", self.reg.prev_pc))
            .field("A", &format_args!("{:02x}", self.reg.a))
            .field("BC", &format_args!("{:02x},{:02x}", self.reg.b, self.reg.c))
            .field("DE", &format_args!("{:02x},{:02x}", self.reg.d, self.reg.e))
            .field("HL", &format_args!("{:02x},{:02x}", self.reg.h, self.reg.l))
            .field("SP", &format_args!("{:04x}", self.reg.sp))
            .field("S", &format_args!("{}", self.flags.sign as u8))
            .field("Z", &format_args!("{}", self.flags.zero as u8))
            .field("P", &format_args!("{}", self.flags.parity as u8))
            .field("C", &format_args!("{}", self.flags.carry as u8))
            .field("AC", &format_args!("{}", self.flags.half_carry as u8))
            .field("I", &format_args!("{}", self.flags.interrupt as u8))
            .finish()*/
    }
    // Notice how the windows flash when typing?

}
impl Debug for CPU {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            fmt,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}    \t{}\t{}\t{}\t{}\t{}\t{}\t",
            "Instruction",
            "Opcode",
            "PC",
            "A",
            "BC",
            "DE",
            "HL",
            "SP",
            "S   ",
            "Z   ",
            "P   ",
            "C   ",
            "AC   ",
            "I   "
        );
        // format!("{:0>10}"
        writeln!(
            // write!(fmt,  "{:w$}", &self.current_instruction, w=12);
            fmt, // S   Z   P   C   AC  Interrupt
            "{}\t{:04X}\t{:04X}\t{:02X}\t{:02X}{:02X}\t{:02X}{:02X}\t{:02X}{:02X}\t{:0>4X}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.current_instruction,
            self.opcode,
            self.reg.prev_pc,
            self.reg.a,
            self.reg.b,
            self.reg.c,
            self.reg.d,
            self.reg.e,
            self.reg.h,
            self.reg.l,
            self.reg.sp,
            self.flags.sign as u8,
            self.flags.zero as u8,
            self.flags.parity as u8,
            self.flags.carry as u8,
            self.flags.half_carry as u8,
            self.flags.interrupt as u8
        )
    }
}
