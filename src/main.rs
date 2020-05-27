extern crate minifb;

use crate::interconnect::Interconnect;
use crate::pacman::display;

mod cpu;
mod formatter;
mod instruction_info;
mod interconnect;
mod memory;
mod tests;
mod pacman {
    pub mod display;
    pub mod keypad;
    mod pacman;
}
fn main() {
    let i = &mut Interconnect::new();
    let args: Vec<String> = std::env::args().collect();
    let mut display = display::Display::new();
    i.cpu.memory.load_bin(&args);
    // i.cpu.debug = true;

    loop {
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        i.execute_cpu();
        // i.keypad.key_down(&mut i.cpu.io, &display.window);

        display.draw_pixel(&i.cpu.memory);
        display.window.update_with_buffer(&display.raster).unwrap();

        /*if i.frame_count % 5 == 1 {
            i.keypad.reset_ports(&mut i.cpu.io);
        }*/
    }
}
