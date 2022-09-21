use termion::raw::IntoRawMode;
use std::io::{self, Read, stdout};

pub fn main() {
    let mut c: char;
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        c = b as char;
        if c.is_control() {            
            println!("{:?} \r", b);            
        } else {            
            println!("{:?} ({})\r", b, c);            
        }
        if c == 'q' {
            break;
        }
    }
}