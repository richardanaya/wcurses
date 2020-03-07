#![no_std]
extern crate wcurses;
use wcurses::*;
extern crate libw;
use libw::*;
extern crate alloc;
use alloc::string::ToString;

#[no_mangle]
pub fn _start() {
    let mut old_time = current_time();
    clear();

    hide_cursor();
    loop {
        // show time since last old time
        let cur_time = current_time();
        reset_color();
        set_cursor_position(0,0);
        let mut frame_time = "frame time: ".to_string();
        frame_time.push_str(&(cur_time-old_time).to_string());
        frame_time.push_str("ms");
        print(&frame_time);
        old_time = cur_time;

        //draw 
        let (rows,cols) = dimensions();
        for r in 0..rows as u32 {
            // safe room for frame time at top
            let mut x = 0 as u32;
            if r == 0 {
                x = frame_time.len() as u32 + 1;
            }
            set_cursor_position(r,x as u32);
            for c in x..cols  {
                // each row column cell gets a random color
                set_foreground_color((random_number()*255.0) as u8, (random_number()*255.0) as u8, (random_number()*255.0) as u8 );
                print(&"@");
            }
        }
    }
}
