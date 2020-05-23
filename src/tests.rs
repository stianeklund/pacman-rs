#[cfg(test)]
mod tests {
    use crate::instruction_info::{Instruction, Register};
    use crate::interconnect::Interconnect;
    use crate::memory::MemoryRW;
    use std::time::Duration;

    #[test]
    fn fast_z80() {
        // assert_eq!(exec_test("tests/prelim.com"), 0x32F);
        // assert_eq!(exec_test("tests/8080PRE.COM"), 0x32F);
        assert_eq!(exec_test("tests/CPUTEST.COM"), 0x3B25);
    }
    // #[test]
    fn all_tests() {
        assert_eq!(exec_test("tests/prelim.com"), 0x32F);
        assert_eq!(exec_test("tests/8080PRE.COM"), 0x000);
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
        // i.cpu.debug = true;
        let _cycles = 0;

        loop {
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
            let af = ((i.cpu.reg.a as u16) << 8 |  (i.cpu.flags.get() as u16));
            if af == 0x0040 && i.cpu.reg.sp == 0x2fff && i.cpu.cycles == 14810168 {
                eprintln!("{:#?}", i.cpu.flags);
                eprintln!("{:?}", i.cpu);
                eprintln!("{}", i.cpu);
                panic!();
            }

            if i.cpu.reg.pc == 0 {
                println!(" Jump to 0 from {:04X}", i.cpu.reg.prev_pc);
                println!("Cycles executed: {}", i.cpu.cycles);
                break;
            }
        }
        i.cpu.reg.prev_pc
    }
}
