extern crate minifb;
extern crate z80_rs;

use flexi_logger::{Logger};
use log::info;

use crate::pacman_arcade::pacman::Pacman;
use crate::pacman_arcade::display::{WIDTH, HEIGHT};
use std::time::Duration;

mod pacman_arcade {
    pub mod display;
    pub mod keypad;
    pub mod pacman;
}

fn main() {
    Logger::with_str("z80")
        .log_to_file()
        .directory("log")
        // .format(default_format)
        .start()
        .unwrap();
    let args: Vec<String> = std::env::args().collect();
    // pac.ctx.cpu.debug = true;
    // pac.ctx.cpu.memory.load_bin(&args);
    let mut pac = Pacman::new();

    pac.ctx.cpu.reset();
    pac.load_rom(&args);
    pac.init();

    let mut sprite = 1;
    let pal_no = 1;
    let mut x = 0;
    let mut y = 0;

    let mut i = 1;
    loop {
        pac.ctx.execute_cpu();
        // pac.draw_screen();
        // pac.fb.draw_sprite(y, x, sprite, pal_no);
        // pac.render_tiles();
        pac.fb.window.is_key_down(minifb::Key::Enter).then(|| {
            pac.fb.draw_sprite(x, y, sprite, i, 1);
            i+=1;
            if i == 64 {
                i = 0;
            }
        });
        pac.fb.window.get_mouse_down(minifb::MouseButton::Right).then(|| panic!("Sprite index:{}", i));
        pac.fb.window.get_mouse_down(minifb::MouseButton::Left).then(|| {
            // x += 8;
            y +=8;
        });

        pac.fb.window.update_with_buffer(&pac.fb.raster, WIDTH as usize, HEIGHT as usize).unwrap();
        // sleep(Duration::from_micros(16));
    }
}
