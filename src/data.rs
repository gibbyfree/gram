use std::fmt;

pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct TextRow {
    text: String,
}

impl TextRow {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }
}

impl fmt::Display for TextRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}