use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

fn read_key() -> Key {
    if let Some(b) = stdin().keys().next() {
        return b.unwrap();
    }
    Key::Null
}

pub fn proc_key() -> u8 {
    let k = read_key();

    match k {
        Key::Ctrl('q') => return 0,
        _ => return 1,
    };
}
