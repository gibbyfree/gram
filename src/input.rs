use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;
use crate::data::{InputEvent, Direction};

fn read_key() -> Key {
    if let Some(b) = stdin().keys().next() {
        return b.unwrap();
    }
    Key::Null
}

pub fn proc_key() -> Option<InputEvent> {
    let k = read_key();

    match k {
        Key::Ctrl('q') => return Some(InputEvent::Quit),
        Key::Up => return Some(InputEvent::Move(Direction::Up)),
        Key::Left => return Some(InputEvent::Move(Direction::Left)),
        Key::Down => return Some(InputEvent::Move(Direction::Down)),
        Key::Right => return Some(InputEvent::Move(Direction::Right)),
        Key::PageUp => return Some(InputEvent::Page(Direction::Up)),
        Key::PageDown => return Some(InputEvent::Page(Direction::Down)),
        Key::Home => return Some(InputEvent::Page(Direction::Left)),
        Key::End => return Some(InputEvent::Page(Direction::Right)),
        _ => return None,
    };
}
