use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct TextRow {
    raw_text: String,
    text: String,
}

impl TextRow {
    pub fn new(text: String) -> Self {
        let copy = text.clone();
        Self {
            raw_text: text,
            text: copy,
        }
    }

    pub fn truncate(&mut self, l: u16) -> &mut TextRow {
        self.text.truncate(l.into());
        self
    }

    pub fn substring(&mut self, start: i16) -> &mut TextRow {
        let len = self.raw_text.len();
        self.text = self.raw_text.chars().skip(start.try_into().unwrap()).take(len).collect();
        self
    }

    pub fn length(&self) -> i16 {
        (self.raw_text.graphemes(true).count() as i16) + 1
    }
}

impl fmt::Display for TextRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
