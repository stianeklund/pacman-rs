use crate::instruction_info::{Instruction, Register, Register::*};
use crate::memory::{Memory, MemoryRW};

// TODO Move macros out of CPU
// Helper macros to increment or decrement, and returned the assigned value.
// Assignments return () and we work around that with these macros.
// match reg {
//     A => { self.reg.a = self.reg.a.wrapping_add(1); self.reg.a },
// Becomes:
// match reg {
//     A => inc_ret!(self.reg.a)
#[macro_export]
macro_rules! inc_ret {
    ($reg:expr) => {{
        $reg = $reg.wrapping_add(1);
        $reg
    }};
}

macro_rules! dec_ret {
    ($reg:expr) => {{
        $reg = $reg.wrapping_sub(1);
        $reg
    }};
}
macro_rules! rlc_ret {
    ($reg:expr, $flag:expr) => {{
        $reg = ($reg << 1) | ($flag << 7);
        $reg
    }};
}
macro_rules! rrc_ret {
    ($reg:expr, $flag:expr) => {{
        $reg = ($reg >> 1) | ($flag << 7);
        $reg
    }};
}
#[derive(Default)]
pub struct Io {
    pub port0: u8,
    pub in0: u8,
    pub in1: u8,
}

#[derive(Default, Debug)]
pub struct Flags {
    pub sf: bool, // Sign
    pub zf: bool, // Zero
    pub yf: bool, // Copy of bit 4 of the result
    pub hf: bool, // Half carry (AC)
    pub xf: bool, // Copy of bit 5 of the result
    pub pf: bool, // Parity
    pub nf: bool, // Subtract. Set if the last operation was a subtraction
    pub cf: bool,

    // Shadow
    pub sf_: bool,
    pub zf_: bool,
    pub yf_: bool,
    pub hf_: bool,
    pub xf_: bool,
    pub pf_: bool,
    pub nf_: bool,
    pub cf_: bool,
}

// IFF1 determines whether interrupts are allowed.
// IFF2's value is copied to PF by LD,AI and LD A, R
// When an NMI occurs IFF1 is reset, IFF2 is left unchanged.
// http://z80.info/z80info.htm (see f)
#[derive(Default, Debug)]
pub struct Interrupt {
    pub irq: bool,
    pub nmi_pending: bool,
    pub nmi: bool,
    pub int: bool,
    pub iff1: bool,
    pub iff2: bool,
    pub mode: u8,
}

pub struct Cpu {
    pub current_instruction: String,
    pub opcode: u16,
    pub breakpoint: bool,
    pub debug: bool,
    pub reg: Registers,
    pub flags: Flags,
    pub cycles: usize, // CPU T states
    pub io: Io,
    pub int: Interrupt,
    pub instruction: Instruction,
    pub int_pending: bool,
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
    // Shadow registers
    pub a_: u8,
    pub b_: u8,
    pub c_: u8,
    pub d_: u8,
    pub e_: u8,
    pub h_: u8,
    pub l_: u8,

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

impl Registers {
    // Swaps the value between the normal registers and shadow registers
    fn swap(&mut self) {
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let e = self.e;
        let h = self.h;
        let l = self.l;

        self.b = self.b_;
        self.c = self.c_;
        self.d = self.d_;
        self.e = self.e_;
        self.h = self.h_;
        self.l = self.l_;

        self.b_ = b;
        self.c_ = c;
        self.d_ = d;
        self.e_ = e;
        self.h_ = h;
        self.l_ = l;
    }
}

impl Flags {
    fn new() -> Self {
        Self {
            sf: false,
            zf: false,
            yf: false,
            hf: false,
            xf: false,
            pf: false,
            nf: false,
            cf: false,
            sf_: false,
            zf_: false,
            yf_: false,
            hf_: false,
            xf_: false,
            pf_: false,
            nf_: false,
            cf_: false,
        }
    }
    pub(crate) fn get(&self) -> u8 {
        return if self.sf { 0x80 } else { 0x0 }
            | if self.zf { 0x40 } else { 0x0 }
            | if self.yf { 0x20 } else { 0x0 }
            | if self.hf { 0x10 } else { 0x0 }
            | if self.xf { 0x08 } else { 0x0 }
            | if self.pf { 0x04 } else { 0x0 }
            | if self.nf { 0x02 } else { 0x0 }
            | if self.cf { 0x01 } else { 0x0 };
    }
    pub fn set(&mut self, value: u8) {
        self.sf = value & 0x80 != 0;
        self.zf = value & 0x40 != 0;
        self.yf = value & 0x20 != 0;
        self.hf = value & 0x10 != 0;
        self.xf = value & 0x08 != 0;
        self.pf = value & 0x04 != 0;
        self.nf = value & 0x02 != 0;
        self.cf = value & 0x01 != 0;
    }
    pub(crate) fn get_shadow(&self) -> u8 {
        return if self.sf_ { 0x80 } else { 0x0 }
            | if self.zf_ { 0x40 } else { 0x0 }
            | if self.yf_ { 0x20 } else { 0x0 }
            | if self.hf_ { 0x10 } else { 0x0 }
            | if self.xf_ { 0x08 } else { 0x0 }
            | if self.pf_ { 0x04 } else { 0x0 }
            | if self.nf_ { 0x02 } else { 0x0 }
            | if self.cf_ { 0x01 } else { 0x0 };
    }

    pub fn set_shadow(&mut self, value: u8) {
        self.sf_ = value & 0x80 != 0;
        self.zf_ = value & 0x40 != 0;
        self.yf_ = value & 0x20 != 0;
        self.hf_ = value & 0x10 != 0;
        self.xf_ = value & 0x08 != 0;
        self.pf_ = value & 0x04 != 0;
        self.nf_ = value & 0x02 != 0;
        self.cf_ = value & 0x01 != 0;
    }

