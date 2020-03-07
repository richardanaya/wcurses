#![no_std]
extern crate wcurses;
use wcurses::*;
extern crate libw;
use libw::*;

#[no_mangle]
pub fn _start() {
    set_foreground_color(40, 60, 200);
    set_background_color(255, 255, 200);
    bold();
    print("foo");
    reset_color();
    underline();
    print("go");
}
