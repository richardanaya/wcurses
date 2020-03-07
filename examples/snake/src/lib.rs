#![no_std]
extern crate wcurses;
use wcurses::*;
extern crate libw;
use libw::*;
extern crate alloc;
use alloc::string::ToString;

#[no_mangle]
pub fn _start() {
    clear();
    loop {
        hide_cursor();
        let (rows,cols) = dimensions();
        for r in 0..rows {
            set_cursor_position(r,0);
            for c in 0..cols {
                set_foreground_color((random_number()*255.0) as u8, (random_number()*255.0) as u8, (random_number()*255.0) as u8 );
                print(&"@");
            }
        }
        sleep(0);
    }
}
