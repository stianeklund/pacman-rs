use minifb::{Key, KeyRepeat, Window};

use crate::z80_rs::cpu::Io;

/*  Pacman specific dip switch configurations
*
*  Bits  Value   Description
*  --------------------------
*                Coins per game:
*  0,1   0       Free play
*        1       1 coin per game
*        2       1 coin per 2 games
*        3       2 coins per game
*
*                Lives per game:
*  2,3   0       1 life
*        1       2 lives
*        2       3 lives
*        3       5 lives
*
*                Bonus score (extra life):
* 4,5    0       10_000 points
*        1       15_000
*        2       20_000
*        3       0
*                Difficulty (jumper pad):
* 6      0,1     (Hard, Normal)
*                Ghost names (jumper pad):
* 7      0,1     (Alternate, Normal)

*/

pub struct Keypad {}

pub trait Input {
    fn key_value(&self);
    fn key_down(&mut self, reg: &mut Io, window: &Window);
    fn key_up(&mut self, reg: &mut Io, window: &Window);
    fn reset_ports(&self, reg: &mut Io);
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {}
    }
}

/*impl Input for Keypad {
    fn key_value(&self) -> &Keypad {
        self.borrow()
    }
    fn key_down(&mut self, io: &mut Io, window: &Window) {
        if window.is_open() {
            window
                .get_keys_pressed(KeyRepeat::Yes)
                .unwrap()
                .iter()
                .for_each(|keys| match keys {
                    Key::Enter => io.port_1_in |= self.p1_start,
                    Key::C => io.port_1_in |= self.credit,
                    Key::Space => io.port_1_in |= self.p1_fire,
                    Key::Key2 => io.port_2_in |= self.p2_start,
                    Key::Key3 => io.port_2_in |= self.coin_info,
                    Key::Left => io.port_1_in |= self.p1_left,
                    Key::Right => io.port_1_in |= self.p1_right,
                    Key::Escape => std::process::exit(0),
                    _ => eprintln!("Key: {:?} not implemented", *keys),
                });
        }
    }

    fn key_up(&mut self, io: &mut Io, window: &Window) {
        // TODO Improve handling
        // Problem here is likely that the keys pressed are not the same
        // as in `key_down()`

        if window.is_open() {
            window
                .get_keys()
                .unwrap()
                .iter()
                .for_each(|keys| match keys {
                    Key::Enter => io.port_1_in &= !self.p1_start,
                    Key::C => io.port_1_in &= !self.credit,
                    Key::Space => io.port_1_in &= !self.p1_fire,
                    Key::Key2 => io.port_2_in &= !self.p2_start,
                    Key::Key3 => io.port_2_in &= !self.coin_info,
                    Key::Left => io.port_1_in &= !self.p1_left,
                    Key::Right => io.port_1_in &= !self.p1_right,
                    _ => eprintln!("Key: {:?} not implemented", *keys),
                });
        }
    }

    fn reset_ports(&self, io: &mut Io) {
        io.port_1_in &= !self.credit;
        io.port_1_in &= !self.p1_left;
        io.port_1_in &= !self.p1_right;
        io.port_1_in &= !self.p1_fire;
        io.port_1_in &= !self.p1_start;
        io.port_2_in &= !self.p2_start;
        io.port_2_in &= !self.coin_info;
    }
}*/
