extern crate minifb;

use crate::pacman::pacman::Pacman;

mod cpu;
mod formatter;
mod instruction_info;
mod interconnect;
mod memory;
mod tests;
mod pacman {
    pub mod display;
    pub mod keypad;
    pub mod pacman;
}
fn main() {
    let mut pac = Pacman::new();
    let args: Vec<String> = std::env::args().collect();
    // pac.ctx.cpu.memory.load_bin(&args);
    pac.load_rom(&args);
    pac.init();
    loop {
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        pac.ctx.execute_cpu();
        // i.keypad.key_down(&mut i.cpu.io, &display.window);

        pac.render_tiles();
        pac.fb.window.update_with_buffer(&pac.fb.raster).unwrap();

        /*if i.frame_count % 5 == 1 {
            i.keypad.reset_ports(&mut i.cpu.io);
        }*/
    }
}
