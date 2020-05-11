use crate::instructions::{Register, Instruction};
use crate::memory::Memory;
use crate::util::ReadWrite;
use std::fmt;
use std::ops::Add;

pub struct Io {
    pub port_0_in: u8,
    pub port_1_in: u8,
    pub port_2_in: u8,
    pub port_3_in: u8,
    pub port_2_out: u8,
    pub port_3_out: u8,
    pub port_4_out_high: u8,
    pub port_4_out_low: u8,
    pub shift_offset: u8,
    pub port_5_out: u8,
    pub port_6_out: u8,
}
impl Io {
    pub fn new() -> Self {
        Self {
            port_0_in: 0b10101100,
            port_1_in: 0,
            port_2_in: 0,
            port_3_in: 0,
            port_2_out: 0,
            port_3_out: 0,
            port_4_out_high: 0,
            port_4_out_low: 0,
            shift_offset: 0,
            port_5_out: 0,
            port_6_out: 0,
        }
    }
}
#[derive(Default)]
pub struct Flags {
    pub parity: bool,
    pub carry: bool,
    pub half_carry: bool,
    pub cycles: usize,
    pub interrupt: bool,
    pub sign: bool,
    pub zero: bool,
}
pub struct CPU {
    pub current_instruction: String,
    pub opcode: u16,
    pub breakpoint: bool,
    pub debug: bool,
    pub reg: Registers,
    pub flags: Flags,
    pub cycles: usize,
    pub interrupt_addr: u16,
    pub io: Io,
    pub instruction: Instruction,
    pub memory: Memory,
}

#[derive(Default)]
pub struct Registers {
    // Main Registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    // Alternate registers:
    pub m: u8,
    pub i: u8, // Interrupt vector
    pub r: u8, // Refresh counter

    pub pc: u16,
    pub prev_pc: u16,

    // Index Registers:
    pub sp: u16,
    pub ix: u16,
    pub iy: u16,
}

impl CPU  {
    pub fn new() -> CPU  {
        CPU  {
            opcode: 0,
            reg: Registers::default(),
            flags: Flags::default(),
            cycles: 0,
            current_instruction: String::new(),
            interrupt_addr: 0x08,
            debug: false,
            breakpoint: false,
            io: Io::new(),
            instruction: Instruction::new(),
            memory: Memory::new(),
        }
    }

    fn set_sp(&mut self, byte: u16) {
        self.reg.sp = byte;
    }

    fn read_reg(&self, reg: Register) -> u8 {
        use Register::*;
        match reg {
            A => self.reg.a,
            B => self.reg.b,
            C => self.reg.c,
            D => self.reg.d,
            E => self.reg.e,
            H => self.reg.h,
            L => self.reg.l,
            M => self.reg.m,
            I => self.reg.i,
            R => self.reg.r,
            // TODO: Handle IXH & IXL here.
            IX => (self.reg.ix & 0xff) as u8,
            IY => (self.reg.iy & 0xff) as u8,
            _ => panic!("Reading register pairs are not supported by read_reg"),
        }
    }

    fn write_reg(&mut self, reg: Register, value: u8) {
        use Register::*;
        match reg {
            A => self.reg.a = value,
            B => self.reg.b = value,
            C => self.reg.c = value,
            D => self.reg.d = value,
            E => self.reg.e = value,
            H => self.reg.h = value,
            L => self.reg.l = value,
            M => self.reg.m = value,
            I => self.reg.i = value,
            R => self.reg.r = value,
            IX => unimplemented!(),
            IY => unimplemented!(),
            _ => panic!("Reading register pairs are not supported by write_reg"),
        }
    }
    fn write_reg_pair(&mut self, reg: Register, word: u16) {
        use Register::*;
        match reg {
            DE => {
                self.write_reg(D, self.memory.read(word + 1) as u8);
                self.write_reg(E, self.memory.read(word) as u8);
            }
            BC => {
                self.write_reg(B, self.memory.read(word + 1) as u8);
                self.write_reg(C, self.memory.read(word) as u8);
            }
            HL => {
                self.write_reg(H, self.memory.read(word + 1) as u8);
                self.write_reg(L, self.memory.read(word) as u8);
            }
            SP => self.set_sp(word),
            IX => self.reg.ix = word,
            IY => self.reg.iy = word,
            _ => panic!("Attemping to write to a non register pair"),
        }
    }
    fn get_pair(&self, reg: Register) -> u16 {
        use Register::*;

        return match reg {
            BC => (self.reg.b as u16) << 8 | (self.reg.c as u16),
            DE => (self.reg.d as u16) << 8 | (self.reg.e as u16),
            HL => (self.reg.h as u16) << 8 | (self.reg.l as u16),
            IX => self.reg.ix,
            IY => self.reg.iy,
            _ => unimplemented!("{:?}", reg),
        };
    }
    fn adv_pc(&mut self, t: u16) {
        self.reg.prev_pc = self.reg.pc;
        self.reg.pc = self.reg.pc.wrapping_add(t);
    }

    fn adv_cycles(&mut self, t: usize) {
        self.cycles = self.cycles.wrapping_add(t);
    }

