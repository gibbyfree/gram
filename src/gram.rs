use termion::raw::IntoRawMode;
use std::io::{self, Read, stdout};

fn is_quit_byte(b: &u8) -> bool {
    *b == 17 as u8
}

fn editor_read_key() -> u8 {
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        return b;
    }
    0
}

fn editor_proc_key() -> u8 {
    let b = editor_read_key();

    match b {
        b if is_quit_byte(&b) => return 0,
        _ => return 1,
    };
}

pub fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    let mut exit = 1;

    while exit != 0 {
        exit = editor_proc_key();
    }
}