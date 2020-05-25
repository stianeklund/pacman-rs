#[cfg(test)]
mod tests {
    use crate::instruction_info::Register;
    use crate::interconnect::Interconnect;
    use crate::instruction_info::Register::{HL, BC};
    use crate::memory::MemoryRW;

    // #[test]
    fn test_hf_flag() {
        // Make sure HF flag gets set on accumulator value wrap from FFh to 00h.
        let mut i = Interconnect::new();
        i.cpu.reg.a = 0xff;
        i.cpu.inc(Register::A);
        assert_eq!(i.cpu.flags.hf, true);
    }

    // #[test]
    fn test_hf_high_byte() {
        let mut i = Interconnect::new();
        i.cpu.set_pair(BC, 1); // Set BC to 1 (we will increment HL by 1)
        i.cpu.reg.a = 0xff; // set to 0FFh
        i.cpu.set_pair(HL, 0x00FF);
        i.cpu.add_hl(BC);
        i.cpu.inc(Register::A);
        assert_eq!(i.cpu.flags.hf, true);
    }


    #[test]
    fn fast_z80() {
        // TODO: Add cycle testing!
        assert_eq!(exec_test("tests/prelim.com"), 0x447); // Should have executed 8710 cycles
        assert_eq!(exec_test("tests/8080PRE.COM"), 0x32F);
        // assert_eq!(exec_test("tests/CPUTEST.COM"), 0x3B25);
    }

    // #[test]
    fn z80_precise() {
        assert_eq!(exec_test("tests/zexdoc.com"), 0x0000);
        assert_eq!(exec_test("tests/zexall.com"), 0x0000);
    }

    // #[test]
    fn all_tests() {
        assert_eq!(exec_test("tests/prelim.com"), 0x49A);
        assert_eq!(exec_test("tests/8080PRE.COM"), 0x32F);
        assert_eq!(exec_test("tests/TEST.COM"), 0x14F);
        assert_eq!(exec_test("tests/TST8080.COM"), 0x6BA);
        assert_eq!(exec_test("tests/CPUTEST.COM"), 0x3B25);
        // assert_eq!(exec_test("tests/zexall.com"), 0x0000);
        // assert_eq!(exec_test("tests/zexdoc.com"), 0x0000);
        // assert_eq!(exec_test("tests/8080EX1.COM"), 0x00);
        // assert_eq!(exec_test("tests/8080EXM.COM"), 0x00)
    }

    fn exec_test(bin: &str) -> u16 {
        let mut i = Interconnect::new();
        i.cpu.reset();
        i.cpu.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0007 to handle the return call.
        // Inject IN, A * to trigger printing of characters.
        // If successful it should return to 0x0007.

        i.cpu.memory.memory[0x0000] = 0xD3;
        i.cpu.memory.memory[0x0001] = 0x00;
        i.cpu.memory.memory[0x0005] = 0xDB;
        i.cpu.memory.memory[0x0006] = 0x00;
        i.cpu.memory.memory[0x0007] = 0xC9;

        // All test binaries start at 0x0100.
        i.cpu.reg.pc = 0x0100;
        let _cycles = 0;

        loop {
            // i.cpu.debug = true;
            i.run_tests();

            if i.cpu.reg.pc == 0x76 {
                assert_ne!(i.cpu.reg.pc, 0x76);
            }

            if i.cpu.reg.pc == 07 {
                if i.cpu.reg.c == 9 {
                    let mut de = (i.cpu.reg.d as u16) << 8 | (i.cpu.reg.e as u16);
                    'print: loop {
                        let output = i.cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.cpu.reg.c == 2 {
                    print!("{}", i.cpu.reg.e as char);
                }
            }
            // let af = (i.cpu.reg.a as u16) << 8 | (i.cpu.flags.get() as u16);
            /*if i.cpu.get_pair(HL) >= 0x2000 {
                i.cpu.debug = true;
                // if i.cpu.reg.pc == 0x02C1 && !i.cpu.flags.hf && i.cpu.flags.pf && i.cpu.cycles != 47693 && af != 0x0044 {
                // eprintln!("{}", i.cpu);
                // eprintln!("{:#?}", i.cpu.flags);
                // panic!();
            }*/
            if i.cpu.reg.pc == 0 {
                println!(" Jump to 0 from {:04X}", i.cpu.reg.prev_pc);
                println!("Cycles executed: {}", i.cpu.cycles);
                break;
            }
        }
        i.cpu.reg.prev_pc
    }
}
