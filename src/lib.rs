#![no_std]
extern crate libw;
use libw::*;
extern crate alloc;
use alloc::string::ToString;

const XTERM_ESCAPE: &'static str = "\u{001b}";

pub fn clear() {
    print("\u{001b}c");
}

pub fn dimensions() -> (u32, u32) {
    let mut rows = 0;
    let mut cols = 0;
    let vars = environment_variables();
    for v in vars {
        if v.name == "LINES" {
            rows = v.value.parse::<u32>().unwrap()
        }
        if v.name == "COLUMNS" {
            cols = v.value.parse::<u32>().unwrap()
        }
    }
    (rows, cols)
}

pub fn set_cursor_position(row: u32, col: u32) {
    let mut cmd = XTERM_ESCAPE.to_string();
    cmd.push_str("[");
    cmd.push_str(&(row+1).to_string());
    cmd.push_str(";");
    cmd.push_str(&(col+1).to_string());
    cmd.push_str("f");
    print(&cmd);
}

pub fn save_cursor_position() {
    print("\u{001b}[s");
}

pub fn restore_cursor_position() {
    print("\u{001b}[u");
}

pub fn set_foreground_color(r: u8, g: u8, b: u8) {
    let mut cmd = XTERM_ESCAPE.to_string();
    cmd.push_str("[38;2;");
    cmd.push_str(&r.to_string());
    cmd.push_str(";");
    cmd.push_str(&g.to_string());
    cmd.push_str(";");
    cmd.push_str(&b.to_string());
    cmd.push_str("m");
    print(&cmd);
}

pub fn set_background_color(r: u8, g: u8, b: u8) {
    let mut cmd = XTERM_ESCAPE.to_string();
    cmd.push_str("[48;2;");
    cmd.push_str(&r.to_string());
    cmd.push_str(";");
    cmd.push_str(&g.to_string());
    cmd.push_str(";");
    cmd.push_str(&b.to_string());
    cmd.push_str("m");
    print(&cmd);
}

pub fn reset_color() {
    print("\u{001b}[0m");
}

pub fn bold() {
    print("\u{001b}[1m");
}

pub fn italic() {
    print("\u{001b}[3m");
}

pub fn slow_blink() {
    print("\u{001b}[5m");
}

pub fn fast_blink() {
    print("\u{001b}[6m");
}

pub fn underline() {
    print("\u{001b}[4m");
}

pub fn hide_cursor() {
    print("\u{001b}[?25l");
}

pub fn show_cursor() {
    print("\u{001b}[?25l");
}