    fn adc(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };
        let result = (self.reg.a as u16)
            .wrapping_add(u16::from(value))
            .wrapping_add(self.flags.carry as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry =
            (self.reg.a & 0x0F) + (value as u8 & 0x0F) + (self.flags.carry as u8) > 0x0F;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Add Immediate to Accumulator with Carry
    fn aci(&mut self) {
        let value = self.memory.read(self.reg.pc + 1) as u16;

        // Add immediate with accumulator + carry flag value
        let reg_a = self.reg.a;
        let carry = self.flags.carry as u8;
        let result = (value)
            .wrapping_add(reg_a as u16)
            .wrapping_add(carry as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (reg_a & 0x0F) + (value as u8 & 0x0F) + (carry & 0x0F) > 0x0F;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn add(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };

        let result = (self.reg.a as u16).wrapping_add(value as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as u8 & 0x0F) + (value as u8 & 0x0F) > 0x0F;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Add Immediate to Accumulator
    fn adi(&mut self) {
        // Read next byte of immediate data (low).
        let value = self.memory.read(self.reg.pc + 1) as u16;
        let result = (value).wrapping_add(self.reg.a as u16);

        // Set CPU flags with new accumulator values
        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.half_carry = (self.reg.a as u8 & 0x0F) + (value as u8 & 0x0F) > 0x0F;
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;
        self.adv_cycles(7);
        self.adv_pc(2);
    }

    pub fn ana(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };
        // And value with accumulator
        let result = self.reg.a as u16 & value as u16;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = ((self.reg.a | value) & 0x08) != 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = false;
        self.reg.a = result as u8;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ani(&mut self) {
        // The byte of immediate data is ANDed with the contents of the accumulator
        // The Carry bit is reset to zero.
        // Set half carry if the accumulator or opcode and the lower 4 bits are 1.

        let value = self.memory.read(self.reg.pc + 1);
        let result = self.reg.a as u16 & value as u16;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = ((self.reg.a | value as u8) & 0x08) != 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = false;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn djnz(&mut self) {
        // The b register is decremented, and if not zero the signed value * is added to PC
        // The jump is measured fro the start of the last instruction opcode
        self.reg.prev_pc = self.reg.pc;
        self.reg.b = self.reg.b.wrapping_sub(1);
        if self.reg.b == 0 {
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
            self.adv_cycles(13);
        } else  {
            self.adv_pc(1);
            self.adv_cycles(8);
        }
    }
    fn jr(&mut self) {
        self.reg.prev_pc = self.reg.pc;
        self.reg.pc = self.memory.read(self.reg.pc + 1);
    }
    fn jmp(&mut self) {
        self.reg.prev_pc = self.reg.pc;
        self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        self.adv_cycles(10);
    }

    // Jump if carry
    fn jc(&mut self) {
        if self.flags.carry {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if no carry
    fn jnc(&mut self) {
        if !self.flags.carry {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Zero (If zero bit is 1)
    fn jz(&mut self) {
        if self.flags.zero {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Not Zero (if zero bit is 0 jump)
    fn jnz(&mut self) {
        if !self.flags.zero {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Minus (If sign bit is one)
    fn jm(&mut self) {
        if self.flags.sign {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Positive (If sign bit is zero)
    fn jp(&mut self) {
        if !self.flags.sign {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // If parity even (If parity bit is 1)
    fn jpe(&mut self) {
        if self.flags.parity {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // If parity odd (If parity bit is 0)
    fn jpo(&mut self) {
        if !self.flags.parity {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump to address in H:L
    fn pchl(&mut self) {
        self.adv_cycles(5);
        self.reg.prev_pc = self.reg.pc;
        self.reg.pc = self.get_pair(Register::HL) as u16;
    }

    // 0xEDA0 Extended instruction
    fn ldi(&mut self) {
        unimplemented!()
    }

    // 0xEDB0 Extended instruction
    // TODO
    fn ldir(&mut self) {
        use Register::*;
        // Transfers a byte of data from the memory location pointed to by HL
        // to the memory location pointed to by DE.
        // Then HL and DE are incremented and BBC is decremented.
        // If BC is not zero this operation is repeated.
        // Interrupts can trigger while this instruction is happening.
        let hl = self.memory.read_word(self.get_pair(HL));
        let de = self.memory.read_word(self.get_pair(DE));
        self.memory.write_byte(de as u16, hl as u8);
        self.adv_cycles(21);
        self.adv_pc(2);


    }
    // Load Register Pair Immediate
    // LXI H, 2000H (2000H is stored in HL & acts as as memory pointer)
    fn lxi(&mut self, reg: Register) {
        use Register::*;

        let hb = self.memory.read_hb(self.reg.pc);
        let lb = self.memory.read_lb(self.reg.pc);

        match reg {
            BC => {
                self.reg.b = hb;
                self.reg.c = lb;
            } // self.write_reg(B, hb); self.write_reg(C, lb); }
            DE => {
                self.reg.d = hb;
                self.reg.e = lb;
            }
            HL => {
                self.memory.read_hb(self.reg.pc);
                self.reg.h = hb;
                self.reg.l = lb;
                // self.write_reg(H, hb);
                // self.write_reg(L, lb);
            }
            SP => self.reg.sp = self.memory.read_word(self.reg.pc + 1),
            IX => unimplemented!(),
            IY => unimplemented!(),
            _ => panic!("Attempted to load value on non register pair"),
        }
        self.adv_cycles(10);
        self.adv_pc(3);
    }

    // Store Accumulator direct
    fn sta(&mut self) {
        let imm = self.memory.read_word(self.reg.pc + 1);
        // Store value of the accumulator in in memory
        self.memory.write_byte(imm, self.reg.a);

        self.adv_cycles(13);
        self.adv_pc(3);
    }

    fn call(&mut self, opcode: u16) {
        // CALL instructions occupy three bytes. (See page 34 of the 8080 Programmers Manual)
        // CALL is just like JMP but also pushes a return address to stack.
        let ret: u16 = self.reg.pc.wrapping_add(3);
        match opcode {
            0xCC | 0xCD | 0xC4 | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFD | 0xFC => {
                // High order byte
                self.memory[self.reg.sp.wrapping_sub(1)] = (ret >> 8) as u8;
                // Low order byte
                self.memory[self.reg.sp.wrapping_sub(2)] = ret as u8;

                // Push return address to stack
                self.reg.sp = self.reg.sp.wrapping_sub(2);
            }
            _ => panic!(format!("Unknown call opcode: {:04x}", opcode)),
        };

        self.reg.prev_pc = self.reg.pc;

        self.reg.pc = self.memory.read_word(self.reg.pc + 1);
        self.adv_cycles(17);
    }

    // Call if Minus (if sign bit is 1, indicating a negative result)
    fn cm(&mut self, addr: u16) {
        if self.flags.sign {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn cc(&mut self, addr: u16) {
        if self.flags.carry {
            self.call(addr)
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    // Call if Zero
    fn cz(&mut self, addr: u16) {
        if self.flags.zero {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn cnz(&mut self, addr: u16) {
        if !self.flags.zero {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    // Call If No Carry
    fn cnc(&mut self, addr: u16) {
        if !self.flags.carry {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn cma(&mut self) {
        self.reg.a ^= 0xFF;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn cmc(&mut self) {
        self.flags.carry = !self.flags.carry;
        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn cmp(&mut self, reg: Register) {
        let value = match reg {
            Register::A => self.reg.a,
            Register::B => self.reg.b,
            Register::C => self.reg.c,
            Register::D => self.reg.d,
            Register::E => self.reg.e,
            Register::H => self.reg.h,
            Register::L => self.reg.l,
            Register::I => self.reg.i,
            Register::R => self.reg.r,
            Register::HL => {
                self.adv_cycles(3);
                self.memory[self.get_pair(Register::HL)]
            }
            _ => panic!("CMP not supported for register pairs"),
        };
        let result = (self.reg.a as u16).wrapping_sub(value as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Compare Immediate with Accumulator
    fn cpi(&mut self) {
        // Fetch byte out of memory which we will use to compare & set flags with.
        let value = self.memory.read(self.reg.pc + 1) as u16;
        // Compare the result of the accumulator with the immediate address.
        let result = (self.reg.a as u16).wrapping_sub(value as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // Call if Parity Even
    fn cpe(&mut self, addr: u16) {
        if self.flags.parity {
            self.reg.prev_pc = self.reg.pc;
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }
    // Call if Parity Odd
    fn cpo(&mut self, addr: u16) {
        if !self.flags.parity {
            self.reg.prev_pc = self.reg.pc;
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    // CALL if plus (if sign bit is zero, indicating a positive result)
    fn cp(&mut self, addr: u16) {
        if !self.flags.sign {
            self.reg.prev_pc = self.reg.pc;
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn dad(&mut self, reg: Register) {
        // Double precision ADD.
        // For these instructions, HL functions as an accumulator.
        // DAD B means BC + HL --> HL. DAD D means DE + HL -- HL.
        use Register::*;

        match reg {
            BC => {
                let result = (self.get_pair(HL) as u32).wrapping_add(self.get_pair(BC) as u32);

                self.reg.h = (result >> 8) as u8;
                self.reg.l = result as u8;
                self.flags.carry = (result & 0x10_000) != 0;
            }
            DE => {
                let result = (self.get_pair(HL) as u32).wrapping_add(self.get_pair(DE) as u32);

                self.reg.h = (result >> 8) as u8;
                self.reg.l = result as u8;
                self.flags.carry = (result & 0x10_000) != 0;
            }
            HL => {
                let result = (self.get_pair(HL) as u32).wrapping_add(self.get_pair(HL) as u32);

                self.reg.h = (result >> 8) as u8;
                self.reg.l = result as u8;
                self.flags.carry = (result & 0x10_000) != 0;
            }
            SP => {
                // let mut value = self.registers.sp as u32;
                let result = (self.get_pair(HL) as u32).wrapping_add(self.reg.sp as u32);

                self.reg.h = (result >> 8) as u8;
                self.reg.l = result as u8;
                self.flags.carry = (result & 0x10_000) != 0;
            }
            IX => unimplemented!(),
            IY => unimplemented!(),
            _ => panic!("Register: {:?} Not allowed for DAD", reg),
        }
        self.adv_cycles(10);
        self.adv_pc(1);
    }

    // TODO Refactor
    // Decrement memory or register
    fn dcr(&mut self, reg: Register) {
        use Register::*;
        // Example:
        // If the H register contains 3AH, and the L register contains 7CH
        // and memory location 3A7CH contains 40H, the instruction:
        // DCR M will cause memory location 3A7CH to contain 3FH.
        let value: u8 = match reg {
            A => {
                self.reg.a = self.reg.a.wrapping_sub(1);
                self.reg.a
            }
            B => {
                self.reg.b = self.reg.b.wrapping_sub(1);
                self.reg.b
            }
            C => {
                self.reg.c = self.reg.c.wrapping_sub(1);
                self.reg.c
            }
            D => {
                self.reg.d = self.reg.d.wrapping_sub(1);
                self.reg.d
            }
            E => {
                self.reg.e = self.reg.e.wrapping_sub(1);
                self.reg.e
            }
            H => {
                self.reg.h = self.reg.h.wrapping_sub(1);
                self.reg.h
            }
            L => {
                self.reg.l = self.reg.l.wrapping_sub(1);
                self.reg.l
            }
            I => {
                self.reg.i = self.reg.i.wrapping_sub(1);
                self.reg.i
            }
            R => {
                self.reg.r = self.reg.r.wrapping_sub(1);
                self.reg.r
            }
            M => {
                self.adv_cycles(5);
                let hl = self.get_pair(Register::HL);
                self.memory[hl] = self.memory[hl].wrapping_sub(1);
                self.memory[hl]
            }
            _ => panic!("DCR of register pair:{:?}, is not allowed", reg),
        };
        self.flags.sign = (value & 0x80) != 0;
        self.flags.zero = (value & 0xFF) == 0;
        self.flags.half_carry = !((value & 0x0F) == 0x0F);
        self.flags.parity = self.parity(value as u8);

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn dec_ex(&mut self, pair: Register) {
        use Register::*;
        match pair {
            BC => {
                let value = self.get_pair(BC).wrapping_sub(1);
                self.reg.b = (value >> 8) as u8;
                self.reg.c = value as u8;
                self.adv_cycles(1);
            }
            DE => {
                let value = self.get_pair(DE).wrapping_sub(1);
                self.reg.d = (value >> 8) as u8;
                self.reg.e = value as u8;
            }
            HL => {
                let value = self.get_pair(HL).wrapping_sub(1);
                self.reg.h = (value >> 8) as u8;
                self.reg.l = value as u8;
            }
            SP => self.reg.sp = self.reg.sp.wrapping_sub(1),
            IX => {
                let value = self.get_pair(IX).wrapping_sub(1);
                self.reg.i = (value >> 8) as u8;
                self.reg.r = value as u8;
            }
            IY => {
                let value = self.get_pair(IY).wrapping_sub(1);
                self.reg.i = (value >> 8) as u8;
                self.reg.r = value as u8;
            }
            _ => panic!("DCX of normal registers not supported"),
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // Double precision add
    fn daa(&mut self) {
        let mut add = if self.flags.half_carry || self.reg.a & 0x0F > 9 {
            0x06
        } else {
            0
        };

        if self.flags.carry
            || (self.reg.a >> 4) > 9
            || (self.reg.a >> 4) >= 9 && self.reg.a & 0x0F > 9
        {
            add |= 0x60;
            self.flags.carry = true;
        }

        let result = (self.reg.a as u16).wrapping_add(add as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as u8 & 0x0F) + (add as u8 & 0x0F) > 0x0F;
        self.flags.parity = self.parity(result as u8);

        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn di(&mut self) {
        self.flags.interrupt = false;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ei(&mut self) {
        self.flags.interrupt = true;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) {
        // The contents of the accumulator are rotated one bit position to the left.
        // The high-order bit of the accumulator replaces the carry bit while the carry bit
        // replaces the high-order bit of the accumulator
        // let carry = self.registers.carry as u8;
        // self.registers.reg_a = (self.registers.reg_a << 1) | carry;
        // self.registers.carry = (self.registers.reg_a & 0x80) != 0;

        let carry = (self.reg.a & 0x80) != 0;
        self.reg.a = (self.reg.a << 1) | (self.flags.carry as u8);
        self.flags.carry = carry;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        // If last bit is 1 bit shift one up so that the accumulator is 1
        let carry = (self.reg.a & 1) != 0;
        self.reg.a = (self.reg.a >> 1) | ((self.flags.carry as u8) << 7);
        self.flags.carry = carry;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 higher bits are 1 we set the carry flag.
        self.reg.a = self.reg.a.rotate_left(1);
        self.flags.carry = (self.reg.a & 1) != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn rrca(&mut self) {
        // The Carry bit is set equal to the low-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        self.reg.a = self.reg.a.rotate_right(1);
        self.flags.carry = (self.reg.a & 0x80) != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Return if no carry
    fn rnc(&mut self) {
        if !self.flags.carry {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if Parity Even
    fn rpe(&mut self) {
        if self.flags.parity {
            self.adv_cycles(1);
            self.ret()
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if Parity Odd
    fn rpo(&mut self) {
        if !self.flags.parity {
            self.adv_cycles(1);
            self.ret()
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if Carry
    fn rc(&mut self) {
        // If Carry flag is set, return from subroutine
        if self.flags.carry {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    fn rnz(&mut self) {
        if !self.flags.zero {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if minus
    fn rm(&mut self) {
        if self.flags.sign {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if positive (if sign bit is 0)
    fn rp(&mut self) {
        if !self.flags.sign {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    fn rz(&mut self) {
        if self.flags.zero {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }

    // Move Immediate Data
    fn mvi(&mut self, reg: Register) {
        // The MVI instruction uses a 8-bit data quantity, as opposed to
        // LXI which uses a 16-bit data quantity.
        let value = self.memory.read(self.reg.pc + 1) as u8;
        if reg != Register::HL {
            self.write_reg(reg, value);
        } else {
            self.adv_cycles(3);
            let hl = self.get_pair(Register::HL);
            self.memory[hl] = value;
        }
        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // LDA Load Accumulator direct
    fn lda(&mut self) {
        let imm = self.memory.read_word(self.reg.pc + 1);
        self.reg.a = self.memory[imm];
        self.adv_cycles(13);
        self.adv_pc(3);
    }

    // LD (Load extended registers)
    // TODO: Maybe consider consolidating in to one method?
    fn ld_ex(&mut self, reg: Register) {
        // The contents of the designated register pair point to a memory location.
        // This instruction copies the contents of that memory location into the
        // accumulator. The contents of either the register pair or the
        // memory location are not altered.
        use Register::*;

        match reg {
            BC => self.reg.a = self.memory[self.get_pair(BC)],
            DE => self.reg.a = self.memory[self.get_pair(DE)],
            HL => self.reg.a = self.memory[self.get_pair(HL)],
            IX => self.reg.a = self.memory[self.get_pair(IX)],
            IY => self.reg.a = self.memory[self.get_pair(IY)],
            _ => eprintln!("LD on invalid register: {:?}", reg),
        };

        self.adv_cycles(7);
        self.adv_pc(1);
    }

    fn lhld(&mut self) {
        // Load the HL register with 16 bits found at addr & addr + 1
        // The byte at the memory address formed by concatenating HI ADD with LOW ADD
        // replaces the contents of the L register.
        // The byte at the next higher memory address replaces the contents of the H
        // register.
        // L <- (adr); H<-(adr+1)
        let imm = self.memory.read_word(self.reg.pc + 1);
        self.reg.h = self.memory[imm + 1];
        self.reg.l = self.memory[imm];

        self.write_reg_pair(Register::HL, self.memory.read_word(self.reg.pc + 1));
        self.adv_cycles(16);
        self.adv_pc(3);
    }

    fn input(&mut self) {
        let port = self.memory.read(self.reg.pc + 1);

        let mut result: u8 = 0;
        match port {
            // Port 0 is not used
            1 => result = self.io.port_1_in,
            2 => result = self.io.port_2_in,
            3 => {
                let value =
                    ((self.io.port_4_out_high as u16) << 8) | (self.io.port_4_out_low as u16);
                result = ((value >> (8 - self.io.shift_offset) as u16) & 0xFF) as u8;
            }
            _ => eprintln!("Input port {}, not implemented", port),
        };

        if self.debug {
            println!("Input port: {}, Result: {:04X}", port, result);
        }
        self.reg.a = result as u8;
        self.adv_cycles(10);
        self.adv_pc(2);
    }

    fn inr(&mut self, reg: Register) {
        use Register::*;
        let value = match reg {
            A => {
                self.reg.a = self.reg.a.wrapping_add(1);
                self.reg.a
            }
            B => {
                self.reg.b = self.reg.b.wrapping_add(1);
                self.reg.b
            }
            C => {
                self.reg.c = self.reg.c.wrapping_add(1);
                self.reg.c
            }
            D => {
                self.reg.d = self.reg.d.wrapping_add(1);
                self.reg.d
            }
            E => {
                self.reg.e = self.reg.e.wrapping_add(1);
                self.reg.e
            }
            H => {
                self.reg.h = self.reg.h.wrapping_add(1);
                self.reg.h
            }
            L => {
                self.reg.l = self.reg.l.wrapping_add(1);
                self.reg.l
            }
            I => {
                self.reg.i = self.reg.i.wrapping_add(1);
                self.reg.i
            }
            R => {
                self.reg.r = self.reg.r.wrapping_add(1);
                self.reg.r
            }
            HL => {
                self.adv_cycles(5);
                let hl = self.get_pair(Register::HL);
                self.memory[hl] = self.memory[hl].wrapping_add(1);
                self.memory[hl]
            }

            // TODO?
            _ => unimplemented!("INR not implemetned for:{:?}", reg),
        };

        self.flags.sign = (value & 0x80) != 0;
        self.flags.zero = (value & 0xFF) == 0;
        self.flags.half_carry = (value & 0x0F) == 0x00;
        self.flags.parity = self.parity(value as u8);

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn inx(&mut self, reg: Register) {
        use Register::*;
        match reg {
            BC => {
                let value = self.get_pair(BC).wrapping_add(1);
                self.reg.b = (value >> 8) as u8;
                self.reg.c = (value) as u8;
            }
            DE => {
                let value = self.get_pair(DE).wrapping_add(1);
                self.reg.d = (value >> 8) as u8;
                self.reg.e = (value) as u8;
            }
            HL => {
                let value = self.get_pair(HL).wrapping_add(1);
                self.reg.h = (value >> 8) as u8;
                self.reg.l = (value) as u8;
            }
            SP => self.reg.sp = self.reg.sp.wrapping_add(1),
            IX => unimplemented!(),
            IY => unimplemented!(),
            // TODO
            _ => {}
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn push(&mut self, reg: Register) {
        use Register::*;
        self.reg.sp = self.reg.sp.wrapping_sub(2);

        match reg {
            Register::B => self.memory.write_word(self.reg.sp, self.get_pair(BC)),
            Register::D => self.memory.write_word(self.reg.sp, self.get_pair(DE)),
            Register::H => self.memory.write_word(self.reg.sp, self.get_pair(HL)),
            Register::L => unimplemented!(),
            Register::R => unimplemented!(),
            _ => eprintln!("Push on wrong register"),
        }

        self.adv_cycles(11);
        self.adv_pc(1);
    }

    fn push_psw(&mut self) {
        let psw = if self.flags.sign { 0x80 } else { 0x0 }
            | if self.flags.zero { 0x40 } else { 0x0 }
            | if self.flags.parity { 0x04 } else { 0x0 }
            | if self.flags.half_carry { 0x10 } else { 0x0 }
            | 0x02
            | if self.flags.carry { 0x01 } else { 0x0 };

        self.memory[self.reg.sp.wrapping_sub(1)] = self.reg.a;
        self.memory[self.reg.sp.wrapping_sub(2)] = psw;
        self.reg.sp = self.reg.sp.wrapping_sub(2);

        self.adv_cycles(11);
        self.adv_pc(1);
    }

    // Store the contents of the accumulator addressed by registers B, C
    // or by registers D and E.
    fn stax(&mut self, reg: Register) {
        use Register::*;

        match reg {
            BC => self.memory.write_byte(self.get_pair(BC), self.reg.a),
            DE => self.memory.write_byte(self.get_pair(DE), self.reg.a),
            HL => self.memory.write_byte(self.get_pair(HL), self.reg.a),
            IX => unimplemented!(),
            IY => unimplemented!(),
            SP => eprintln!("STAX should not run on SP register"),
            _ => unimplemented!("STAX not implemented for:{:?}", reg),
        }
        self.adv_cycles(7);
        self.adv_pc(1);
    }

    // SBB Subtract Register or Memory from Accumulator with borrow
    fn sbb(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };

        let result = (self.reg.a as u16)
            .wrapping_sub(value as u16)
            .wrapping_sub(self.flags.carry as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry =
            (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) - (self.flags.carry as i8) >= 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    // Subtract Immediate with Borrow
    fn sbi(&mut self) {
        let imm = self.memory.read(self.reg.pc + 1);
        let value = imm + self.flags.carry as u16;
        let result = (self.reg.a as u16).wrapping_sub(value);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // SUB Subtract Register or Memory From Accumulator
    fn sub(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };
        let result = (self.reg.a as u16).wrapping_sub(value as u16);

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // SUI Subtract Immediate From Accumulator
    fn sui(&mut self) {
        let value = self.memory.read(self.reg.pc + 1);
        let result = (self.reg.a as u16).wrapping_sub(value) as u16;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = (result & 0x0100) != 0;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // Set Carry (set carry bit to 1)
    fn stc(&mut self) {
        self.flags.carry = true;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // XRA Logical Exclusive-Or memory with Accumulator (Zero accumulator)
    fn xra(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };

        let result = self.reg.a as u16 ^ value as u16;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = false;
        self.flags.carry = false;
        self.flags.parity = self.parity(result as u8);
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // XRI Exclusive-Or Immediate with Accumulator
    fn xri(&mut self) {
        let imm = self.memory.read(self.reg.pc + 1);
        let result = self.reg.a ^ imm as u8;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = false;
        self.flags.parity = self.parity(result);
        self.flags.carry = false;
        self.reg.a = result;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn xchg(&mut self) {
        use std::mem;

        mem::swap(&mut self.reg.h, &mut self.reg.d);
        mem::swap(&mut self.reg.l, &mut self.reg.e);

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn xthl(&mut self) {
        // Swap H:L with top word on stack
        let hl = self.get_pair(Register::HL) as u16;
        let new_hl = self.memory.read_word(self.reg.sp);

        // Write old HL values to memory
        self.memory.write_word(self.reg.sp, hl);
        self.reg.h = (new_hl >> 8) as u8;
        self.reg.l = new_hl as u8;
        self.adv_cycles(18);
        self.adv_pc(1);
    }

    fn pop(&mut self, reg: Register) {
        let hb = self.memory.read_word(self.reg.sp + 1) as u8;
        let lb = self.memory.read_word(self.reg.sp) as u8;

        match reg {
            Register::BC => {
                self.reg.c = lb;
                self.reg.b = hb;
            }
            Register::DE => {
                self.reg.e = lb;
                self.reg.d = hb;
            }
            Register::HL => {
                self.reg.l = lb;
                self.reg.h = hb;
            }
            Register::SP => eprintln!("POP called on SP"),
            Register::IX => unimplemented!(),
            Register::IY => unimplemented!(),
            _ => panic!("Attempted to pop a single register"),
        }
        self.reg.sp = self.reg.sp.wrapping_add(2);

        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn pop_psw(&mut self) {
        let sp = self.reg.sp;
        self.flags.sign = (self.memory[sp] & 0x80) != 0;
        self.flags.zero = (self.memory[sp] & 0x40) != 0;
        self.flags.parity = (self.memory[sp] & 0x04) != 0;
        self.flags.half_carry = (self.memory[sp] & 0x10) != 0;
        self.flags.carry = (self.memory[sp] & 0x01) != 0;

        self.reg.a = self.memory.read(sp as u16 + 1) as u8;
        self.reg.sp = sp.wrapping_add(2) as u16;

        self.adv_cycles(10);
        self.adv_pc(1);
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.memory.read_word(self.reg.sp + 1);
        if self.debug {
            println!("Popping stack. SP value: {:04X}", sp);
        }
        self.reg.sp += 2;
        sp
    }

    fn ret(&mut self) {
        let low = self.memory[self.reg.sp];
        let high = self.memory[self.reg.sp.wrapping_add(1)];
        let ret: u16 = (high as u16) << 8 | (low as u16);
        // Set program counter for debug output
        self.reg.prev_pc = self.reg.pc;
        // println!("Returning to {:04X}", ret);
        self.reg.pc = ret as u16;

        self.reg.sp = self.reg.sp.wrapping_add(2);
        self.adv_cycles(10);
    }

    fn output(&mut self) {
        let port = self.memory.read(self.reg.pc + 1);

        match port {
            2 => {
                // Sets the offset size for shift reg
                self.io.shift_offset = self.reg.a & 0x7;
                // println!("Output Port 2: {:04X}", self.registers.port_2_out);
            }
            // Sound port
            3 => {
                self.io.port_3_out = self.reg.a;
                // println!("Output SND Port 3: {:04X}", self.registers.port_3_out);
            }

            // Sets shift register values respective of port 4 low & high values
            // I.e for both shift registers
            4 => {
                self.io.port_4_out_low = self.io.port_4_out_high;
                self.io.port_4_out_high = self.reg.a;
            }
            // Sound port
            5 => self.io.port_5_out = self.reg.a,

            // Watchdog port
            6 => {
                // On real hardware the watch dog system is a counter that counts 60Hz pulses
                // from the video divider chain. It was reset for each pulse, and would time out
                // after 4 seconds. The watchdog pin had to be low on boot for some games to start.

                // Assign accumulator value to port 6 output
                self.io.port_6_out = self.reg.a;

                // println!("Watchdog, value: {:04X}", self.registers.port_6_out);
            }
            7 => {
                // Debug port
                // self.registers.reg_a & 0x01;
            }
            _ => println!("Output port: {:04X}, does not match implementation", port),
        }
        self.adv_cycles(10);
        self.adv_pc(2);
    }
    fn ora(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };

        let result = self.reg.a as u16 | value as u16;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = false;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = false;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Or Immediate with Accumulator
    fn ori(&mut self) {
        let result = self.reg.a as u16 | self.memory.read(self.reg.pc + 1) as u16;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.zero = (result & 0xFF) == 0;
        self.flags.half_carry = false;
        self.flags.parity = self.parity(result as u8);
        self.flags.carry = false;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }
    // TODO Refactor
    fn ld(&mut self, dst: Register, src: Register) {
        use Register::*;
        let value = self.read_reg(src);
        let addr = self.get_pair(Register::HL) as u16;

        match dst {
            A => {
                if src == Register::HL { self.reg.a = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value)
                }
            }
            B => {
                if src == Register::HL {
                    self.reg.b = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            C => {
                if src == Register::HL {
                    self.reg.c = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            D => {
                if src == Register::HL {
                    self.reg.d = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            E => {
                if src == Register::HL {
                    self.reg.e = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            H => {
                if src == Register::HL {
                    self.reg.h = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            L => {
                if src == Register::HL {
                    self.reg.l = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            HL => {
                self.memory[addr] = self.read_reg(src);
                self.adv_cycles(2);
            }
            I => {
                if src == Register::HL {
                    self.reg.i = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
                unimplemented!();
            }
            R => {
                if src == Register::HL{
                    self.reg.r = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
                unimplemented!();
            }
            _ => {}
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // RESET (used for interrupt jump / calls)
    pub fn rst(&mut self, value: u8) {
        // Address to return to after interrupt is finished.
        let ret = self.reg.pc + 1;
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.memory.write_word(self.reg.sp, ret);
        self.reg.prev_pc = self.reg.pc;

        // self.memory.write_word(self.registers.sp - 1, (ret >> 8));
        // self.memory.write_word(self.registers.sp - 2, ret);
        // self.registers.sp = self.registers.sp.wrapping_sub(2);

        match value {
            0 => self.reg.pc = 0x0000,
            1 => self.reg.pc = 0x0008,
            2 => self.reg.pc = 0x0010,
            3 => self.reg.pc = 0x0018,
            4 => self.reg.pc = 0x0020,
            5 => self.reg.pc = 0x0028,
            6 => self.reg.pc = 0x0030,
            7 => self.reg.pc = 0x0038,
            _ => println!("Couldn't match RST value, {:04X}", value),
        };

        self.adv_cycles(11);
    }

    fn sphl(&mut self) {
        self.reg.sp = self.get_pair(Register::HL) as u16;
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // Store H & L direct
    fn shld(&mut self) {
        let addr = self.memory.read_word(self.reg.pc + 1);
        let hl = self.get_pair(Register::HL) as u16;
        self.memory.write_word(addr, hl);
        self.adv_cycles(16);
        self.adv_pc(3);
    }

    pub fn nop(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);

    }
    fn fetch_op(&mut self) -> u16 {
        let pc = self.reg.pc;
        let op = self.memory.read(self.reg.pc);
        self.adv_pc(1);
        op
    }

    #[allow(overflowing_literals)]
    pub fn execute_instruction(&mut self) {
        use self::Register::*;

        self.opcode = self.memory.read(self.reg.pc);
        // self.opcode = self.fetch_op();

        /*let instruction = Instruction::decode(self.opcode);
        let cycles = instruction.cycles;
        let alt_cycles = instruction.alt_cycles;
        let size = instruction.alt_cycles;*/
        // What if each instruction function just does the actual execution of the CPU
        // instruction and a separate function handles incrementing the PC and setting the cycles &
        // flags respectively independently of the instruction functions?
        // This way the functions themselves can be more agnostic?

        match self.opcode {
            0x00 | 0x08 | 0x28 | 0x38 => self.nop(),
            0x01 => self.lxi(BC,),
            0x02 => self.stax(BC),
            0x03 => self.inx(BC),
            0x04 => self.inr(B),
            0x05 => self.dcr(B),
            0x06 => self.mvi(B),
            0x07 => self.rlc(),
            0x09 => self.dad(BC),
            0x10 => self.djnz(),

            0x0A => self.ld_ex(BC),
            0x0B => self.dec_ex(BC),
            0x0C => self.inr(C),
            0x0D => self.dcr(C),
            0x0E => self.mvi(C),
            0x0F => self.rrca(),

            0x11 => self.lxi(DE),
            0x12 => self.stax(DE),
            0x13 => self.inx(DE),
            0x14 => self.inr(D),
            0x15 => self.dcr(D),
            0x16 => self.mvi(D),
            0x17 => self.ral(),
            0x18 => self.jr(),
            0x19 => self.dad(DE),

            0x1A => self.ld_ex(DE),
            0x1B => self.dec_ex(DE),
            0x1C => self.inr(E),
            0x1D => self.dcr(E),
            0x1E => self.mvi(E),
            0x1F => self.rar(),

            // TODO JR JNZ?
            0x20 => self.jnz(),
            0x21 => self.lxi(HL),
            0x22 => self.shld(),
            0x23 => self.inx(HL),
            0x24 => self.inr(H),
            0x25 => self.dcr(H),
            0x26 => self.mvi(H),
            0x27 => self.daa(),
            0x29 => self.dad(HL),

            0x2A => self.lhld(),
            0x2B => self.dec_ex(HL),
            0x2C => self.inr(L),
            0x2D => self.dcr(L),
            0x2E => self.mvi(L),
            0x2F => self.cma(),

            // TODO JR JNC
            0x30 => self.jnc(),
            0x31 => self.lxi(SP),
            0x32 => self.sta(),
            0x33 => self.inx(SP),
            0x34 => self.inr(HL),
            0x35 => self.dcr(HL),
            0x36 => self.mvi(HL),
            0x37 => self.stc(),
            0x39 => self.dad(SP),

            0x3A => self.lda(),
            0x3B => self.dec_ex(SP),
            0x3C => self.inr(A),
            0x3D => self.dcr(A),
            0x3E => self.mvi(A),
            0x3F => self.cmc(),

            // MOV Instructions 0x40 - 0x7F
            0x40 => self.ld(B, B),
            0x41 => self.ld(B, C),
            0x42 => self.ld(B, D),
            0x43 => self.ld(B, E),
            0x44 => self.ld(B, H),
            0x45 => self.ld(B, L),
            0x46 => self.ld(B, HL),
            0x47 => self.ld(B, A),

            0x48 => self.ld(C, B),
            0x49 => self.ld(C, C),
            0x4A => self.ld(C, D),
            0x4B => self.ld(C, E),
            0x4C => self.ld(C, H),
            0x4D => self.ld(C, L),
            0x4E => self.ld(C, HL),
            0x4F => self.ld(C, A),

            0x50 => self.ld(D, B),
            0x51 => self.ld(D, C),
            0x52 => self.ld(D, D),
            0x53 => self.ld(D, E),
            0x54 => self.ld(D, H),
            0x55 => self.ld(D, L),
            0x56 => self.ld(D, HL),
            0x57 => self.ld(D, A),

            0x58 => self.ld(E, B),
            0x59 => self.ld(E, C),
            0x5A => self.ld(E, D),
            0x5B => self.ld(E, E),
            0x5C => self.ld(E, H),
            0x5D => self.ld(E, L),
            0x5E => self.ld(E, HL),
            0x5F => self.ld(E, A),

            0x60 => self.ld(H, B),
            0x61 => self.ld(H, C),
            0x62 => self.ld(H, D),
            0x63 => self.ld(H, E),
            0x64 => self.ld(H, H),
            0x65 => self.ld(H, L),
            0x66 => self.ld(H, HL),
            0x67 => self.ld(H, A),

            0x68 => self.ld(L, B),
            0x69 => self.ld(L, C),
            0x6A => self.ld(L, D),
            0x6B => self.ld(L, E),
            0x6C => self.ld(L, H),
            0x6D => self.ld(L, L),
            0x6E => self.ld(L, HL),
            0x6F => self.ld(L, A),

            0x70 => self.ld(HL, B),
            0x71 => self.ld(HL, C),
            0x72 => self.ld(HL, D),
            0x73 => self.ld(HL, E),
            0x74 => self.ld(HL, H),
            0x75 => self.ld(HL, L),

            0x76 => self.hlt(),
            0x77 => self.ld(HL, A),

            0x78 => self.ld(A, B),
            0x79 => self.ld(A, C),
            0x7A => self.ld(A, D),
            0x7B => self.ld(A, E),
            0x7C => self.ld(A, H),
            0x7D => self.ld(A, L),
            // TODO: MOV (A,M) / LD(A,HL) works?
            0x7E => self.ld_ex(HL),
            0x7F => self.ld(A, A),

            // ADD Instructions
            0x80 => self.add(B),
            0x81 => self.add(C),
            0x82 => self.add(D),
            0x83 => self.add(E),
            0x84 => self.add(H),
            0x85 => self.add(L),
            0x86 => self.add(HL),
            0x87 => self.add(A),

            0x88 => self.adc(B),
            0x89 => self.adc(C),
            0x8A => self.adc(D),
            0x8B => self.adc(E),
            0x8C => self.adc(H),
            0x8D => self.adc(L),
            0x8E => self.adc(HL),
            0x8F => self.adc(A),

            // SUB Instructions
            0x90 => self.sub(B),
            0x91 => self.sub(C),
            0x92 => self.sub(D),
            0x93 => self.sub(E),
            0x94 => self.sub(H),
            0x95 => self.sub(L),
            0x96 => self.sub(HL),
            0x97 => self.sub(A),

            0x98 => self.sbb(B),
            0x99 => self.sbb(C),
            0x9A => self.sbb(D),
            0x9B => self.sbb(E),
            0x9C => self.sbb(H),
            0x9D => self.sbb(L),
            0x9E => self.sbb(HL),
            0x9F => self.sbb(A),

            // ANA
            0xA0 => self.ana(B),
            0xA1 => self.ana(C),
            0xA2 => self.ana(D),
            0xA3 => self.ana(E),
            0xA4 => self.ana(H),
            0xA5 => self.ana(L),
            0xA6 => self.ana(HL),
            0xA7 => self.ana(A),

            // XRA
            0xA8 => self.xra(B),
            0xA9 => self.xra(C),
            0xAA => self.xra(D),
            0xAB => self.xra(E),
            0xAC => self.xra(H),
            0xAD => self.xra(L),
            0xAE => self.xra(HL),
            0xAF => self.xra(A),

            // ORA Instructions  0xB(reg)
            0xB0 => self.ora(B),
            0xB1 => self.ora(C),
            0xB2 => self.ora(D),
            0xB3 => self.ora(E),
            0xB4 => self.ora(H),
            0xB5 => self.ora(L),
            0xB6 => self.ora(HL),
            0xB7 => self.ora(A),

            // CMP
            0xB8 => self.cmp(B),
            0xB9 => self.cmp(C),
            0xBA => self.cmp(D),
            0xBB => self.cmp(E),
            0xBC => self.cmp(H),
            0xBD => self.cmp(L),
            0xBE => self.cmp(HL),
            0xBF => self.cmp(A),

            0xC0 => self.rnz(),
            0xC1 => self.pop(BC),
            0xC2 => self.jnz(),
            0xC3 => self.jmp(),
            0xC4 => self.cnz(0xC4),
            0xC5 => self.push(B),
            0xC6 => self.adi(),
            0xC7 => self.rst(0),
            0xC8 => self.rz(),
            0xC9 => self.ret(),

            0xCA => self.jz(),
            0xCB => unimplemented!("CB prefixed BITS instructions not implemented"),// self.jmp(),
            0xCC => self.cz(0xCC),
            0xCD => self.call(0xCD),
            0xCE => self.aci(),
            0xCF => self.rst(1),

            0xD0 => self.rnc(),
            0xD1 => self.pop(DE),
            0xD2 => self.jnc(),
            0xD3 => self.output(),
            0xD4 => self.cnc(0xD4),
            0xD5 => self.push(D),
            0xD6 => self.sui(),
            0xD7 => self.rst(2),
            0xD8 => self.rc(),
            0xD9 => self.ret(),

            0xDA => self.jc(),
            0xDB => self.input(),
            0xDC => self.cc(0xDC),
            0xDD => unimplemented!("IX instructions not implemtende yet"),//self.call(0xDD),
            0xDE => self.sbi(),
            0xDF => self.rst(3),

            0xE0 => self.rpo(),
            0xE1 => self.pop(HL),
            0xE2 => self.jpo(),
            0xE3 => self.xthl(),
            0xE4 => self.cpo(0xE4),
            0xE5 => self.push(H),
            0xE6 => self.ani(),
            0xE7 => self.rst(4),
            0xE8 => self.rpe(),
            0xE9 => self.pchl(),

            0xEA => self.jpe(),
            0xEB => self.xchg(),
            0xEC => self.cpe(0xEC),
            0xED => {
                match self.memory.read_byte(self.reg.pc + 1) {
                    0x00B0 => self.ldir(),
                    0x0073 => self.ld_ex(HL), // TODO: LD (**i), HL
                    _ => unimplemented!("ED instruction not yet implemented"),

                }
            }

            0xEE => self.xri(),
            0xEF => self.rst(5),

            0xF0 => self.rp(),
            0xF1 => self.pop_psw(),
            0xF2 => self.jp(),
            0xF3 => self.di(),
            0xF4 => self.cp(0xF4),
            0xF5 => self.push_psw(),
            0xF6 => self.ori(),
            0xF7 => self.rst(4),
            0xF8 => self.rm(),
            0xF9 => self.sphl(),

            0xFA => self.jm(),
            0xFB => self.ei(),
            0xFC => self.cm(0xFC),
            0xFD => self.call(0xFD),
            0xFE => self.cpi(),
            0xFF => self.rst(7),

            _ => println!("Unknown opcode: {:04X}", self.opcode),
        }

        // Lookup opcode & get instruction name back for debugging purposes
        if Instruction::decode(self.opcode.into()).name.to_string().len() < 2 {
            self.current_instruction = format!("N:>12");
        } else {
            self.current_instruction = Instruction::decode(self.opcode.into()).name.to_string();
        }

    }
    pub fn execute_tests(&mut self) {
        self.execute_instruction();
        self.try_reset_cycles();
    }

    pub fn step(&mut self) {
        let mut cycles_executed: usize = 0;

        while cycles_executed <= 16666 * 2 {
            let start_cycles = self.cycles;
            self.execute_instruction();
            cycles_executed += self.cycles - start_cycles;
            self.try_interrupt();
        }
    }

    pub fn reset(&mut self) {
        println!("Resetting emulator");

        self.reg.a = 0;
        self.reg.b = 0;
        self.reg.c = 0;
        self.reg.d = 0;
        self.reg.e = 0;
        self.reg.h = 0;
        self.reg.l = 0;

        // Reset flag conditions
        self.flags.sign = false;
        self.flags.zero = false;
        self.flags.parity = false;
        self.flags.carry = false;
        self.flags.half_carry = false;

        self.adv_pc(1);
    }

    // TODO interrupt handle
    fn hlt(&mut self) {
        eprintln!("Halting CPU");
        self.adv_cycles(7);
        ::std::process::exit(1);
    }

    fn parity(&self, value: u8) -> bool {
        let mut bits: u8 = 0;
        for i in 0..8 {
            bits += (value >> i) & 1;
        }
        (bits & 1) == 0
    }
    fn emulate_interrupt(&mut self) {
        if self.flags.interrupt {
            let ret: u16 = self.reg.pc;
            self.reg.sp = self.reg.sp.wrapping_sub(2);
            self.memory.write_word(self.reg.sp, ret);

            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.interrupt_addr;
            self.flags.interrupt = false;
        }
    }
    pub fn try_reset_cycles(&mut self) {
        if self.cycles < 16_667 {
            return;
        } else {
            self.cycles = 0;
        }
    }

    // These interrupts are specific to Space Invaders
    pub fn try_interrupt(&mut self) {
        if self.cycles < 16_667 {
            return;
        }
        if self.interrupt_addr == 0x08 {
            self.cycles = 0;
            self.emulate_interrupt();
            self.interrupt_addr = 0x10;
        } else if self.interrupt_addr == 0x10 {
            self.cycles = 0;
            self.emulate_interrupt();
            self.interrupt_addr = 0x08;
        }
    }
}
