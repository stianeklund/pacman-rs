use super::cpu::CPU;
use crate::display::Display;
use crate::keypad::Keypad;

pub struct Interconnect {
    pub cpu: CPU,
    pub keypad: Keypad,
    pub frame_count: u32,
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            keypad: Keypad::new(),
            frame_count: 0,
        }
    }

    pub fn execute_cpu(&mut self) -> u32 {
        // self.cpu.debug = true;
        let mut cycles_executed: usize = 0;

        self.cpu.debug = true;
        while cycles_executed <= 16666 {
            let start_cycles = self.cpu.cycles;
            self.cpu.execute_instruction();
            if self.cpu.debug {
                println!("{:?}", self.cpu);
            }
            cycles_executed += self.cpu.cycles - start_cycles;
            self.cpu.try_interrupt();
        }

        self.frame_count += 1;
        return self.frame_count;
    }

    // Step once when pressing a key
    pub fn run_tests(&mut self) {
        if self.cpu.debug {
            // println!("{}", self.cpu);
        }
        // self.execute_cpu();
        self.cpu.execute_tests();
    }
}
