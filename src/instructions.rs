use std::fmt;
use crate::memory::Memory;

#[derive(Debug, Default)]
pub struct Instruction {
    pub name: String,    // Mnemonic
    pub size: u8,        // Instruction size (bytes)
    pub cycles: u8 ,     // Clock cycles (if branch taken)
    pub alt_cycles: u8,  // If not branch taken etc.
}

impl fmt::UpperHex for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:X}", val)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    I,
    R,
    BC,
    DE,
    HL,
    SP,
    IX,
    IY,
}

impl fmt::UpperHex for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:X}", val)
    }
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            size: 0,
            cycles: 0,
            alt_cycles: 0
        }
    }

    pub fn from(mnemonic: &str, size: u8, cycles: u8, alt_cycles: u8) -> Instruction {
        Instruction {
            // name: mnemonic.to_string().pad_to_width_with_alignment(12, Alignment::Left),
            name: format!("{:w$}", mnemonic, w = 12),
            size,
            cycles,
            alt_cycles,
        }
    }
    pub fn decode(opcode: u16) -> Self {
        // MNEMONIC, Byte size, CPU cycles, conditional extra cycles
        match opcode {
            0x00 => Instruction::from("NOP", 1, 4, 0),
            0x01 => Instruction::from("LD BC, **", 3, 10, 0),
            0x02 => Instruction::from("LD (BC), A", 1, 7, 0),
            0x03 => Instruction::from("INC BC", 1, 6, 0),
            0x04 => Instruction::from("INC B", 1, 4, 0),
            0x05 => Instruction::from("DEC B", 1, 4, 0),
            0x06 => Instruction::from("LD B, *", 2, 7, 0),
            0x07 => Instruction::from("RLCA", 1, 4, 0),
            0x08 => Instruction::from("EX AF, AF", 1, 4, 0),
            0x09 => Instruction::from("ADD HL, BC", 1, 11, 0),
            0x0A => Instruction::from("LD A, (BC)", 1, 7, 0),
            0x0B => Instruction::from("DEC BC", 1, 6, 0),
            0x0C => Instruction::from("INC C", 1, 4, 0),
            0x0D => Instruction::from("DEC C", 1, 4, 0),
            0x0E => Instruction::from("LD C, *", 2, 7, 0),
            0x0F => Instruction::from("RRCA", 1, 4, 0),
            0x10 => Instruction::from("DJNZ, *", 2, 13, 8),
            0x11 => Instruction::from("LD DE, **", 3, 10, 0),
            0x12 => Instruction::from("LD (DE), A", 1, 7, 0),
            0x13 => Instruction::from("INC DE", 1, 6, 0),
            0x14 => Instruction::from("INC D", 1, 4, 0),
            0x15 => Instruction::from("DEC D", 1, 4, 0),
            0x16 => Instruction::from("LD D", 2, 7, 0),
            0x17 => Instruction::from("RLA", 1, 4, 0),
            0x18 => Instruction::from("JR *", 2, 12, 0),
            0x19 => Instruction::from("ADD HL, DE,", 1, 11, 0),
            0x1A => Instruction::from("LD A, (DE)", 1, 7, 0),
            0x1B => Instruction::from("DEC DE", 1, 6, 0),
            0x1C => Instruction::from("INC E", 1, 4, 0),
            0x1D => Instruction::from("DEC E", 1, 4, 0),
            0x1E => Instruction::from("LD E, *", 2, 7, 0),
            0x1F => Instruction::from("RRA", 1, 4, 0),
            0x20 => Instruction::from("JR NZ, *", 2, 12, 7),
            0x21 => Instruction::from("LD HL, **", 3, 10, 0),
            0x22 => Instruction::from("LD **, HL", 3, 16, 0),
            0x23 => Instruction::from("INC HL", 1, 6, 0),
            0x24 => Instruction::from("INC H", 1, 4, 0),
            0x25 => Instruction::from("DEC H", 1, 4, 0),
            0x26 => Instruction::from("LD H, *", 2, 7, 0),
            0x27 => Instruction::from("DAA", 1, 4, 0),
            0x28 => Instruction::from("JR Z, *", 2, 12, 7),
            0x29 => Instruction::from("ADD HL, HL", 1, 11, 0),
            0x2A => Instruction::from("LD HL, (**)", 3, 16, 0),
            0x2B => Instruction::from("DEC HL", 1, 6, 0),
            0x2C => Instruction::from("INC L", 1, 4, 0),
            0x2D => Instruction::from("DEC L", 1, 4, 0),
            0x2E => Instruction::from("LD L, *", 2, 7, 0),
            0x2F => Instruction::from("CPL", 1, 4, 0),
            0x30 => Instruction::from("JR NC, *", 2, 12, 7),
            0x31 => Instruction::from("LD SP", 3, 10, 0),
            0x32 => Instruction::from("LD (**), A", 3, 13, 0),
            0x33 => Instruction::from("INC SP", 1, 6, 0),
            0x34 => Instruction::from("INC (HL)", 1, 11, 0),
            0x35 => Instruction::from("DEC (HL)", 1, 11, 0),
            0x36 => Instruction::from("LD (HL), *", 2, 10, 0),
            0x37 => Instruction::from("SCF", 1, 4, 0),
            0x38 => Instruction::from("JR C, *", 2, 12, 7),
            0x39 => Instruction::from("ADD HL,SP", 1, 11, 0),
            0x3A => Instruction::from("LD A, (**)", 3, 13, 0),
            0x3B => Instruction::from("DEC SP", 1, 6, 0),
            0x3C => Instruction::from("INC A", 1, 4, 0),
            0x3D => Instruction::from("DEC A", 1, 4, 0),
            0x3E => Instruction::from("LD A, *", 2, 7, 0),
            0x3F => Instruction::from("CCF", 1, 4, 0),
            0x40 => Instruction::from("LD B, B", 1, 4, 0),
            0x41 => Instruction::from("LD B, C", 1, 4, 0),
            0x42 => Instruction::from("LD B, D", 1, 4, 0),
            0x43 => Instruction::from("LD B, E", 1, 4, 0),
            0x44 => Instruction::from("LD B, H", 1, 4, 0),
            0x45 => Instruction::from("LD B, L", 1, 4, 0),
            0x46 => Instruction::from("LD B, (HL)", 1, 7, 0),
            0x47 => Instruction::from("LD B, A", 1, 4, 0),
            0x48 => Instruction::from("LD C, B", 1, 4, 0),
            0x49 => Instruction::from("LD C, C", 1, 4, 0),
            0x4A => Instruction::from("LD C, D", 1, 4, 0),
            0x4B => Instruction::from("LD C, E", 1, 4, 0),
            0x4C => Instruction::from("LD C, H", 1, 4, 0),
            0x4D => Instruction::from("LD C, L", 1, 4, 0),
            0x4E => Instruction::from("LD C, (HL)", 1, 7, 0),
            0x4F => Instruction::from("LD C, A", 1, 4, 0),
            0x50 => Instruction::from("LD D, B", 1, 4, 0),
            0x51 => Instruction::from("LD D, C", 1, 4, 0),
            0x52 => Instruction::from("LD D, D", 1, 4, 0),
            0x53 => Instruction::from("LD D, E", 1, 4, 0),
            0x54 => Instruction::from("LD D, H", 1, 4, 0),
            0x55 => Instruction::from("LD D, L", 1, 4, 0),
            0x56 => Instruction::from("LD D, (HL)", 1, 7, 0),
            0x57 => Instruction::from("LD D, A", 1, 4, 0),
            0x58 => Instruction::from("LD E, B", 1, 4, 0),
            0x59 => Instruction::from("LD E, C", 1, 4, 0),
            0x5A => Instruction::from("LD E, D", 1, 4, 0),
            0x5B => Instruction::from("LD E, E", 1, 4, 0),
            0x5C => Instruction::from("LD E, H", 1, 4, 0),
            0x5D => Instruction::from("LD E, L", 1, 4, 0),
            0x5E => Instruction::from("LD E, (HL)", 1, 7, 0),
            0x5F => Instruction::from("LD E, A", 1, 4, 0),
            0x60 => Instruction::from("LD H, B", 1, 4, 0),
            0x61 => Instruction::from("LD H, C", 1, 4, 0),
            0x62 => Instruction::from("LD H, D", 1, 4, 0),
            0x63 => Instruction::from("LD H, E", 1, 4, 0),
            0x64 => Instruction::from("LD H, H", 1, 4, 0),
            0x65 => Instruction::from("LD H, L", 1, 4, 0),
            0x66 => Instruction::from("LD H, (HL)", 1, 7, 0),
            0x67 => Instruction::from("LD H, A", 1, 4, 0),
            0x68 => Instruction::from("LD L, B", 1, 4, 0),
            0x69 => Instruction::from("LD L, C", 1, 4, 0),
            0x6A => Instruction::from("LD L, D", 1, 4, 0),
            0x6B => Instruction::from("LD L, E", 1, 4, 0),
            0x6C => Instruction::from("LD L, H", 1, 4, 0),
            0x6D => Instruction::from("LD L, L", 1, 4, 0),
            0x6E => Instruction::from("LD L, (HL)", 1, 7, 0),
            0x6F => Instruction::from("LD L, A", 1, 4, 0),
            0x70 => Instruction::from("LD (HL), B", 1, 7, 0),
            0x71 => Instruction::from("LD (HL), C", 1, 7, 0),
            0x72 => Instruction::from("LD (HL), D", 1, 7, 0),
            0x73 => Instruction::from("LD (HL), E", 1, 7, 0),
            0x74 => Instruction::from("LD (HL), H", 1, 7, 0),
            0x75 => Instruction::from("LD (HL), L", 1, 7, 0),
            0x76 => Instruction::from("HALT", 1, 4, 0),
            0x77 => Instruction::from("LD (HL), A)", 1, 7, 0),
            0x78 => Instruction::from("LD A, B", 1, 4, 0),
            0x79 => Instruction::from("LD A, C", 1, 4, 0),
            0x7A => Instruction::from("LD A, D", 1, 4, 0),
            0x7B => Instruction::from("LD A, E", 1, 4, 0),
            0x7C => Instruction::from("LD A, H", 1, 4, 0),
            0x7D => Instruction::from("LD A, L", 1, 4, 0),
            0x7E => Instruction::from("LD A, (HL)", 1, 7, 0),
            0x7F => Instruction::from("LD A, A", 1, 7, 0),
            0x80 => Instruction::from("ADD B", 1, 4, 0),
            0x81 => Instruction::from("ADD C", 1, 4, 0),
            0x82 => Instruction::from("ADD D", 1, 4, 0),
            0x83 => Instruction::from("ADD E", 1, 4, 0),
            0x84 => Instruction::from("ADD H", 1, 4, 0),
            0x85 => Instruction::from("ADD L", 1, 4, 0),
            0x86 => Instruction::from("ADD A, (HL)", 1, 7, 0),
            0x87 => Instruction::from("ADD A", 1, 4, 0),
            0x88 => Instruction::from("ADC B", 1, 4, 0),
            0x89 => Instruction::from("ADC C", 1, 4, 0),
            0x8A => Instruction::from("ADC D", 1, 4, 0),
            0x8B => Instruction::from("ADC E", 1, 4, 0),
            0x8C => Instruction::from("ADC H", 1, 4, 0),
            0x8D => Instruction::from("ADC L", 1, 4, 0),
            0x8E => Instruction::from("ADC (HL)", 1, 7, 0),
            0x8F => Instruction::from("ADC A", 1, 4, 0),

            0x90 => Instruction::from("SUB B", 1, 4, 0),
            0x91 => Instruction::from("SUB C", 1, 4, 0),
            0x92 => Instruction::from("SUB D", 1, 4, 0),
            0x93 => Instruction::from("SUB E", 1, 4, 0),
            0x94 => Instruction::from("SUB H", 1, 4, 0),
            0x95 => Instruction::from("SUB L", 1, 4, 0),
            0x96 => Instruction::from("SUB (HL)", 1, 7, 0),
            0x97 => Instruction::from("SUB A", 1, 4, 0),
            0x98 => Instruction::from("SBB B", 1, 4, 0),
            0x99 => Instruction::from("SBB C", 1, 4, 0),
            0x9A => Instruction::from("SBB D", 1, 4, 0),
            0x9B => Instruction::from("SBB E", 1, 4, 0),
            0x9C => Instruction::from("SBB H", 1, 4, 0),
            0x9D => Instruction::from("SBB L", 1, 4, 0),
            0x9E => Instruction::from("SBB (HL)", 1, 7, 0),
            0x9F => Instruction::from("SBB A", 1, 4, 0),
            0xA0 => Instruction::from("AND B", 1, 4, 0),
            0xA1 => Instruction::from("AND C", 1, 4, 0),
            0xA2 => Instruction::from("AND D", 1, 4, 0),
            0xA3 => Instruction::from("AND E", 1, 4, 0),
            0xA4 => Instruction::from("AND H", 1, 4, 0),
            0xA5 => Instruction::from("AND L", 1, 4, 0),
            0xA6 => Instruction::from("AND (HL)", 1, 7, 0),
            0xA7 => Instruction::from("AND A", 1, 4, 0),
            0xA8 => Instruction::from("XOR B", 1, 4, 0),
            0xA9 => Instruction::from("XOR C", 1, 4, 0),
            0xAA => Instruction::from("XOR D", 1, 4, 0),
            0xAB => Instruction::from("XOR E", 1, 4, 0),
            0xAC => Instruction::from("XOR H", 1, 4, 0),
            0xAD => Instruction::from("XOR L", 1, 4, 0),
            0xAE => Instruction::from("XOR (HL)", 1, 7, 0),
            0xAF => Instruction::from("XOA A", 1, 4, 0),
            0xB0 => Instruction::from("OR B", 1, 4, 0),
            0xB1 => Instruction::from("OR C", 1, 4, 0),
            0xB2 => Instruction::from("OR D", 1, 4, 0),
            0xB3 => Instruction::from("OR E", 1, 4, 0),
            0xB4 => Instruction::from("OR H", 1, 4, 0),
            0xB5 => Instruction::from("OR L", 1, 4, 0),
            0xB6 => Instruction::from("OR (HL)", 1, 7, 0),
            0xB7 => Instruction::from("OR A", 1, 4, 0),
            0xB8 => Instruction::from("CP B", 1, 4, 0),
            0xB9 => Instruction::from("CP C", 1, 4, 0),
            0xBA => Instruction::from("CP D", 1, 4, 0),
            0xBB => Instruction::from("CP E", 1, 4, 0),
            0xBC => Instruction::from("CP H", 1, 4, 0),
            0xBD => Instruction::from("CP L", 1, 4, 0),
            0xBE => Instruction::from("CP (HL)", 1, 7, 0),
            0xBF => Instruction::from("CP A", 1, 4, 0),
            0xC0 => Instruction::from("RET NZ", 1, 11, 5),
            0xC1 => Instruction::from("POP BC", 1, 10, 0),
            0xC2 => Instruction::from("JP NZ, **", 3, 10, 0),
            0xC3 => Instruction::from("JP **", 3, 10, 0),
            0xC4 => Instruction::from("CALL NZ, **", 3, 17, 10),
            0xC5 => Instruction::from("PUSH BC", 1, 11, 0),
            0xC6 => Instruction::from("ADD A, *", 2, 7, 0),
            0xC7 => Instruction::from("RST 00H", 1, 11, 0),
            0xC8 => Instruction::from("RET Z", 1, 11, 5),
            0xC9 => Instruction::from("RET", 1, 10, 0),
            0xCA => Instruction::from("JP Z, **", 3, 10, 0),
            0xCB => {
                // BIT instructions
                match opcode {
                    0x00 => Instruction::from("RLC B", 2, 8, 0),
                    0x01 => Instruction::from("RLC C", 2, 8, 0),
                    0x02 => Instruction::from("RLC D", 2, 8, 0),
                    0x03 => Instruction::from("RLC E", 2, 8, 0),
                    0x04 => Instruction::from("RLC H", 2, 8, 0),
                    0x05 => Instruction::from("RLC L", 2, 8, 0),
                    0x06 => Instruction::from("RLC (HL)", 2, 15, 0),
                    0x07 => Instruction::from("RLC A", 2, 8, 0),
                    0x08 => Instruction::from("RRC B", 2, 8, 0),
                    0x09 => Instruction::from("RRC C", 2, 8, 0),
                    0x0A => Instruction::from("RRC D", 2, 8, 0),
                    0x0B => Instruction::from("RRC E", 2, 8, 0),
                    0x0C => Instruction::from("RRC H", 2, 8, 0),
                    0x0D => Instruction::from("RRC L", 2, 8, 0),
                    0x0E => Instruction::from("RRC (HL)", 2, 15, 0),
                    0x0F => Instruction::from("RRC A", 2, 8, 0),
                    0x10 => Instruction::from("RL B", 2, 8, 0),
                    0x11 => Instruction::from("RL C", 2, 8, 0),
                    0x12 => Instruction::from("RL D", 2, 8, 0),
                    0x13 => Instruction::from("RL E", 2, 8, 0),
                    0x14 => Instruction::from("RL H", 2, 8, 0),
                    0x15 => Instruction::from("RL L", 2, 8, 0),
                    0x16 => Instruction::from("RL (HL)", 2, 15, 0),
                    0x17 => Instruction::from("RL A", 2, 8, 0),
                    0x18 => Instruction::from("RR B", 2, 8, 0),
                    0x19 => Instruction::from("RR C", 2, 8, 0),
                    0x1A => Instruction::from("RR D", 2, 8, 0),
                    0x1B => Instruction::from("RR E", 2, 8, 0),
                    0x1C => Instruction::from("RR H", 2, 8, 0),
                    0x1D => Instruction::from("RR L", 2, 8, 0),
                    0x1E => Instruction::from("RR (HL)", 2, 15, 0),
                    0x1F => Instruction::from("RR A", 2, 8, 0),
                    0x20 => Instruction::from("SLA B", 2, 8, 0),
                    0x21 => Instruction::from("SLA C", 2, 8, 0),
                    0x22 => Instruction::from("SLA D", 2, 8, 0),
                    0x23 => Instruction::from("SLA E", 2, 8, 0),
                    0x24 => Instruction::from("SLA H", 2, 8, 0),
                    0x25 => Instruction::from("SLA L", 2, 8, 0),
                    0x26 => Instruction::from("SLA (HL)", 2, 15, 0),
                    0x27 => Instruction::from("SLA A", 2, 8, 0),
                    0x28 => Instruction::from("SRA B", 2, 8, 0),
                    0x29 => Instruction::from("SRA C", 2, 8, 0),
                    0x2A => Instruction::from("SRA D", 2, 8, 0),
                    0x2B => Instruction::from("SRA E", 2, 8, 0),
                    0x2C => Instruction::from("SRA H", 2, 8, 0),
                    0x2D => Instruction::from("SRA L", 2, 8, 0),
                    0x2E => Instruction::from("SRA (HL)", 2, 15, 0),
                    0x2F => Instruction::from("SRA A", 2, 8, 0),

                    0x30 => Instruction::from("SLL B", 2, 8, 0),
                    0x31 => Instruction::from("SLL C", 2, 8, 0),
                    0x32 => Instruction::from("SLL D", 2, 8, 0),
                    0x33 => Instruction::from("SLL E", 2, 8, 0),
                    0x34 => Instruction::from("SLL H", 2, 8, 0),
                    0x35 => Instruction::from("SLL L", 2, 8, 0),
                    0x36 => Instruction::from("SLL (HL)", 2, 15, 0),
                    0x37 => Instruction::from("SLL A", 2, 8, 0),
                    0x38 => Instruction::from("SRL B", 2, 8, 0),
                    0x39 => Instruction::from("SRL C", 2, 8, 0),
                    0x3A => Instruction::from("SRL D", 2, 8, 0),
                    0x3B => Instruction::from("SRL E", 2, 8, 0),
                    0x3C => Instruction::from("SRL H", 2, 8, 0),
                    0x3D => Instruction::from("SRL L", 2, 8, 0),
                    0x3E => Instruction::from("SRL (HL)", 2, 15, 0),
                    0x3F => Instruction::from("SRL A", 2, 8, 0),

                    0x40 => Instruction::from("BIT 0, B", 2, 8, 0),
                    0x41 => Instruction::from("BIT 0, C", 2, 8, 0),
                    0x42 => Instruction::from("BIT 0, D", 2, 8, 0),
                    0x43 => Instruction::from("BIT 0, E", 2, 8, 0),
                    0x44 => Instruction::from("BIT 0, H", 2, 8, 0),
                    0x45 => Instruction::from("BIT 0, L", 2, 8, 0),
                    0x46 => Instruction::from("BIT 0, (HL)", 2, 12, 0),
                    0x47 => Instruction::from("BIT 0, A", 2, 8, 0),

                    0x48 => Instruction::from("BIT 1, B", 2, 8, 0),
                    0x49 => Instruction::from("BIT 1, C", 2, 8, 0),
                    0x4A => Instruction::from("BIT 1, D", 2, 8, 0),
                    0x4B => Instruction::from("BIT 1, E", 2, 8, 0),
                    0x4C => Instruction::from("BIT 1, H", 2, 8, 0),
                    0x4D => Instruction::from("BIT 1, L", 2, 8, 0),
                    0x4E => Instruction::from("BIT 1, (HL)", 2, 12, 0),
                    0x4F => Instruction::from("BIT 1, A", 2, 8, 0),

                    0x50 => Instruction::from("BIT 2, B", 2, 8, 0),
                    0x51 => Instruction::from("BIT 2, C", 2, 8, 0),
                    0x52 => Instruction::from("BIT 2, D", 2, 8, 0),
                    0x53 => Instruction::from("BIT 2, E", 2, 8, 0),
                    0x54 => Instruction::from("BIT 2, H", 2, 8, 0),
                    0x55 => Instruction::from("BIT 2, L", 2, 8, 0),
                    0x56 => Instruction::from("BIT 2, (HL)", 2, 12, 0),
                    0x57 => Instruction::from("BIT 2, A", 2, 8, 0),
                    0x58 => Instruction::from("BIT 3, B", 2, 8, 0),
                    0x59 => Instruction::from("BIT 3, C", 2, 8, 0),
                    0x5A => Instruction::from("BIT 3, D", 2, 8, 0),
                    0x5B => Instruction::from("BIT 3, E", 2, 8, 0),
                    0x5C => Instruction::from("BIT 3, H", 2, 8, 0),
                    0x5D => Instruction::from("BIT 3, L", 2, 8, 0),
                    0x5E => Instruction::from("BIT 3, (HL)", 2, 12, 0),
                    0x5F => Instruction::from("BIT 3, A", 2, 8, 0),

                    0x60 => Instruction::from("BIT 4, B", 2, 8, 0),
                    0x61 => Instruction::from("BIT 4, C", 2, 8, 0),
                    0x62 => Instruction::from("BIT 4, D", 2, 8, 0),
                    0x63 => Instruction::from("BIT 4, E", 2, 8, 0),
                    0x64 => Instruction::from("BIT 4, H", 2, 8, 0),
                    0x65 => Instruction::from("BIT 4, L", 2, 8, 0),
                    0x66 => Instruction::from("BIT 4, (HL)", 2, 12, 0),
                    0x67 => Instruction::from("BIT 4, A", 2, 8, 0),
                    0x68 => Instruction::from("BIT 5, B", 2, 8, 0),
                    0x69 => Instruction::from("BIT 5, C", 2, 8, 0),
                    0x6A => Instruction::from("BIT 5, D", 2, 8, 0),
                    0x6B => Instruction::from("BIT 5, E", 2, 8, 0),
                    0x6C => Instruction::from("BIT 5, H", 2, 8, 0),
                    0x6D => Instruction::from("BIT 5, L", 2, 8, 0),
                    0x6E => Instruction::from("BIT 5, (HL)", 2, 12, 0),
                    0x6F => Instruction::from("BIT 5, A", 2, 8, 0),

                    0x70 => Instruction::from("BIT 6, B", 2, 8, 0),
                    0x71 => Instruction::from("BIT 6, C", 2, 8, 0),
                    0x72 => Instruction::from("BIT 6, D", 2, 8, 0),
                    0x73 => Instruction::from("BIT 6, E", 2, 8, 0),
                    0x74 => Instruction::from("BIT 6, H", 2, 8, 0),
                    0x75 => Instruction::from("BIT 6, L", 2, 8, 0),
                    0x76 => Instruction::from("BIT 6, (HL)", 2, 12, 0),
                    0x77 => Instruction::from("BIT 6, A", 2, 8, 0),
                    0x78 => Instruction::from("BIT 7, B", 2, 8, 0),
                    0x79 => Instruction::from("BIT 7, C", 2, 8, 0),
                    0x7A => Instruction::from("BIT 7, D", 2, 8, 0),
                    0x7B => Instruction::from("BIT 7, E", 2, 8, 0),
                    0x7C => Instruction::from("BIT 7, H", 2, 8, 0),
                    0x7D => Instruction::from("BIT 7, L", 2, 8, 0),
                    0x7E => Instruction::from("BIT 7, (HL)", 2, 12, 0),
                    0x7F => Instruction::from("BIT 7, A", 2, 8, 0),

                    0x80 => Instruction::from("RES 0, B", 2, 8, 0),
                    0x81 => Instruction::from("RES 0, C", 2, 8, 0),
                    0x82 => Instruction::from("RES 0, D", 2, 8, 0),
                    0x83 => Instruction::from("RES 0, E", 2, 8, 0),
                    0x84 => Instruction::from("RES 0, H", 2, 8, 0),
                    0x85 => Instruction::from("RES 0, L", 2, 8, 0),
                    0x86 => Instruction::from("RES 0, (HL)", 2, 12, 0),
                    0x87 => Instruction::from("RES 0, A", 2, 8, 0),
                    0x88 => Instruction::from("RES 1, B", 2, 8, 0),
                    0x89 => Instruction::from("RES 1, C", 2, 8, 0),
                    0x8A => Instruction::from("RES 1, D", 2, 8, 0),
                    0x8B => Instruction::from("RES 1, E", 2, 8, 0),
                    0x8C => Instruction::from("RES 1, H", 2, 8, 0),
                    0x8D => Instruction::from("RES 1, L", 2, 8, 0),
                    0x8E => Instruction::from("RES 1, (HL)", 2, 12, 0),
                    0x8F => Instruction::from("RES 1, A", 2, 8, 0),

                    0x90 => Instruction::from("RES 2, B", 2, 8, 0),
                    0x91 => Instruction::from("RES 2, C", 2, 8, 0),
                    0x92 => Instruction::from("RES 2, D", 2, 8, 0),
                    0x93 => Instruction::from("RES 2, E", 2, 8, 0),
                    0x94 => Instruction::from("RES 2, H", 2, 8, 0),
                    0x95 => Instruction::from("RES 2, L", 2, 8, 0),
                    0x96 => Instruction::from("RES 2, (HL)", 2, 12, 0),
                    0x97 => Instruction::from("RES 2, A", 2, 8, 0),
                    0x98 => Instruction::from("RES 3, B", 2, 8, 0),
                    0x99 => Instruction::from("RES 3, C", 2, 8, 0),
                    0x9A => Instruction::from("RES 3, D", 2, 8, 0),
                    0x9B => Instruction::from("RES 3, E", 2, 8, 0),
                    0x9C => Instruction::from("RES 3, H", 2, 8, 0),
                    0x9D => Instruction::from("RES 3, L", 2, 8, 0),
                    0x9E => Instruction::from("RES 3, (HL)", 2, 12, 0),
                    0x9F => Instruction::from("RES 3, A", 2, 8, 0),

                    0xA0 => Instruction::from("RES 4, B", 2, 8, 0),
                    0xA1 => Instruction::from("RES 4, C", 2, 8, 0),
                    0xA2 => Instruction::from("RES 4, D", 2, 8, 0),
                    0xA3 => Instruction::from("RES 4, E", 2, 8, 0),
                    0xA4 => Instruction::from("RES 4, H", 2, 8, 0),
                    0xA5 => Instruction::from("RES 4, L", 2, 8, 0),
                    0xA6 => Instruction::from("RES 4, (HL)", 2, 12, 0),
                    0xA7 => Instruction::from("RES 4, A", 2, 8, 0),
                    0xA8 => Instruction::from("RES 5, B", 2, 8, 0),
                    0xA9 => Instruction::from("RES 5, C", 2, 8, 0),
                    0xAA => Instruction::from("RES 5, D", 2, 8, 0),
                    0xAB => Instruction::from("RES 5, E", 2, 8, 0),
                    0xAC => Instruction::from("RES 5, H", 2, 8, 0),
                    0xAD => Instruction::from("RES 5, L", 2, 8, 0),
                    0xAE => Instruction::from("RES 5, (HL)", 2, 12, 0),
                    0xAF => Instruction::from("RES 5, A", 2, 8, 0),

                    0xB0 => Instruction::from("RES 6, B", 2, 8, 0),
                    0xB1 => Instruction::from("RES 6, C", 2, 8, 0),
                    0xB2 => Instruction::from("RES 6, D", 2, 8, 0),
                    0xB3 => Instruction::from("RES 6, E", 2, 8, 0),
                    0xB4 => Instruction::from("RES 6, H", 2, 8, 0),
                    0xB5 => Instruction::from("RES 6, L", 2, 8, 0),
                    0xB6 => Instruction::from("RES 6, (HL)", 2, 12, 0),
                    0xB7 => Instruction::from("RES 6, A", 2, 8, 0),
                    0xB8 => Instruction::from("RES 7, B", 2, 8, 0),
                    0xB9 => Instruction::from("RES 7, C", 2, 8, 0),
                    0xBA => Instruction::from("RES 7, D", 2, 8, 0),
                    0xBB => Instruction::from("RES 7, E", 2, 8, 0),
                    0xBC => Instruction::from("RES 7, H", 2, 8, 0),
                    0xBD => Instruction::from("RES 7, L", 2, 8, 0),
                    0xBE => Instruction::from("RES 7, (HL)", 2, 12, 0),
                    0xBF => Instruction::from("RES 7, A", 2, 8, 0),

                    0xC0 => Instruction::from("SET 0, B", 2, 8, 0),
                    0xC1 => Instruction::from("SET 0, C", 2, 8, 0),
                    0xC2 => Instruction::from("SET 0, D", 2, 8, 0),
                    0xC3 => Instruction::from("SET 0, E", 2, 8, 0),
                    0xC4 => Instruction::from("SET 0, H", 2, 8, 0),
                    0xC5 => Instruction::from("SET 0, L", 2, 8, 0),
                    0xC6 => Instruction::from("SET 0, (HL)", 2, 15, 0),
                    0xC7 => Instruction::from("SET 0, A", 2, 8, 0),
                    0xC8 => Instruction::from("SET 1, B", 2, 8, 0),
                    0xC9 => Instruction::from("SET 1, C", 2, 8, 0),
                    0xCA => Instruction::from("SET 1, D", 2, 8, 0),
                    0xCB => Instruction::from("SET 1, E", 2, 8, 0),
                    0xCC => Instruction::from("SET 1, H", 2, 8, 0),
                    0xCD => Instruction::from("SET 1, L", 2, 8, 0),
                    0xCE => Instruction::from("SET 1, (HL)", 2, 15, 0),
                    0xCF => Instruction::from("SET 1, A", 2, 8, 0),

                    0xD0 => Instruction::from("SET 2, B", 2, 8, 0),
                    0xD1 => Instruction::from("SET 2, C", 2, 8, 0),
                    0xD2 => Instruction::from("SET 2, D", 2, 8, 0),
                    0xD3 => Instruction::from("SET 2, E", 2, 8, 0),
                    0xD4 => Instruction::from("SET 2, H", 2, 8, 0),
                    0xD5 => Instruction::from("SET 2, L", 2, 8, 0),
                    0xD6 => Instruction::from("SET 2, (HL)", 2, 15, 0),
                    0xD7 => Instruction::from("SET 2, A", 2, 8, 0),
                    0xD8 => Instruction::from("SET 3, B", 2, 8, 0),
                    0xD9 => Instruction::from("SET 3, C", 2, 8, 0),
                    0xDA => Instruction::from("SET 3, D", 2, 8, 0),
                    0xDB => Instruction::from("SET 3, E", 2, 8, 0),
                    0xDC => Instruction::from("SET 3, H", 2, 8, 0),
                    0xDD => Instruction::from("SET 3, L", 2, 8, 0),
                    0xDE => Instruction::from("SET 3, (HL)", 2, 15, 0),
                    0xDF => Instruction::from("SET 3, A", 2, 8, 0),

                    0xE0 => Instruction::from("SET 4, B", 2, 8, 0),
                    0xE1 => Instruction::from("SET 4, C", 2, 8, 0),
                    0xE2 => Instruction::from("SET 4, D", 2, 8, 0),
                    0xE3 => Instruction::from("SET 4, E", 2, 8, 0),
                    0xE4 => Instruction::from("SET 4, H", 2, 8, 0),
                    0xE5 => Instruction::from("SET 4, L", 2, 8, 0),
                    0xE6 => Instruction::from("SET 4, (HL)", 2, 15, 0),
                    0xE7 => Instruction::from("SET 4, A", 2, 8, 0),
                    0xE8 => Instruction::from("SET 5, B", 2, 8, 0),
                    0xE9 => Instruction::from("SET 5, C", 2, 8, 0),
                    0xEA => Instruction::from("SET 5, D", 2, 8, 0),
                    0xEB => Instruction::from("SET 5, E", 2, 8, 0),
                    0xEC => Instruction::from("SET 5, H", 2, 8, 0),
                    0xED => Instruction::from("SET 5, L", 2, 8, 0),
                    0xEE => Instruction::from("SET 5, (HL)", 2, 15, 0),
                    0xEF => Instruction::from("SET 5, A", 2, 8, 0),

                    0xF0 => Instruction::from("SET 6, B", 2, 8, 0),
                    0xF1 => Instruction::from("SET 6, C", 2, 8, 0),
                    0xF2 => Instruction::from("SET 6, D", 2, 8, 0),
                    0xF3 => Instruction::from("SET 6, E", 2, 8, 0),
                    0xF4 => Instruction::from("SET 6, H", 2, 8, 0),
                    0xF5 => Instruction::from("SET 6, L", 2, 8, 0),
                    0xF6 => Instruction::from("SET 6, (HL)", 2, 15, 0),
                    0xF7 => Instruction::from("SET 6, A", 2, 8, 0),
                    0xF8 => Instruction::from("SET 7, B", 2, 8, 0),
                    0xF9 => Instruction::from("SET 7, C", 2, 8, 0),
                    0xFA => Instruction::from("SET 7, D", 2, 8, 0),
                    0xFB => Instruction::from("SET 7, E", 2, 8, 0),
                    0xFC => Instruction::from("SET 7, H", 2, 8, 0),
                    0xFD => Instruction::from("SET 7, L", 2, 8, 0),
                    0xFE => Instruction::from("SET 7, (HL)", 2, 15, 0),
                    0xFF => Instruction::from("SET 7, A", 2, 8, 0),

                    _ => unimplemented!("Unknown CB prefixed instruction"),
                }
            },
            0xCC => Instruction::from("CALL Z, **", 3, 17, 10),
            0xCD => Instruction::from("CALL **", 3, 17, 0),
            0xCE => Instruction::from("ADC A, *", 2, 7, 0),
            0xCF => Instruction::from("RST 08H", 1, 11, 0),
            0xD0 => Instruction::from("RET NC", 1, 11, 5),
            0xD1 => Instruction::from("POP DE", 1, 10, 0),
            0xD2 => Instruction::from("JP NC, **", 3, 10, 0),
            0xD3 => Instruction::from("OUT_(*), A", 2, 11, 0),
            0xD4 => Instruction::from("CALL NC, **", 3, 17, 10),
            0xD5 => Instruction::from("PUSH DE", 1, 11, 0),
            0xD6 => Instruction::from("SUB *", 2, 7, 0),
            0xD7 => Instruction::from("RST 10H", 1, 11, 0),
            0xD8 => Instruction::from("RET C", 1, 11, 5),
            0xD9 => Instruction::from("EXX", 1, 4, 0),
            0xDA => Instruction::from("JP C, **", 3, 10, 0),
            0xDB => Instruction::from("IN A, (*)", 2, 11, 0),
            0xDC => Instruction::from("CALL C, **", 3, 17, 10),
            0xDD => {
                // IX Instructions
                match opcode {
                    0x09 => Instruction::from("ADD IX, B", 2, 15, 0),
                    0x21 => Instruction::from("LD IX, **", 4, 14, 0),
                    0x22 => Instruction::from("LD (**), IX", 4, 20, 0),
                    0x23 => Instruction::from("INC IX", 2, 10, 0),
                    0x24 => Instruction::from("INC IXH", 2, 8, 0),
                    0x25 => Instruction::from("DEC IXH", 2, 8, 0),
                    0x26 => Instruction::from("LD IXH", 3, 11, 0),
                    0x29 => Instruction::from("ADD IX, IX", 2, 15, 0),
                    0x2A => Instruction::from("LD IX, **", 4, 20, 0),
                    0x2B => Instruction::from("DEC IX", 2, 10, 0),
                    0x2C => Instruction::from("INC IXL", 2, 8, 0),
                    0x2D => Instruction::from("DEC IXL", 2, 8, 0),
                    0x2E => Instruction::from("LD IXL, *", 3, 11, 0),

                    0x34 => Instruction::from("INC (IX+*)", 2, 23, 0),
                    0x35 => Instruction::from("DEC (IX+*)", 2, 23, 0),
                    0x36 => Instruction::from("LD, (IX+*), *", 4, 19, 0),
                    0x39 => Instruction::from("ADD IX, SP", 2, 15, 0),

                    0x44 => Instruction::from("LD B, IXH, *", 2, 8, 0),
                    0x45 => Instruction::from("LD B, IXL, *", 2, 8, 0),
                    0x46 => Instruction::from("LD B, (IX+*)", 3, 19, 0),
                    0x4C => Instruction::from("LD C, IXH", 2, 8, 0),
                    0x4D => Instruction::from("LD C, IXL)", 2, 8, 0),
                    0x4E => Instruction::from("LD C, (IX+*)", 3, 19, 0),

                    0x54 => Instruction::from("LD D, IXH, *", 2, 8, 0),
                    0x55 => Instruction::from("LD D, IXL, *", 2, 8, 0),
                    0x56 => Instruction::from("LD D, (IX+*)", 3, 19, 0),
                    0x5C => Instruction::from("LD E, IXH", 2, 8, 0),
                    0x5D => Instruction::from("LD E, IXL)", 2, 8, 0),
                    0x5E => Instruction::from("LD E, (IX+*)", 3, 19, 0),

                    0x60 => Instruction::from("LD IXH, B", 2, 8, 0),
                    0x61 => Instruction::from("LD IXH, C", 2, 8, 0),
                    0x62 => Instruction::from("LD IXH, D", 2, 8, 0),
                    0x63 => Instruction::from("LD IXH, E", 2, 8, 0),
                    0x64 => Instruction::from("LD IXH, IXH", 2, 8, 0),
                    0x65 => Instruction::from("LD IXH, IXL", 2, 8, 0),
                    0x66 => Instruction::from("LD IXH, (IX+*)", 3, 11, 0),
                    0x67 => Instruction::from("LD IXH, A", 2, 8, 0),
                    0x68 => Instruction::from("LD IXL,B", 2, 8, 0),
                    0x69 => Instruction::from("LD IXL,C", 2, 8, 0),
                    0x6A => Instruction::from("LD IXL,D", 2, 8, 0),
                    0x6B => Instruction::from("LD IXL,E", 2, 8, 0),
                    0x6C => Instruction::from("LD IXL,H", 2, 8, 0),
                    0x6D => Instruction::from("LD IXL,L", 2, 8, 0),
                    0x6E => Instruction::from("LD L (IX+*)", 3, 19, 0),
                    0x6F => Instruction::from("LD IXL,A", 2, 8, 0),

                    0x70 => Instruction::from("LD (IX+*), B", 3, 19, 0),
                    0x71 => Instruction::from("LD (IX+*), C", 3, 19, 0),
                    0x72 => Instruction::from("LD (IX+*), D", 3, 19, 0),
                    0x73 => Instruction::from("LD (IX+*), E", 3, 19, 0),
                    0x74 => Instruction::from("LD (IX+*), H", 3, 19, 0),
                    0x75 => Instruction::from("LD (IX+*), L", 3, 19, 0),
                    0x77 => Instruction::from("LD (IX+*), A", 3, 19, 0),
                    0x7C => Instruction::from("LD A, IXH", 2, 8, 0),
                    0x7D => Instruction::from("LD A IXL", 2, 8, 0),
                    0x7E => Instruction::from("LD A (IX+*)", 3, 19, 0),

                    0x84 => Instruction::from("ADD A, IXH", 2, 8, 0),
                    0x85 => Instruction::from("ADD A, IXL", 2, 8, 0),
                    0x86 => Instruction::from("ADD A, (IX+*)", 3, 19, 0),
                    0x8C => Instruction::from("ADC A, IXH", 2, 8, 0),
                    0x8D => Instruction::from("ADC A IXL", 2, 8, 0),
                    0x8E => Instruction::from("ADC A (IX+*)", 3, 19, 0),

                    0x94 => Instruction::from("SUB IXH", 2, 8, 0),
                    0x95 => Instruction::from("SUB IXL", 2, 8, 0),
                    0x96 => Instruction::from("SUB (IX+*) ", 3, 19, 0),
                    0x9C => Instruction::from("SBC A, IXH ", 2, 8, 0),
                    0x9D => Instruction::from("SBC A, IXL", 2, 8, 0),
                    0x9E => Instruction::from("SBC A, (IX+*)", 3, 19, 0),

                    0xA4 => Instruction::from("AND IXH", 2, 8, 0),
                    0xA5 => Instruction::from("AND IXL", 2, 8, 0),
                    0xA6 => Instruction::from("AND (IX+*) ", 3, 19, 0),
                    0xAC => Instruction::from("XOR IXH ", 2, 8, 0),
                    0xAD => Instruction::from("XOR, IXL", 2, 8, 0),
                    0xAE => Instruction::from("XOR (IX+*)", 3, 19, 0),

                    0xB4 => Instruction::from("OR IXH", 2, 8, 0),
                    0xB5 => Instruction::from("OR IXL", 2, 8, 0),
                    0xB6 => Instruction::from("OR (IX+*) ", 3, 19, 0),
                    0xBC => Instruction::from("CP IXH ", 2, 8, 0),
                    0xBD => Instruction::from("CP IXL", 2, 8, 0),
                    0xBE => Instruction::from("CP (IX+*)", 3, 19, 0),
                    // TODO IX BITS 0xDDCB

                    0xE1 => Instruction::from("POP IX", 2, 14, 0),
                    0xE3 => Instruction::from("EX (SP), IX", 2, 23, 0),
                    0xE5 => Instruction::from("PUSH IX", 2, 15, 0),
                    0xE9 => Instruction::from("JP (IX)", 2, 8, 0),
                    0xF9 => Instruction::from("LD SP, IX", 2, 10, 0),
                    _ => unimplemented!("IX Instruction not implemented"),
                }
            },
            0xDE => Instruction::from("SBC A,*", 2, 7, 0),
            0xDF => Instruction::from("RST 18H", 1, 11, 0),

            0xE0 => Instruction::from("RET PO", 1, 11, 5),
            0xE1 => Instruction::from("POP HL", 1, 10, 0),
            0xE2 => Instruction::from("JP PO, **", 3, 10, 0),
            0xE3 => Instruction::from("EX (SP), HL", 1, 19, 0),
            0xE4 => Instruction::from("CALL PO, **", 3, 17, 10),
            0xE5 => Instruction::from("PUSH HL", 1, 11, 0),
            0xE6 => Instruction::from("AND *", 2, 7, 0),
            0xE7 => Instruction::from("RST 20H", 1, 11, 0),
            0xE8 => Instruction::from("RET PE", 1, 11, 5),
            0xE9 => Instruction::from("JP (HL)", 1, 4, 0),
            0xEA => Instruction::from("JP PE, **", 3, 10, 0),
            0xEB => Instruction::from("EX DE, HL", 1, 4, 0),
            0xEC => Instruction::from("CALL PE, **", 3, 17, 10),
            // TODO Extended instructions
            0xED => match opcode {
                0xb0 => Instruction::from("LDIR", 2, 21, 16),
                0x0073 => Instruction::from("LD (**), HL", 4, 20, 0),
                _ => Instruction::from(&*format!("ED unknown opcode:{:02x}", opcode), 1, 1, 0),
            },
            0xEE => Instruction::from("XOR *", 2, 7, 0),
            0xEF => Instruction::from("RST 28H", 1, 11, 0),
            0xF0 => Instruction::from("RET P", 1, 11, 5),
            0xF1 => Instruction::from("POP AF", 1, 10, 0),
            0xF2 => Instruction::from("JP P, **", 3, 10, 0),
            0xF3 => Instruction::from("DI", 1, 4, 0),
            0xF4 => Instruction::from("CALL P, **", 3, 17, 10),
            0xF5 => Instruction::from("PUSH AF", 1, 11, 0),
            0xF6 => Instruction::from("OR *", 2, 7, 0),
            0xF7 => Instruction::from("RST 30H", 1, 11, 0),
            0xF8 => Instruction::from("RET M", 1, 11, 5),
            0xF9 => Instruction::from("LD SP, HL", 1, 6, 0),
            0xFA => Instruction::from("JP M, **", 3, 10, 0),
            0xFB => Instruction::from("EI", 1, 4, 0),
            0xFC => Instruction::from("CALL M, **", 3, 17, 10),
            0xFD => unimplemented!("IY instructions are not implemented"),
            0xFE => Instruction::from("CP *", 2, 7, 0),
            0xFF => Instruction::from("RST 38H", 1, 11, 0),
            _ => Instruction {
                name: format!("Unknown opcode:{:04x}", opcode),
                size: 0,
                cycles: 0,
                alt_cycles: 0
            }
        }
    }
}


    /*pub fn print(opcode: u8) -> &'static str {
        match opcode {
            0x00 | 0x08 => "NOP       ",
            0x01 => "LXI
            BC    ",
            0x02 => "STAX
            BC   ",
            0x03 => "INX
            BC    ",
            0x04 => "INR
            B     ",
            0x05 => "DCR
            B     ",
            0x06 => "MVI
            B     ",
            0x07 => "RLC
            ",
            0x09 => "DAD BC
            ",

            0x0A => "LDAX BC
            ",
            0x0B => "DCX BC
            ",
            0x0C => "INR C
            ",
            0x0D => "DCR C
            ",
            0x0E => "MVI C
            ",
            0x0F => "RRC       ",

            0x10 => "NOP
            ",
            0x11 => "LXI DE
            ",
            0x12 => "STAX DE
            ",
            0x13 => "INX DE
            ",
            0x14 => "INR D
            ",
            0x15 => "DCR D
            ",
            0x16 => "MVI D
            ",

            0x17 => "RAL       ",
            0x18 => "NOP
            ",
            0x19 => "DAD DE
            ",
            0x1A => "LDAX DE
            ",
            0x1B => "DCX DE
            ",
            0x1C => "INR E
            ",
            0x1D => "DCR E
            ",
            0x1E => "MVI E
            ",
            0x1F => "RAR       ",
            0x20 => "NOP
            ",
            0x21 => "LXI HL
            ",
            0x22 => "SHLD      ",
            0x23 => "INX
            HL    ",
            0x24 => "INR
            H     ",
            0x25 => "DCR
            H     ",
            0x26 => "MVI
            H     ",
            0x27 => "DAA
            ",
            0x28 => "NOP       ",
            0x29 => "DAD
            HL    ",

            0x2A => "LHLD
            ",
            0x2B => "DCX HL
            ",
            0x2C => "INR L
            ",
            0x2D => "DCR L
            ",
            0x2E => "MVI L
            ",
            0x2F => "CMA       ",

            0x30 => "NOP
            ",
            0x31 => "LXI SP
            ",
            0x32 => "STA       ",
            0x33 => "INX
            SP    ",
            0x34 => "INR
            M     ",
            0x35 => "DCR
            M     ",
            0x36 => "MVI
            M     ",
            0x37 => "STC
            ",
            0x38 => "NOP       ",
            0x39 => "DAD
            SP    ",

            0x3A => "LDA
            ",
            0x3B => "DCX SP
            ",
            0x3C => "INR A
            ",
            0x3D => "DCR A
            ",
            0x3E => "MVI A
            ",
            0x3F => "CMC       ",

            // MOV Instructions 0x40 - 0x7F
            0x40 => "MOV(B, B)
            ",
            0x41 => "MOV (B, C)
            ",
            0x42 => "MOV (B, D)
            ",
            0x43 => "MOV (B, E)
            ",
            0x44 => "MOV (B, H)
            ",
            0x45 => "MOV (B, L)
            ",
            0x46 => "MOV (B, M)
            ",
            0x47 => "MOV (B, A)
            ",

            0x48 => "MOV (C, B)
            ",
            0x49 => "MOV (C, C)
            ",
            0x4A => "MOV (C, D)
            ",
            0x4B => "MOV (C, E)
            ",
            0x4C => "MOV (C, H)
            ",
            0x4D => "MOV (C, L)
            ",
            0x4E => "MOV (C, M)
            ",
            0x4F => "MOV (C, A)
            ",

            0x50 => "MOV (D, B)
            ",
            0x51 => "MOV (D, C)
            ",
            0x52 => "MOV (D, D)
            ",
            0x53 => "MOV (D, E)
            ",
            0x54 => "MOV (D, H)
            ",
            0x55 => "MOV (D, L)
            ",
            0x56 => "MOV (D, M)
            ",
            0x57 => "MOV (D, A)
            ",

            0x58 => "MOV (E, B)
            ",
            0x59 => "MOV (E, C)
            ",
            0x5A => "MOV (E, D)
            ",
            0x5B => "MOV (E, E)
            ",
            0x5C => "MOV (E, H)
            ",
            0x5D => "MOV (E, L)
            ",
            0x5E => "MOV (E, M)
            ",
            0x5F => "MOV (E, A)
            ",

            0x60 => "MOV (H, B)
            ",
            0x61 => "MOV (H, C)
            ",
            0x62 => "MOV (H, D)
            ",
            0x63 => "MOV (H, E)
            ",
            0x64 => "MOV (H, H)
            ",
            0x65 => "MOV (H, L)
            ",
            0x66 => "MOV (H, M)
            ",
            0x67 => "MOV (H, A)
            ",

            0x68 => "MOV (L, B)
            ",
            0x69 => "MOV (L, C)
            ",
            0x6A => "MOV (L, D)
            ",
            0x6B => "MOV (L, E)
            ",
            0x6C => "MOV (L, H)
            ",
            0x6D => "MOV (L, L)
            ",
            0x6E => "MOV (L, M)
            ",
            0x6F => "MOV (L, A)
            ",

            0x70 => "MOV (M, B)
            ",
            0x71 => "MOV (M, C)
            ",
            0x72 => "MOV (M, D)
            ",
            0x73 => "MOV (M, E)
            ",
            0x74 => "MOV (M, H)
            ",
            0x75 => "MOV (M, L)
            ",
            0x76 => "HLT       ",
            0x77 => "MOV(M, A)
            ",

            0x78 => "MOV (A, B)
            ",
            0x79 => "MOV (A, C)
            ",
            0x7A => "MOV (A, D)
            ",
            0x7B => "MOV (A, E)
            ",
            0x7C => "MOV (A, H)
            ",
            0x7D => "MOV (A, L)
            ",
            0x7E => "MOV (A, M)
            ",
            0x7F => "MOV (A, A)
            ",

            // ADD Instructions
            0x80 => "ADD B
            ",
            0x81 => "ADD C
            ",
            0x82 => "ADD D
            ",
            0x83 => "ADD E
            ",
            0x84 => "ADD H
            ",
            0x85 => "ADD L
            ",
            0x86 => "ADD M
            ",
            0x87 => "ADD A
            ",

            0x88 => "ADC B
            ",
            0x89 => "ADC C
            ",
            0x8A => "ADC D
            ",
            0x8B => "ADC E
            ",
            0x8C => "ADC H
            ",
            0x8D => "ADC L
            ",
            0x8E => "ADC M
            ",
            0x8F => "ADC A
            ",

            // SUB Instructions
            0x90 => "SUB B
            ",
            0x91 => "SUB C
            ",
            0x92 => "SUB D
            ",
            0x93 => "SUB E
            ",
            0x94 => "SUB H
            ",
            0x95 => "SUB L
            ",
            0x96 => "SUB M
            ",
            0x97 => "SUB A
            ",

            0x98 => "SBB B
            ",
            0x99 => "SBB C
            ",
            0x9A => "SBB D
            ",
            0x9B => "SBB E
            ",
            0x9C => "SBB H
            ",
            0x9D => "SBB L
            ",
            0x9E => "SBB M
            ",
            0x9F => "SBB A
            ",

            // ANA
            0xA0 => "ANA B
            ",
            0xA1 => "ANA C
            ",
            0xA2 => "ANA D
            ",
            0xA3 => "ANA E
            ",
            0xA4 => "ANA H
            ",
            0xA5 => "ANA L
            ",
            0xA6 => "ANA M
            ",
            0xA7 => "ANA A
            ",

            // XRA
            0xA8 => "XRA B
            ",
            0xA9 => "XRA C
            ",
            0xAA => "XRA D
            ",
            0xAB => "XRA E
            ",
            0xAC => "XRA H
            ",
            0xAD => "XRA L
            ",
            0xAE => "XRA M
            ",
            0xAF => "XRA A
            ",

            // ORA Instructions  0xB(reg)
            0xB0 => "ORA B
            ",
            0xB1 => "ORA C
            ",
            0xB2 => "ORA D
            ",
            0xB3 => "ORA E
            ",
            0xB4 => "ORA H
            ",
            0xB5 => "ORA L
            ",
            0xB6 => "ORA M
            ",
            0xB7 => "ORA A
            ",

            // CMP
            0xB8 => "CMP B
            ",
            0xB9 => "CMP C
            ",
            0xBA => "CMP D
            ",
            0xBB => "CMP E
            ",
            0xBC => "CMP H
            ",
            0xBD => "CMP L
            ",
            0xBE => "CMP M
            ",
            0xBF => "CMP A
            ",

            0xC0 => "RNZ       ",
            0xC1 => "POP
            BC    ",
            0xC2 => "JNZ
            ",
            0xC3 => "JMP       ",
            0xC4 => "CNZ
            ",
            0xC5 => "PUSH B
            ",
            0xC6 => "ADI       ",
            0xC7 => "RST
            0     ",
            0xC8 => "RZ
            ",
            0xC9 => "RET       ",

            0xCA => "JZ
            ",
            0xCB => "JMP       ",
            0xCC => "CZ
            ",
            0xCD => "CALL      ",
            0xCE => "ACI
            ",
            0xCF => "RST 1
            ",

            0xD0 => "RNC       ",
            0xD1 => "POP
            DE    ",
            0xD2 => "JNC
            ",
            0xD3 => "OUT       ",
            0xD4 => "CNC
            ",
            0xD5 => "PUSH D
            ",
            0xD6 => "SUI       ",
            0xD7 => "RST
            2     ",
            0xD8 => "RC
            ",
            0xD9 => "RET       ",

            0xDA => "JC
            ",
            0xDB => "IN        ",
            0xDC => "CC
            ",
            0xDD => "CALL      ",
            0xDE => "SBI
            ",
            0xDF => "RST 3
            ",

            0xE0 => "RPO       ",
            0xE1 => "POP
            HL    ",
            0xE2 => "JPO
            ",
            0xE3 => "XTHL      ",
            0xE4 => "CPO
            ",
            0xE5 => "PUSH H
            ",
            0xE6 => "ANI       ",
            0xE7 => "RST
            4     ",
            0xE8 => "RPE
            ",
            0xE9 => "PCHL      ",

            0xEA => "JPE
            ",
            0xEB => "XCHG      ",
            0xEC => "CPE
            ",
            0xED => "CALL      ",
            0xEE => "XRI
            ",
            0xEF => "RST 5
            ",

            0xF0 => "RP        ",
            0xF1 => "POP
            PSW   ",
            0xF2 => "JP
            ",
            0xF3 => "DI        ",
            0xF4 => "CP ",
            0xF5 => "PUSH PSW",
            0xF6 => "ORI       ",
            0xF7 => "RST
            6     ",
            0xF8 => "RM
            ",
            0xF9 => "SPHL      ",

            0xFA => "JM
            ",
            0xFB => "EI        ",
            0xFC => "CM
            ",
            0xFD => "CALL      ",
            0xFE => "CPI
            ",
            0xFF => "RST 7
            ",
            _ => ("Unknown opcode
            "),
        }
    }
}*/
