use termion::raw::IntoRawMode;
use std::io::{self, Read, Write, stdout};

// "data definitions" but not really
fn is_quit_byte(b: &u8) -> bool {
    *b == 17 as u8
}

// terminal
fn read_key() -> u8 {
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        return b;
    }
    0
}

// input
fn proc_key() -> u8 {
    let b = read_key();

    match b {
        b if is_quit_byte(&b) => return 0,
        _ => return 1,
    };
}

// output
fn draw_lhs_edge() {
    for _n in 1..25 {
        println!("~\r")
    }
}

fn tick_screen() -> u8 {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    draw_lhs_edge();
    print!("{}", termion::cursor::Goto(1, 1));
    let res = io::stdout().flush();
    match res {
        Ok(_) => return 1,
        Err(_) => return 0,
    };
}

// driver
pub fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    let (mut exit, mut err) = (1, 1);

    while exit != 0 && err != 0 {
        err = tick_screen();
        exit = proc_key();
    }

    print!("{}", termion::clear::All);
}