    fn swap(&mut self) {
        let f = self.get();
        self.set(self.get_shadow());
        self.set_shadow(f);
    }
}

impl MemoryRW for Cpu {
    fn read8(&self, addr: u16) -> u8 {
        self.memory[addr]
    }
    fn read16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read8(addr), self.read8(addr + 1)])
    }

    fn write16(&mut self, addr: u16, word: u16) {
        self.write8(addr, word as u8);
        self.write8(addr.wrapping_add(1), (word >> 8) as u8);
    }
    fn write8(&mut self, addr: u16, byte: u8) {
        self.memory[addr & 0xFFFF] = byte
    }
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            opcode: 0,
            reg: Registers::default(),
            flags: Flags::new(),
            cycles: 0,
            current_instruction: String::new(),
            debug: false,
            breakpoint: false,
            io: Io::default(),
            int: Interrupt::default(),
            int_pending: false,
            instruction: Instruction::new(),
            memory: Memory::new(),
        }
    }

    fn read_reg(&self, reg: Register) -> u8 {
        match reg {
            A => self.reg.a,
            B => self.reg.b,
            C => self.reg.c,
            D => self.reg.d,
            E => self.reg.e,
            H => self.reg.h,
            L => self.reg.l,
            M => self.reg.m,
            R => self.reg.r,
            IX => self.reg.ix as u8,
            IXH => (self.reg.ix >> 8) as u8,
            IXL => ((self.reg.ix as u8) & 0xff),
            IY => self.reg.iy as u8,
            IYH => (self.reg.iy >> 8) as u8,
            IYL => ((self.reg.iy as u8) & 0xff),
            // TODO Potential value loss here
            BC => self.get_pair(BC) as u8,
            DE => self.get_pair(DE) as u8,
            HL => self.get_pair(HL) as u8,
            _ => {
                println!(
                    "Called by:{}, Opcode:{:02X}",
                    self.current_instruction, self.opcode
                );
                eprintln!("Instruction:{:?}", Instruction::decode(self.opcode));
                panic!("Register not supported");
            }
        }
    }
    fn write_reg(&mut self, reg: Register, value: u8) {
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
            _ => panic!("Writing to register pairs is not supported by write_reg"),
        }
    }

    // Loads register pair with value at location
    fn write_pair_indirect(&mut self, reg: Register, word: u16) {
        match reg {
            DE => {
                self.write_reg(D, self.read8(word + 1) as u8);
                self.write_reg(E, self.read8(word) as u8);
            }
            BC => {
                self.write_reg(B, self.read8(word + 1) as u8);
                self.write_reg(C, self.read8(word) as u8);
            }
            HL => {
                self.write_reg(H, self.read8(word + 1) as u8);
                self.write_reg(L, self.read8(word) as u8);
            }
            SP => self.reg.sp = word,
            IX => self.reg.ix = word,
            IY => self.reg.iy = word,
            _ => panic!("Attempting to write to a non register pair"),
        }
    }

    // Loads register pair with direct value
    pub fn write_pair_direct(&mut self, reg: Register, value: u16) {
        match reg {
            DE => {
                self.reg.d = (value >> 8) as u8;
                self.reg.e = (value & 0xFF) as u8;
            }
            BC => {
                self.reg.b = (value >> 8) as u8;
                self.reg.c = (value & 0xFF) as u8;
            }
            HL => {
                self.reg.h = (value >> 8) as u8;
                self.reg.l = (value & 0xFF) as u8;
            }
            IX => self.reg.ix = value,
            IY => self.reg.iy = value,
            SP => self.reg.sp = value,
            _ => panic!("Attempting to write to a non register pair: {:#?}", reg),
        }
    }
    pub fn get_pair(&self, reg: Register) -> u16 {
        return match reg {
            BC => (self.reg.b as u16) << 8 | (self.reg.c as u16),
            DE => (self.reg.d as u16) << 8 | (self.reg.e as u16),
            HL => (self.reg.h as u16) << 8 | (self.reg.l as u16),
            IX => self.reg.ix,
            IY => self.reg.iy,
            SP => self.reg.sp,
            AF => (self.reg.a as u16) << 8 | (self.flags.get() as u16),
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

    // TODO refactor ADD / ADC instructions
    // pass value in from the caller and have one method for most of these
    fn adc(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else if reg == Register::HL {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        } else {
            unimplemented!();
        };
        let result: u16 = (self.reg.a as u16)
            .wrapping_add(value as u16)
            .wrapping_add(self.flags.cf as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_add(self.reg.a, value);
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = result & 0x0100 != 0;

        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn adc_hl(&mut self, reg: Register) {
        let mut result = match reg {
            BC => (self.get_pair(HL) as u32)
                .wrapping_add(self.get_pair(BC) as u32)
                .wrapping_add(self.flags.cf as u32),
            DE => (self.get_pair(HL) as u32)
                .wrapping_add(self.get_pair(DE) as u32)
                .wrapping_add(self.flags.cf as u32),
            HL => (self.get_pair(HL) as u32)
                .wrapping_add(self.get_pair(HL) as u32)
                .wrapping_add(self.flags.cf as u32),
            SP => (self.get_pair(HL) as u32).wrapping_add(self.reg.sp as u32),
            _ => panic!("Register: {:?} Not allowed for ADC HL", reg),
        };

        self.reg.h = (result >> 8) as u8;
        self.reg.l = result as u8;

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.pf = self.overflow(result.wrapping_sub(1) as u8, result as u8);
        self.flags.cf = ((result >> 8) & 0x0100) != 0;
        self.flags.hf = self.hf_add(self.reg.a, (result >> 8) as u8);
        self.flags.nf = false;
        self.flags.yf = (result >> 8) & 0x20 != 0;
        self.flags.xf = (result >> 8) & 0x08 != 0;

        self.adv_cycles(15);
        self.adv_pc(2);
    }

    // Add Immediate to Accumulator with Carry
    fn adc_im(&mut self) {
        let value = self.read8(self.reg.pc + 1) as u16;

        // Add immediate with accumulator + carry flag value
        let reg_a = self.reg.a;
        let carry = self.flags.cf as u8;
        let result = (value)
            .wrapping_add(reg_a as u16)
            .wrapping_add(carry as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_add(reg_a, value as u8);
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.nf = false;
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.cf = result & 0x0100 != 0;

        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn add_ex(&mut self, dst: Register, src: Register) {
        let result = (self.get_pair(dst)).wrapping_add(self.get_pair(src) as u16);
        self.write_pair_direct(dst, result);
        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_add(self.reg.a, self.get_pair(src) as u8);
        // self.flags.pf = self.carry(7, result as u16, value as u16, false) != self.carry(8, result as u16, value as u16, false);
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = result & 0x0100 != 0;
        self.adv_cycles(15);
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

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_add(self.reg.a, value);
        // self.flags.pf = self.carry(7, result as u16, value as u16, false) != self.carry(8, result as u16, value as u16, false);
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = result & 0x0100 != 0;

        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Add Immediate to Accumulator
    fn adi(&mut self) {
        // Read next byte of immediate data (low).
        let value = self.read8(self.reg.pc + 1) as u16;
        let result = (value).wrapping_add(self.reg.a as u16);

        // Set CPU flags with new accumulator values
        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        // self.flags.pf = self.carry(7, result as u16, value as u16, false) != self.carry(8, result as u16, value as u16, false);
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.nf = false;
        // self.flags.hf = (self.reg.a as u8 & 0x0F) + (value as u8 & 0x0F) > 0x0F;
        self.flags.hf = self.hf_add(self.reg.a, value as u8);
        self.flags.cf = result & 0x0100 != 0;

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
        let result = self.reg.a & value;

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.nf = false;
        self.flags.hf = true;
        self.flags.pf = self.parity(result);
        // self.flags.pf = self.carry(7, result as u16, value as u16, false) != self.carry(8, result as u16, value as u16, false);
        self.flags.cf = false;

        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ani(&mut self) {
        // The byte of immediate data is ANDed with the contents of the accumulator
        // The Carry bit is reset to zero.
        // Set half carry if the accumulator or opcode and the lower 4 bits are 1.

        let value = self.read8(self.reg.pc + 1);
        let result = self.reg.a as u16 & value as u16;

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.nf = false;
        self.flags.hf = true;
        self.flags.pf = self.parity(result as u8);
        self.flags.cf = false;

        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn djnz(&mut self) {
        // The b register is decremented, and if not zero the signed value * is added to PC
        // The jump is measured from the start of the last instruction opcode
        self.adv_cycles(1);
        dec_ret!(self.reg.b);
        self.jr_cond(self.reg.b != 0);
    }
    // TODO Clean up JR
    fn jr(&mut self, offset: i16) {
        self.adv_pc(2);
        self.reg.prev_pc = self.reg.pc;
        self.reg.pc = (self.reg.pc as i16 + offset) as u16;
        self.adv_cycles(12);
    }
    // "Generic" function for conditional JR operations
    fn jr_cond(&mut self, cond: bool) {
        // E.g if zero flag == 0 { JR + offset
        let byte = self.read8(self.reg.pc + 1) as i8;
        if cond {
            self.jr(byte as i16);
        } else {
            self.adv_cycles(7);
            self.adv_pc(2);
        }
    }
    fn jp(&mut self, addr: u16) {
        self.reg.prev_pc = self.reg.pc;
        self.adv_cycles(self.instruction.cycles as usize);
        self.reg.pc = addr;
    }
    fn jp_cond(&mut self, cond: bool) {
        if cond {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.read16(self.reg.pc + 1);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump to address in H:L
    fn pchl(&mut self) {
        self.adv_cycles(4);
        self.reg.prev_pc = self.reg.pc;
        self.reg.pc = self.get_pair(Register::HL) as u16;
    }

    // 0xEDA0 Extended instruction
    fn ldi(&mut self) {
        // YF and XF are copies of bit 1 of n and bit 3 of n respectively.
        let de = self.read8(self.get_pair(DE));
        let hl = self.read8(self.get_pair(HL));
        self.write8(de as u16, hl as u8);

        let n = hl + self.reg.a;

        self.write_pair_direct(HL, self.get_pair(HL).wrapping_add(1));
        self.write_pair_direct(DE, self.get_pair(DE).wrapping_add(1));
        self.write_pair_direct(BC, self.get_pair(BC).wrapping_sub(1));

        self.flags.pf = self.get_pair(BC) != 0;
        self.flags.nf = false;
        self.flags.nf = false;
        self.flags.yf = (n & 0x02) != 0;
        self.flags.xf = (n & 0x08) != 0;
        self.adv_cycles(16);
        self.adv_pc(2);
    }

    // 0xEDB0 Extended instruction
    fn ldir(&mut self) {
        // LDIR is basically LDI + BC if BC is not 0 decrease PC by 2.
        self.ldi();
        if self.get_pair(BC) != 0 {
            self.reg.prev_pc = self.reg.pc;
            self.reg.pc = self.reg.pc.wrapping_sub(2);
            self.adv_cycles(5);
        }
        // TODO This is wrong, gives value 7Fh twice before wrapping to 00, should be 7Fh -> 01h?
        if self.get_pair(BC) <= 0 {
            self.reg.r = (self.reg.r & 0x80) | (self.reg.r.wrapping_add(0) as u8 & 0x7f);
            /*self.reg.r = (self.reg.r & 0x80)
            | (self.reg.r.wrapping_add(255_u8.wrapping_mul(2)) as u8 & 0x7f);*/
        }
    }

    // Extended instructions: ex: LD (**), HL
    // 0xED63, 0xED53 etc..
    // Stores (REGPAIR) into the memory loc pointed to by **
    fn ld_nn(&mut self, reg: Register) {
        let ptr = self.read16(self.reg.pc);
        self.write16(ptr, self.get_pair(reg));
        self.adv_cycles(20);
        self.adv_pc(4);
    }

    // Extended instructions: ex: LD HL, (**)
    // 0xED6B, 0xED5B etc..
    // Loads the value pointed to by ** into (REGPAIR)
    fn load_indirect(&mut self, reg: Register) {
        let word = self.read16(self.reg.pc);
        match reg {
            BC => self.write_pair_indirect(BC, word),
            DE => self.write_pair_indirect(DE, word),
            HL => self.write_pair_indirect(HL, word),
            SP => self.write_pair_indirect(SP, word),
            IX => self.write_pair_indirect(IX, word),
            IY => self.write_pair_indirect(IY, word),
            _ => unimplemented!(),
        }
        self.adv_cycles(20);
        self.adv_pc(4);
    }

    // Load Register Pair Immediate
    // LXI H, 2000H (2000H is stored in HL & acts as as memory pointer)
    fn lxi(&mut self, reg: Register) {
        self.write_pair_direct(reg, self.read16(self.reg.pc + 1));
        self.adv_cycles(10);
        self.adv_pc(3);
    }

    // Store Accumulator direct
    fn sta(&mut self) {
        let imm = self.read16(self.reg.pc + 1);
        self.write8(imm, self.reg.a);
        self.adv_cycles(13);
        self.adv_pc(3);
    }

    fn call(&mut self, opcode: u16) {
        let ret: u16 = self.reg.pc.wrapping_add(3);
        match opcode {
            0xCC | 0xCD | 0xC4 | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC | 0x66 => {
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
        self.reg.pc = self.read16(self.reg.pc + 1);
        self.adv_cycles(17);
    }

    // Conditional calls
    fn call_cond(&mut self, addr: u16, cond: bool) {
        if cond {
            self.call(addr);
        } else {
            self.adv_cycles(10);
            self.adv_pc(3);
        }
    }

    fn cpl(&mut self) {
        self.reg.a ^= 0xFF;
        self.flags.hf = true;
        self.flags.nf = true;
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ccf(&mut self) {
        self.flags.hf = self.flags.cf;
        self.flags.cf = !self.flags.cf;
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.flags.nf = false;
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

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_sub(self.reg.a, value as u8);
        // self.flags.hf = self.carry_sub(3, a, result);
        self.flags.nf = true;
        // The XF & YF flags use the non compared value
        self.flags.yf = value & 0x20 != 0;
        self.flags.xf = value & 0x08 != 0;
        // TODO CHECK OVERFLOW HERE
        // self.flags.pf = self.overflow(value as u16, result);
        self.flags.pf = self.carry(7, self.reg.a as u16, !value as u16)
            != self.carry(8, self.reg.a as u16, !value as u16);
        self.flags.cf = result & 0x0100 != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Compare Immediate with Accumulator
    fn cp(&mut self) {
        let value = self.read8(self.reg.pc + 1);
        let result = (self.reg.a as i16).wrapping_sub(value as i16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.yf = value & 0x20 != 0;
        self.flags.hf = self.hf_sub(self.reg.a, value as u8);
        self.flags.xf = value & 0x08 != 0;
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.nf = true;
        self.flags.cf = result & 0x0100 != 0;
        // self.flags.pf = self.carry_sub(7, a, value, false) != self.carry_sub(8, a, value, false);

        self.adv_cycles(7);
        self.adv_pc(2);
    }
    // Extended instruction
    fn cpi(&mut self) {
        // TODO
        // Compares the value of the memory location pointed to by HL with A.
        // HL is then incremented and BC is decremented.
        // S,Z,H from (A - (HL) ) as in CP (HL)
        // F3 is bit 3 of (A - (HL) - H), H as in F after instruction
        // F5 is bit 1 of (A - (HL) - H), H as in F after instruction
        let value = self.read8(self.get_pair(HL));
        let result = (self.reg.a as u16).wrapping_sub(value as u16);
        self.flags.nf = true;
        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_sub(self.reg.a, value);
        self.flags.pf = self.overflow(value, result as u8);
        self.flags.cf = result & 0x0100 != 0;
        self.flags.yf = value & 0x20 != 0;
        self.flags.xf = value & 0x08 != 0;
    }

    pub(crate) fn add_hl(&mut self, reg: Register) {
        let hl: u16 = self.get_pair(HL);
        let (result, add) = match reg {
            BC => (
                ((self.get_pair(HL) as u32).wrapping_add(self.get_pair(BC) as u32)),
                self.get_pair(BC),
            ),
            DE => (
                ((self.get_pair(HL) as u32).wrapping_add(self.get_pair(DE) as u32)),
                self.get_pair(DE),
            ),
            HL => (
                ((self.get_pair(HL) as u32).wrapping_add(self.get_pair(HL) as u32)),
                self.get_pair(HL),
            ),
            SP => (
                ((self.get_pair(HL) as u32).wrapping_add(self.reg.sp as u32)),
                self.get_pair(SP),
            ),
            _ => panic!("Register: {:?} Not allowed for ADD HL", reg),
        };

        self.reg.h = (result >> 8) as u8;
        self.reg.l = result as u8;
        self.flags.cf = ((result >> 8) & 0x0100) != 0;

        // TODO Figure out why HF_ADD_W doesn't work here
        // self.flags.hf = self.hf_add_w(hl, add as u16);
        self.flags.hf = self.carry(12, hl, add as u16);

        self.flags.nf = false;
        self.flags.yf = (result >> 8) & 0x20 != 0;
        self.flags.xf = (result >> 8) & 0x08 != 0;
        self.adv_cycles(11);
        self.adv_pc(1);
    }

    // Decrement memory or register
    fn dec(&mut self, reg: Register) {
        // Example:
        // If the H register contains 3AH, and the L register contains 7CH
        // and memory location 3A7CH contains 40H, the instruction:
        // DCR M will cause memory location 3A7CH to contain 3FH.

        let result = match reg {
            A => dec_ret!(self.reg.a),
            B => dec_ret!(self.reg.b),
            C => dec_ret!(self.reg.c),
            D => dec_ret!(self.reg.d),
            E => dec_ret!(self.reg.e),
            I => dec_ret!(self.reg.i),
            H => dec_ret!(self.reg.h),
            L => dec_ret!(self.reg.l),
            HL | M => {
                self.adv_cycles(5);
                let hl = self.get_pair(Register::HL);
                self.memory[hl] = self.memory[hl].wrapping_sub(1);
                self.memory[hl]
            }
            _ => panic!("DEC on unknown reg:{:#?}", reg),
        };
        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_sub(result.wrapping_add(1), 1);

        self.flags.pf = self.carry(7, result.wrapping_add(1) as u16, !1_u16)
            != self.carry(8, result.wrapping_add(1) as u16, !1_u16);
        self.flags.nf = true;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // DEC register pair.
    // decrement! macro used for actual 16 bit registers for simplicity
    fn dex(&mut self, pair: Register) {
        match pair {
            BC => {
                let value = self.get_pair(BC).wrapping_sub(1);
                self.reg.b = (value >> 8) as u8;
                self.reg.c = value as u8;
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
            IX => self.reg.ix = self.reg.ix.wrapping_sub(1),
            IY => self.reg.iy = self.reg.iy.wrapping_sub(1),
            _ => panic!("DCX of normal registers not supported"),
        };
        if (pair == IY) || (pair == IX) {
            self.adv_cycles(4);
            self.adv_pc(1);
        }
        self.adv_cycles(6);
        self.adv_pc(1);
    }

    // Double precision add
    fn daa(&mut self) {
        // If C is set OR a > 0x99, add or subtract 0x60 depending on N, and set C
        // If H is set OR (a & 0xf) > 9, add or subtract 6 depending on N
        // H is set if bit 4 of A changed, otherwise cleared.
        // C is set as described above; note that DAA never clears the C flag if it was already set
        // (that would break multi-byte BCD arithmetic).
        // N is preserved, and the rest of the flags are set the usual way (P/V is parity, not overflow)

        let mut offset = 0;

        if self.flags.hf || self.reg.a & 0x0F > 0x09 {
            offset += 0x06;
        }
        if (self.reg.a > 0x99) || self.flags.cf {
            offset += 0x60;
            self.flags.cf = true;
        }
        if self.flags.nf {
            self.flags.hf = self.flags.hf && (self.reg.a & 0x0F) < 0x06;
            self.reg.a -= offset;
        } else {
            self.flags.hf = (self.reg.a & 0x0F) > 0x09;
            self.reg.a += offset;
        }
        let result = (self.reg.a as u16).wrapping_add(offset as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.pf = self.parity(result as u8);
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn set_interrupt_mode(&mut self, mode: u8) {
        self.int.mode = mode;
        self.adv_cycles(8);
        self.adv_pc(2);
    }
    // EI & DI instructions
    fn interrupt(&mut self, value: bool) {
        self.int.int = value;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left Through Carry
    fn rla(&mut self) {
        // The contents of the accumulator are rotated one bit position to the left.
        // The high-order bit of the accumulator replaces the carry bit while the carry bit
        // replaces the high-order bit of the accumulator
        let carry = (self.reg.a >> 7) != 0;
        self.reg.a = (self.reg.a << 1) | ((self.flags.cf as u8) << 7);
        self.flags.nf = false;
        self.flags.hf = false;
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.flags.cf = carry;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn rrc(&mut self, reg: Register) {
        let cf = self.flags.cf as u8;
        let value = match reg {
            A => rrc_ret!(self.reg.a, cf),
            B => rrc_ret!(self.reg.b, cf),
            C => rrc_ret!(self.reg.c, cf),
            D => rrc_ret!(self.reg.d, cf),
            E => rrc_ret!(self.reg.e, cf),
            H => rrc_ret!(self.reg.h, cf),
            L => rrc_ret!(self.reg.l, cf),
            _ => panic!(),
        };
        let carry = value & 0x80 != 0;
        self.flags.nf = false;
        self.flags.hf = false;
        self.flags.yf = value & 0x20 != 0;
        self.flags.xf = value & 0x08 != 0;
        self.flags.cf = carry;
        self.parity(value);
        self.adv_pc(2);
        self.adv_cycles(8);
    }
    // Extended instruction 0xCB03
    fn rlc(&mut self, reg: Register) {
        let cf = self.flags.cf as u8;
        let value = match reg {
            A => rlc_ret!(self.reg.a, cf),
            B => rlc_ret!(self.reg.b, cf),
            C => rlc_ret!(self.reg.c, cf),
            D => rlc_ret!(self.reg.d, cf),
            E => rlc_ret!(self.reg.e, cf),
            H => rlc_ret!(self.reg.h, cf),
            L => rlc_ret!(self.reg.l, cf),
            _ => panic!(),
        };
        let carry = (value & 0x80) != 0;
        self.flags.nf = false;
        self.flags.hf = false;
        self.flags.yf = value & 0x20 != 0;
        self.flags.xf = value & 0x08 != 0;
        self.flags.cf = carry;
        self.parity(value);
        self.adv_pc(2);
        self.adv_cycles(8);
    }
    // Rotate Accumulator Right Through Carry
    fn rra(&mut self) {
        let carry = (self.reg.a & 1) != 0;
        self.reg.a = (self.reg.a >> 1) | ((self.flags.cf as u8) << 7);
        self.flags.cf = carry;
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.flags.nf = false;
        self.flags.hf = false;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left
    fn rlca(&mut self) {
        self.flags.cf = (self.reg.a >> 7) != 0;
        self.reg.a = (self.reg.a << 1) | self.flags.cf as u8;
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.flags.nf = false;
        self.flags.hf = false;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn rrca(&mut self) {
        self.flags.cf = (self.reg.a & 1) != 0;
        self.reg.a = (self.reg.a >> 1) | ((self.flags.cf as u8) << 7);
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
        self.flags.nf = false;
        self.flags.hf = false;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Conditional return
    fn ret_cond(&mut self, cond: bool) {
        if cond {
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
        let value = self.read8(self.reg.pc + 1);
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
    fn lda_im(&mut self) {
        let imm = self.read16(self.reg.pc + 1);
        self.reg.a = self.memory[imm];
        self.adv_cycles(13);
        self.adv_pc(3);
    }

    // 0xDD Instruction LD IX/IY or IXH IYL etc + *
    // E.g stores A to the memory location pointed to by IX + *
    fn ld_dd(&mut self, dst: Register, src: Register) {
        let b = self.read8(self.reg.pc + 1) as u16;
        let pair = self.get_pair(dst);
        match src {
            A => self.memory[pair + b] = self.read_reg(src),
            _ => panic!(
                "DD prefixed LD unknown destination: {:?}, src:{:?}",
                dst, src
            ),
        };

        // TODO LD IXH Only uses 2 bytes and 8 cycles
        self.adv_cycles(19);
        self.adv_pc(3);
    }
    // LD (Load extended registers)
    fn ld_ex(&mut self, reg: Register) {
        // The contents of the designated register pair point to a memory location.
        // This instruction copies the contents of that memory location into the
        // accumulator. The contents of either the register pair or the
        // memory location are not altered.

        match reg {
            BC => self.reg.a = self.memory[self.get_pair(BC)],
            DE => self.reg.a = self.memory[self.get_pair(DE)],
            HL => self.reg.a = self.memory[self.get_pair(HL)],
            IX => self.reg.a = self.memory[self.get_pair(IX)],
            IY => self.reg.a = self.memory[self.get_pair(IY)],
            _ => panic!("LD on invalid register: {:?}", reg),
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
        let imm = self.read16(self.reg.pc + 1);
        self.reg.h = self.memory[imm + 1];
        self.reg.l = self.memory[imm];

        self.write_pair_indirect(Register::HL, self.read16(self.reg.pc + 1));
        self.adv_cycles(16);
        self.adv_pc(3);
    }

    fn in_(&mut self, _reg: Register) {
        // TODO: handle other registers than A
        let port = self.read8(self.reg.pc + 1);
        self.io.port0 = port;
        self.reg.a = 0xFF;
        self.adv_cycles(11);
        self.adv_pc(2);
    }

    pub(crate) fn inc(&mut self, reg: Register) {
        let result = match reg {
            A => inc_ret!(self.reg.a),
            B => inc_ret!(self.reg.b),
            C => inc_ret!(self.reg.c),
            D => inc_ret!(self.reg.d),
            E => inc_ret!(self.reg.e),
            H => inc_ret!(self.reg.h),
            L => inc_ret!(self.reg.l),
            I => inc_ret!(self.reg.i),
            HL => {
                self.adv_cycles(5);
                let hl = self.get_pair(Register::HL);
                self.memory[hl] = self.memory[hl].wrapping_add(1);
                self.memory[hl]
            }
            _ => unimplemented!("INC not implemented for:{:?}", reg),
        };

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_add(result.wrapping_sub(1), 1);
        self.flags.pf = self.carry(7, result.wrapping_sub(1) as u16, 1_u16)
            != self.carry(8, result.wrapping_sub(1) as u16, 1_u16);
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn inx(&mut self, reg: Register) {
        let value = self.get_pair(reg).wrapping_add(1);
        match reg {
            BC => {
                self.reg.b = (value >> 8) as u8;
                self.reg.c = (value & 0xFF) as u8;
            }
            DE => {
                self.reg.d = (value >> 8) as u8;
                self.reg.e = (value & 0xFF) as u8;
            }
            HL => {
                self.reg.h = (value >> 8) as u8;
                self.reg.l = (value & 0xFF) as u8;
            }
            SP => self.reg.sp = self.reg.sp.wrapping_add(1),
            IX => {
                self.reg.ix = self.reg.ix.wrapping_add(1);
                self.adv_cycles(4);
                self.adv_pc(1);
            }
            IY => {
                self.reg.iy = self.reg.iy.wrapping_add(1);
                self.adv_cycles(4);
                self.adv_pc(1);
            }
            _ => panic!("INX on non implemented register:{:#?}", reg),
        }
        self.adv_cycles(6);
        self.adv_pc(1);
    }

    fn push(&mut self, reg: Register) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);

        match reg {
            BC => self.write16(self.reg.sp, self.get_pair(BC)),
            DE => self.write16(self.reg.sp, self.get_pair(DE)),
            HL | H => self.write16(self.reg.sp, self.get_pair(HL)),
            L => unimplemented!(),
            R => unimplemented!(),
            IY => {
                self.write16(self.reg.sp, self.get_pair(IY));
                self.adv_pc(1);
                self.adv_cycles(4);
            }
            IX => {
                self.write16(self.reg.sp, self.get_pair(IX));
                self.adv_pc(1);
                self.adv_cycles(4);
            }
            AF => self.write16(self.reg.sp, self.get_pair(AF)),
            _ => panic!("Attempted Push on:{:#?}", reg),
        }

        self.adv_cycles(11);
        self.adv_pc(1);
    }

    // Store the contents of the accumulator addressed by registers B, C
    // or by registers D and E.
    fn stax(&mut self, reg: Register) {
        match reg {
            BC => self.write8(self.get_pair(BC), self.reg.a),
            DE => self.write8(self.get_pair(DE), self.reg.a),
            HL => self.write8(self.get_pair(HL), self.reg.a),
            IX => unimplemented!(),
            IY => unimplemented!(),
            SP => eprintln!("STAX should not run on SP register"),
            _ => unimplemented!("STAX not implemented for:{:?}", reg),
        }
        self.adv_cycles(7);
        self.adv_pc(1);
    }

    // SBC Subtract Register or Memory from Accumulator with carry flag
    fn sbc(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };

        let result = (self.reg.a as u16)
            .wrapping_sub(value as u16)
            .wrapping_sub(self.flags.cf as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;

        self.flags.hf = self.hf_sub(self.reg.a, value as u8);
        /*self.flags.hf =
            (self.reg.a as i8 & 0x0F) - (value as i8 & 0x0F) - (self.flags.cf as i8) >= 0;*/
        self.flags.pf = self.overflow(value, result as u8);
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = result & 0x0100 != 0;
        self.flags.nf = true;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    // TODO: SBI & SUI can be consolidated to one function
    // Subtract Immediate with Borrow
    fn sbi(&mut self) {
        let imm = self.read8(self.reg.pc + 1);
        let value = imm + self.flags.cf as u8;
        let result = (self.reg.a as u16).wrapping_sub(value as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_sub(self.reg.a, value as u8);
        self.flags.pf = self.overflow(imm, result as u8);
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.nf = true;
        self.flags.cf = result & 0x0100 != 0;
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
        if reg == Register::IX || reg == Register::IY {
            self.adv_pc(1);
            self.adv_cycles(4);
        }

        let result = (self.reg.a as u16).wrapping_sub(value as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_sub(self.reg.a, value);
        self.flags.pf = self.overflow(self.reg.a, result as u8);
        self.flags.nf = true;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = result & 0x0100 != 0;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // SUI Subtract Immediate From Accumulator
    fn sui(&mut self) {
        let value = self.read8(self.reg.pc + 1);
        let result = (self.reg.a as u16).wrapping_sub(value as u16);

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = self.hf_sub(self.reg.a, value as u8);
        self.flags.pf = self.overflow(value, result as u8);
        self.flags.nf = true;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = result & 0x0100 != 0;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // Set Carry (set carry bit to 1)
    fn scf(&mut self) {
        self.flags.cf = true;
        self.flags.nf = false;
        self.flags.hf = false;
        self.flags.yf = self.reg.a & 0x20 != 0;
        self.flags.xf = self.reg.a & 0x08 != 0;
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

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = false;
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.cf = false;
        self.flags.pf = self.parity(result as u8);
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // XRI Exclusive-Or Immediate with Accumulator
    fn xri(&mut self) {
        let imm = self.read8(self.reg.pc + 1);
        let result = self.reg.a ^ imm as u8;

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = false;
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.pf = self.parity(result);
        self.flags.cf = false;
        self.reg.a = result;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn ex_af_af(&mut self) {
        let a = self.reg.a;
        let a_ = self.reg.a_;
        self.reg.a = a_;
        self.reg.a_ = a;
        self.flags.swap();
        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn exx(&mut self) {
        let b = self.reg.b;
        let c = self.reg.c;
        let d = self.reg.d;
        let e = self.reg.e;
        let h = self.reg.h;
        let l = self.reg.l;

        self.reg.b = self.reg.b_;
        self.reg.c = self.reg.c_;
        self.reg.d = self.reg.d_;
        self.reg.e = self.reg.e_;
        self.reg.h = self.reg.h_;
        self.reg.l = self.reg.l_;

        self.reg.b_ = b;
        self.reg.c_ = c;
        self.reg.d_ = d;
        self.reg.e_ = e;
        self.reg.h_ = h;
        self.reg.l_ = l;
        self.adv_pc(1);
        self.adv_cycles(4);
    }
    fn ex_de_hl(&mut self) {
        use std::mem;
        mem::swap(&mut self.reg.h, &mut self.reg.d);
        mem::swap(&mut self.reg.l, &mut self.reg.e);
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn xthl(&mut self) {
        // Swap H:L with top word on stack
        let hl = self.get_pair(Register::HL) as u16;
        let new_hl = self.read16(self.reg.sp);

        // Write old HL values to memory
        self.write16(self.reg.sp, hl);
        self.reg.h = (new_hl >> 8) as u8;
        self.reg.l = new_hl as u8;
        self.adv_cycles(19);
        self.adv_pc(1);
    }

    fn pop(&mut self, reg: Register) {
        let hb = self.read8(self.reg.sp + 1);
        let lb = self.read8(self.reg.sp);

        match reg {
            BC => {
                self.reg.c = lb;
                self.reg.b = hb;
            }
            DE => {
                self.reg.e = lb;
                self.reg.d = hb;
            }
            HL => {
                self.reg.l = lb;
                self.reg.h = hb;
            }
            IX => {
                let hb = hb as u16;
                let lb = lb as u16;
                self.reg.ix = hb << 8 | lb;
                self.adv_pc(1);
                self.adv_cycles(4);
            }
            IY => {
                let hb = hb as u16;
                let lb = lb as u16;
                self.reg.iy = hb << 8 | lb;
                self.adv_pc(1);
                self.adv_cycles(4);
            }
            AF => {
                self.flags.set(lb);
                self.reg.a = hb;
            }
            _ => panic!("Attempted to pop unknown register:{:?}", reg),
        }
        self.reg.sp = self.reg.sp.wrapping_add(2);

        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.read16(self.reg.sp + 1);
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
        self.reg.pc = ret as u16;
        self.reg.sp = self.reg.sp.wrapping_add(2);
        self.adv_cycles(10);
    }

    // TODO: Implement
    fn out(&mut self) {
        let port = self.read8(self.reg.pc + 1);
        self.adv_cycles(11);
        self.adv_pc(2);
    }
    // TODO: Consolidate ORA & ORI (pass value directly)
    fn ora(&mut self, reg: Register) {
        let value = if reg != Register::HL {
            self.read_reg(reg)
        } else {
            self.adv_cycles(3);
            self.memory[self.get_pair(Register::HL)]
        };

        let result = self.reg.a as u16 | value as u16;

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = false;
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.pf = self.parity(result as u8);
        self.flags.cf = false;
        self.reg.a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Or Immediate with Accumulator
    fn ori(&mut self) {
        let result = self.reg.a as u16 | self.read8(self.reg.pc + 1) as u16;

        self.flags.sf = result & 0x80 != 0;
        self.flags.zf = result & 0xFF == 0;
        self.flags.hf = false;
        self.flags.nf = false;
        self.flags.yf = result & 0x20 != 0;
        self.flags.xf = result & 0x08 != 0;
        self.flags.pf = self.parity(result as u8);
        self.flags.cf = false;
        self.reg.a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }
    // TODO Refactor
    fn ld(&mut self, dst: Register, src: Register) {
        // let value =  self.read_reg(src);
        let value: u16 = match src {
            A => u16::from(self.reg.a),
            B => u16::from(self.reg.b),
            C => u16::from(self.reg.c),
            D => u16::from(self.reg.d),
            E => u16::from(self.reg.e),
            H => u16::from(self.reg.h),
            L => u16::from(self.reg.l),
            I => u16::from(self.reg.i),
            R => u16::from(self.reg.r),
            HL => self.get_pair(HL),
            DE => self.get_pair(DE),
            BC => self.get_pair(BC),
            _ => panic!("Non handled LD source"),
        };

        let addr = self.get_pair(Register::HL) as u16;

        match dst {
            A => {
                if src == Register::HL {
                    self.reg.a = self.read8(addr);
                    self.adv_cycles(3);
                } else if (src == R) || (src == I) {
                    self.write_reg(dst, value as u8);
                    self.flags.sf = (self.reg.a & 0x80) != 0;
                    self.flags.zf = (self.reg.a & 0xFF) == 0;
                    // TODO PF interrupt interrupt handling
                    self.flags.pf = self.int.iff2;
                    self.flags.hf = false;
                    self.flags.nf = false;
                    self.adv_cycles(5);
                    self.adv_pc(1);
                } else {
                    self.write_reg(dst, value as u8)
                }
            }
            B => {
                if src == Register::HL {
                    self.reg.b = self.read8(addr);
                    self.adv_cycles(3);
                } else {
                    self.write_reg(dst, value as u8);
                }
            }
            C => {
                if src == Register::HL {
                    self.reg.c = self.read8(addr);
                    self.adv_cycles(3);
                } else {
                    self.write_reg(dst, value as u8);
                }
            }
            D => {
                if src == Register::HL {
                    self.reg.d = self.read8(addr);
                    self.adv_cycles(3);
                } else {
                    self.write_reg(dst, value as u8);
                }
            }
            E => {
                if src == Register::HL {
                    self.reg.e = self.read8(addr);
                    self.adv_cycles(3);
                } else {
                    self.write_reg(dst, value as u8);
                }
            }
            H => {
                if src == Register::HL {
                    self.reg.h = self.read8(addr);
                    self.adv_cycles(3);
                } else {
                    self.write_reg(dst, value as u8);
                }
            }
            L => {
                if src == Register::HL {
                    self.reg.l = self.read8(addr);
                    self.adv_cycles(3);
                } else {
                    self.write_reg(dst, value as u8);
                }
            }
            HL => {
                self.memory[addr] = self.read_reg(src);
                match src {
                    A => self.reg.a = self.memory[self.get_pair(HL)],
                    B => self.reg.b = self.memory[self.get_pair(HL)],
                    C => self.reg.c = self.memory[self.get_pair(HL)],
                    D => self.reg.d = self.memory[self.get_pair(HL)],
                    E => self.reg.e = self.memory[self.get_pair(HL)],
                    H => self.reg.h = self.memory[self.get_pair(HL)],
                    L => self.reg.l = self.memory[self.get_pair(HL)],
                    _ => panic!(
                        "LD (HL), REG on non implemented reg. src:{:?}, dst:{:?}",
                        src, dst
                    ),
                }
                self.adv_cycles(3);
            }
            I => {
                if src == Register::HL {
                    self.reg.i = self.read8(addr);
                    self.adv_cycles(2);
                }
                self.adv_cycles(5);
                self.adv_pc(1);
                self.write_reg(dst, value as u8);
            }
            R => {
                if src == Register::HL {
                    self.reg.r = self.read8(addr);
                    self.adv_cycles(2);
                }
                self.adv_pc(1);
                self.adv_cycles(5);
                self.write_reg(dst, value as u8);
            }
            _ => panic!("Unhandled LD register"),
        }
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // RESET (used for interrupt jump / calls)
    pub fn rst(&mut self, value: u8) {
        // Address to return to after interrupt is finished.
        let ret = self.reg.pc + 1;
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.write16(self.reg.sp, ret);
        self.reg.prev_pc = self.reg.pc;

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
        self.adv_cycles(6);
        self.adv_pc(1);
    }

    // Store H & L direct
    fn shld(&mut self) {
        let addr = self.read16(self.reg.pc + 1);
        let hl = self.get_pair(Register::HL) as u16;
        self.write16(addr, hl);
        self.adv_cycles(16);
        self.adv_pc(3);
    }

    pub fn nop(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    pub fn execute(&mut self) {
        self.fetch();
        self.decode(self.opcode);
    }

    pub(crate) fn fetch(&mut self) {
        self.opcode = self.read8(self.reg.pc) as u16;
        self.instruction = Instruction::decode(self.opcode);

        if self.instruction.name.to_string().len() < 1 {
            self.current_instruction = format!("{:w$}", self.current_instruction, w = 12);
        } else {
            self.current_instruction = self.instruction.name.to_string();
        }
    }

    pub fn decode(&mut self, opcode: u16) {
        use self::Register::*;
        if self.debug {
            println!("{:?}", self);
        }

        self.reg.r = (self.reg.r & 0x80) | (self.reg.r.wrapping_add(1)) & 0x7f;
        match opcode {
            0x00 => self.nop(),
            0x01 => self.lxi(BC),
            0x02 => self.stax(BC),
            0x03 => self.inx(BC),
            0x04 => self.inc(B),
            0x05 => self.dec(B),
            0x06 => self.mvi(B),
            0x07 => self.rlca(),
            0x08 => self.ex_af_af(),
            0x09 => self.add_hl(BC),
            0x10 => self.djnz(),

            0x0A => self.ld_ex(BC),
            0x0B => self.dex(BC),
            0x0C => self.inc(C),
            0x0D => self.dec(C),
            0x0E => self.mvi(C),
            0x0F => self.rrca(),

            0x11 => self.lxi(DE),
            0x12 => self.stax(DE),
            0x13 => self.inx(DE),
            0x14 => self.inc(D),
            0x15 => self.dec(D),
            0x16 => self.mvi(D),
            0x17 => self.rla(),
            0x18 => self.jr(self.read8(self.reg.pc) as i16),
            0x19 => self.add_hl(DE),

            0x1A => self.ld_ex(DE),
            0x1B => self.dex(DE),
            0x1C => self.inc(E),
            0x1D => self.dec(E),
            0x1E => self.mvi(E),
            0x1F => self.rra(),

            0x20 => self.jr_cond(!self.flags.zf),
            0x21 => self.lxi(HL),
            0x22 => self.shld(),
            0x23 => self.inx(HL),
            0x24 => self.inc(H),
            0x25 => self.dec(H),
            0x26 => self.mvi(H),
            0x27 => self.daa(),
            0x28 => self.jr_cond(self.flags.zf),
            0x29 => self.add_hl(HL),

            0x2A => self.lhld(),
            0x2B => self.dex(HL),
            0x2C => self.inc(L),
            0x2D => self.dec(L),
            0x2E => self.mvi(L),
            0x2F => self.cpl(),

            0x30 => self.jr_cond(!self.flags.cf),
            0x31 => self.lxi(SP),
            0x32 => self.sta(),
            0x33 => self.inx(SP),
            0x34 => self.inc(HL),
            0x35 => self.dec(HL),
            0x36 => self.mvi(HL),
            0x37 => self.scf(),
            0x38 => self.jr_cond(self.flags.cf), // JR C, *
            0x39 => self.add_hl(SP),

            0x3A => self.lda_im(),
            0x3B => self.dex(SP),
            0x3C => self.inc(A),
            0x3D => self.dec(A),
            0x3E => self.mvi(A),
            0x3F => self.ccf(),

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

            0x98 => self.sbc(B),
            0x99 => self.sbc(C),
            0x9A => self.sbc(D),
            0x9B => self.sbc(E),
            0x9C => self.sbc(H),
            0x9D => self.sbc(L),
            0x9E => self.sbc(HL),
            0x9F => self.sbc(A),

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

            0xC0 => self.ret_cond(!self.flags.zf),
            0xC1 => self.pop(BC),
            0xC2 => self.jp_cond(!self.flags.zf),
            0xC3 => self.jp_cond(true),
            0xC4 => self.call_cond(0xC4, !self.flags.zf),
            0xC5 => self.push(BC),
            0xC6 => self.adi(),
            0xC7 => self.rst(0),
            0xC8 => self.ret_cond(self.flags.zf),
            0xC9 => self.ret(),

            0xCA => self.jp_cond(self.flags.zf),
            0xCB => {
                self.opcode = self.read8(self.reg.pc + 1) as u16;
                self.reg.r = (self.reg.r & 0x80) | (self.reg.r.wrapping_add(1)) & 0x7f;
                match self.opcode {
                    0xCB00 => unimplemented!(),
                    0xCB03 => self.rlc(E),
                    0xCB08 => self.rrc(B),
                    0xCBC7 => unimplemented!(),
                    _ => unimplemented!(),
                }
            }
            0xCC => self.call_cond(0xCC, self.flags.zf),
            0xCD => self.call(0xCD),
            0xCE => self.adc_im(),
            0xCF => self.rst(1),

            0xD0 => self.ret_cond(!self.flags.cf),
            0xD1 => self.pop(DE),
            0xD2 => self.jp_cond(!self.flags.cf),
            0xD3 => self.out(),
            0xD4 => self.call_cond(0xD4, !self.flags.cf),
            0xD5 => self.push(DE),
            0xD6 => self.sui(),
            0xD7 => self.rst(2),
            0xD8 => self.ret_cond(self.flags.cf),
            0xD9 => self.exx(),
            0xDA => self.jp_cond(self.flags.cf),
            0xDB => self.in_(A),
            0xDC => self.call_cond(0xDC, self.flags.cf),
            0xDD => {
                self.opcode = self.read8(self.reg.pc + 1) as u16;
                self.reg.r = (self.reg.r & 0x80) | self.reg.r.wrapping_add(1) & 0x7f;
                self.instruction = Instruction::decode(self.opcode);

                match self.opcode {
                    0x09 => unimplemented!("{:#?}", self.instruction),
                    0x19 => self.add_ex(IX, DE),
                    0x21 => {
                        self.reg.ix = self.read16(self.reg.pc + 2);
                        self.adv_pc(4);
                        self.adv_cycles(14);
                    }
                    0x22 => {
                        self.write8(
                            self.read8(self.reg.pc + 2) as u16,
                            self.read16(self.reg.ix) as u8,
                        );
                        self.adv_pc(4);
                        self.adv_cycles(20);
                    }
                    0x23 => self.inx(IX),
                    0x24 => unimplemented!(),
                    0x25 => unimplemented!(),
                    0x26 => unimplemented!(),
                    0x29 => unimplemented!(),
                    0x2A => unimplemented!(),
                    0x2B => unimplemented!(),
                    0x2C => unimplemented!(),
                    0x2D => unimplemented!(),
                    0x2E => unimplemented!(),
                    0x34 => unimplemented!(),
                    0x35 => unimplemented!(),
                    0x36 => unimplemented!(),
                    0x39 => unimplemented!(),
                    0x3C => unimplemented!(),
                    0x3D => unimplemented!(),
                    0x3E => unimplemented!(),
                    0xE1 => self.pop(IX),
                    0xE5 => self.push(IX),
                    0x7E => {
                        // byte is the signed displacement byte
                        let byte = self.read8(self.reg.pc + 2) as i8;
                        let addr = self.reg.ix.wrapping_add(byte as u16);
                        self.reg.a = self.read8(addr) as i8 as u8;
                        self.adv_pc(3);
                        self.adv_cycles(19);
                    }
                    0x77 => self.ld_dd(IX, A),
                    0xE9 => {
                        self.opcode = 0xDDE9;
                        self.instruction.cycles = 8;
                        self.jp(self.reg.ix);
                    }

                    _ => panic!(
                        "Unknown or unimplemented instruction:{:#?}",
                        self.instruction
                    ),
                }
            }
            0xDE => self.sbi(),
            0xDF => self.rst(3),
            0xE0 => self.ret_cond(!self.flags.pf),
            0xE1 => self.pop(HL),
            0xE2 => self.jp_cond(!self.flags.pf),
            0xE3 => self.xthl(),
            0xE4 => self.call_cond(0xE4, !self.flags.pf),
            0xE5 => self.push(HL),
            0xE6 => self.ani(),
            0xE7 => self.rst(4),
            0xE8 => self.ret_cond(self.flags.pf),
            0xE9 => self.pchl(),

            0xEA => self.jp_cond(self.flags.pf),
            0xEB => self.ex_de_hl(),
            0xEC => self.call_cond(0xEC, self.flags.pf),
            0xED => {
                self.opcode = self.read8(self.reg.pc + 1) as u16;
                self.reg.r = (self.reg.r & 0x80) | (self.reg.r.wrapping_add(1)) & 0x7f;
                match self.opcode {
                    0xA0 => self.ldi(),
                    0xA1 => self.cpi(),
                    0xB0 => self.ldir(),
                    0x43 => self.ld_nn(BC),
                    0x46 => self.set_interrupt_mode(0),
                    0x47 => self.ld(I, A),
                    0x53 => self.ld_nn(DE),
                    0x56 => self.set_interrupt_mode(1),
                    0x57 => self.ld(A, I),
                    0x63 => self.ld_nn(HL),
                    0x66 => self.set_interrupt_mode(0),
                    0x76 => self.set_interrupt_mode(1),
                    0x4A => self.adc_hl(BC),
                    0x4B => self.load_indirect(BC),
                    0x4F => self.ld(R, A),
                    0x5F => self.ld(A, R),
                    0x5A => self.adc_hl(DE),
                    0x5B => self.load_indirect(DE),
                    0x6A => self.adc_hl(HL),
                    0x6B => self.load_indirect(HL),
                    0x73 => self.ld_nn(SP),
                    0x7B => self.load_indirect(SP),
                    0x7A => self.adc_hl(SP),
                    0x7E => self.set_interrupt_mode(2),
                    _ => panic!("{:#?}", Instruction::decode(self.opcode)),
                }
            }

            0xEE => self.xri(),
            0xEF => self.rst(5),
            0xF0 => self.ret_cond(!self.flags.sf),
            0xF1 => self.pop(AF),
            0xF2 => self.jp_cond(!self.flags.sf),
            0xF3 => self.interrupt(false),
            0xF4 => self.call_cond(0xF4, !self.flags.sf),
            0xF5 => self.push(AF),
            0xF6 => self.ori(),
            0xF7 => self.rst(4),
            0xF8 => self.ret_cond(self.flags.sf),
            0xF9 => self.sphl(),
            0xFA => self.jp_cond(self.flags.sf),
            0xFB => self.interrupt(true),
            0xFC => self.call_cond(0xFC, self.flags.sf),
            0xFD => {
                self.opcode = self.read8(self.reg.pc + 1) as u16;
                self.reg.r = (self.reg.r & 0x80) | (self.reg.r.wrapping_add(1)) & 0x7f;
                match self.opcode {
                    0x09 => unimplemented!(),
                    0x19 => unimplemented!(),
                    0x21 => {
                        self.reg.iy = self.read16(self.reg.pc + 2);
                        self.adv_pc(4);
                        self.adv_cycles(14);
                    }
                    0x22 => {
                        self.write8(
                            self.read8(self.reg.pc + 2) as u16,
                            self.read16(self.reg.iy) as u8,
                        );
                        self.adv_pc(4);
                        self.adv_cycles(20);
                    }
                    0x23 => self.inx(IY),
                    0x24 => unimplemented!(),
                    0x25 => unimplemented!(),
                    0x26 => unimplemented!(),
                    0x29 => unimplemented!(),
                    0x2A => unimplemented!(),
                    0x2B => self.dex(IY),
                    0x2D => unimplemented!(),
                    0x2C => unimplemented!(),
                    0x2E => unimplemented!(),
                    0xE1 => self.pop(IY),
                    0xE5 => self.push(IY),
                    0xE9 => {
                        self.instruction.cycles = 8;
                        self.jp(self.get_pair(IY))
                    }
                    0x7E => {
                        // byte is the signed displacement byte
                        let byte = self.read8(self.reg.pc + 2) as i8;
                        let addr = self.reg.iy.wrapping_add(byte as u16);
                        self.reg.a = self.read8(addr) as i8 as u8;
                        self.adv_pc(3);
                        self.adv_cycles(19);
                    }
                    _ => panic!("{:#?}", Instruction::decode(self.opcode)),
                }
            }
            0xFE => self.cp(),
            0xFF => self.rst(7),
            _ => println!("Unknown opcode: {:04X}", self.opcode),
        }
    }

    pub fn reset(&mut self) {
        self.reg.a = 0xff;
        self.reg.b = 0;
        self.reg.c = 0;
        self.reg.d = 0;
        self.reg.e = 0;
        self.reg.h = 0;
        self.reg.l = 0;
        self.reg.sp = 0xffff;
        self.reg.r = 0;
        // Reset flag conditions
        self.flags.set(0xff);
        self.int.mode = 1;
        self.int.iff1 = false;
        self.int.iff2 = false;
    }

    // TODO interrupt handle
    fn hlt(&mut self) {
        if self.int_pending && self.int.int {
            ::std::process::exit(1);
        }
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

    // Borrowed from github.com/superzazu for debugging purposes
    // returns if there was a carry between bit "bit_no" and "bit_no - 1" when
    // executing "a + b + cy"
    fn carry(&self, bit_no: u8, a: u16, b: u16) -> bool {
        let result = a.wrapping_add(b);
        let carry = result ^ a ^ b;
        return bool::from(carry & (1 << bit_no) != 0);
    }
    fn carry_sub(&self, bit_no: u8, a: u16, b: u16) -> bool {
        let result = a.wrapping_sub(b);
        let carry = result ^ a ^ b;
        return bool::from(carry & (1 << bit_no) != 0);
    }

    fn hf_add(&self, a: u8, b: u8) -> bool {
        // (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
        ((((a & 0xF) + (b & 0xF)) & 0x10) & (1 << 4)) != 0
    }
    fn hf_add_w(&self, a: u16, b: u16) -> bool {
        // Check carry of bit 12
        (((a & 0x0F00) + (b & 0x0F00) & 0x1000) & (1 << 12)) != 0
    }
    fn hf_sub_w(a: u16, b: u16) -> bool {
        // ((((a as i16 & 0x0F00) + (b as i16 & 0x0F00)) & 0x1000) & (1 << 12)) != 0
        (((a as i16 & 0x0F00) - (b as i16 & 0x0F00)) & 0x1000) < 0
    }

    fn hf_sub(&self, a: u8, b: u8) -> bool {
        (a as i8 & 0x0F) - (b as i8 & 0x0F) < 0
    }

    fn overflow(&mut self, a: u8, result: u8) -> bool {
        // Overflow should be set if the 2-complement result does not fit the register
        // Set overflow flag when A and the operand have the same sign
        // and A and the result have different sign
        let op = self.read8(self.reg.pc + 1);
        ((a >> 7) == (op.wrapping_shl(7))) && ((a.wrapping_shr(7)) != (result.wrapping_shr(7)))
    }
    pub(crate) fn handle_interrupts(&mut self, mut delay: u8) {
        if delay > 0 {
            delay -= 1;
        }
        if delay == 0 {
            self.int.iff1 = true;
            self.int.iff2 = true;
        }
        if self.int.nmi_pending && self.int.int {
            self.int.nmi_pending = false;
            self.int.iff1 = false;
            inc_ret!(self.reg.r);
            self.adv_cycles(11);
            self.call(0x66);
            return;
        }
        if self.int_pending && self.int.iff1 {
            self.int_pending = false;
            self.int.iff1 = false;
            self.int.iff2 = false;
            inc_ret!(self.reg.r);
        }
        // Interrupt Mode 0 is the 8080 compatibility mode
        // Most commonly the instruction executed on the bus is RST,
        // but it can be any instruction (technically)
        // The I register is not used for IM0
        match self.int.mode {
            0 => {
                self.adv_cycles(11);
                self.decode(0xFA);
            }
            1 => {
                // Mode 1, RST38h, regardless of bus value or I reg value.
                self.adv_cycles(13);
                self.rst(7)
            }
            2 => {
                // Call made to the address read from memory.
                // address to be read from is calculated:
                // self.reg.i * 256 + busvalue
                self.adv_cycles(19);

                // so the MSB (8 bits) of the address, and the interrupt data as the LSB 8 bits
                self.call(self.read16((self.reg.i).wrapping_shl(8) as u16) | 0xFC)
            }
            _ => {}
        }
    }

    pub fn try_reset_cycles(&mut self) {
        if self.cycles < 25_600 {
            return;
        } else {
            self.cycles = 0;
        }
    }
}
