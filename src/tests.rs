#[cfg(test)]
mod tests {
    use crate::instruction_info::Register;
    use crate::instruction_info::Register::{BC, DE, HL};
    use crate::interconnect::Interconnect;
    use crate::memory::MemoryRW;

    #[test]
    fn test_hf_flag() {
        // Make sure HF flag gets set on accumulator value wrap from FFh to 00h.
        let mut i = Interconnect::new();
        i.cpu.reg.a = 0xff;
        i.cpu.inc(Register::A);
        assert_eq!(i.cpu.flags.hf, true);
    }

    #[test]
    fn test_hf_high_byte() {
        // The half carry flag should be set once we increment HL from 00FFh to 0000h
        let mut i = Interconnect::new();
        i.cpu.write_pair_direct(BC, 1); // Set BC to 1 (we will increment HL by 1)
        i.cpu.reg.a = 0xff;
        i.cpu.write_pair_direct(HL, 0x00FF);
        i.cpu.add_hl(BC);
        i.cpu.inc(Register::A);
        assert_eq!(i.cpu.flags.hf, true);
    }

    #[test]
    fn fast_z80() {
        // TODO: Add cycle testing!
        assert_eq!(exec_test("tests/prelim.com"), 0x447); // Should have executed 8710 cycles
        assert_eq!(exec_test("tests/8080PRE.COM"), 0x32F);
        assert_eq!(exec_test("tests/CPUTEST.COM"), 0x0C2E);
    }

    // #[test]
    fn z80_precise() {
        assert_eq!(exec_test("tests/zexdoc.com"), 0x0000);
        // assert_eq!(exec_test("tests/zexall.com"), 0x0000);
    }

    // #[test]
    fn all_tests() {
        assert_eq!(exec_test("tests/prelim.com"), 0x447);
        assert_eq!(exec_test("tests/8080PRE.COM"), 0x32F);
        assert_eq!(exec_test("tests/CPUTEST.COM"), 0x3B25);
        assert_eq!(exec_test("tests/zexall.com"), 0x0000);
        assert_eq!(exec_test("tests/zexdoc.com"), 0x0000);
        // assert_eq!(exec_test("tests/8080EX1.COM"), 0x00);
        // assert_eq!(exec_test("tests/8080EXM.COM"), 0x00)
    }

    fn exec_test(bin: &str) -> u16 {
        let mut i = Interconnect::new();
        i.cpu.reset();
        i.cpu.memory.load_tests(bin);

        // Patches the test rom(s) to intercept CP/M bdos routine
        // Inject OUT *, A at 0x0000.
        // Inject RET (0xC9) at 0x0007 to handle the return call.
        // Inject IN, A * to store BDOS output
        // If successful it should return to 0x0007.

        i.cpu.memory.rom[0x0000] = 0xD3;
        i.cpu.memory.rom[0x0001] = 0x00;
        i.cpu.memory.rom[0x0005] = 0xDB;
        i.cpu.memory.rom[0x0006] = 0x00;
        i.cpu.memory.rom[0x0007] = 0xC9;

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
                    let mut de = i.cpu.get_pair(DE);
                    'print: loop {
                        let output = i.cpu.memory.rom[de as usize];
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

            if i.cpu.reg.pc == 0 {
                println!(
                    "\nBDOS routine called, jumped to: 0 from {:04X}",
                    i.cpu.reg.prev_pc
                );
                println!("Cycles executed: {}\n", i.cpu.cycles);
                println!();
                break;
            }
        }
        i.cpu.reg.prev_pc
    }
}
