use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

// TextRow. Represents a line of text in the editor.
// raw_text: The complete text of the line.
// text: Modified text line, truncated or substringed.
#[derive(Default)]
pub struct TextRow {
    pub raw_text: String,
    text: String,
}

impl TextRow {
    // TextRows are constructed with a String.
    pub fn new(text: String) -> Self {
        let copy = text.clone();
        Self {
            raw_text: text,
            text: copy,
        }
    }

    // Truncate a TextRow's text to a given length.
    pub fn truncate(&mut self, l: u16) -> &mut TextRow {
        self.text.truncate(l.into());
        self
    }

    // Take a substring of a TextRow's text, starting at a given index.
    pub fn substring(&mut self, start: i16) -> &mut TextRow {
        let len = self.raw_text.len();
        self.text = self.raw_text.chars().skip(start.try_into().unwrap()).take(len).collect();
        self
    }

    // Returns the length of a TextRow's text, using grapheme clusters. 
    // (This should best represent what a human using a text editor understands to be a character.)
    pub fn length(&self) -> i16 {
        (self.raw_text.graphemes(true).count() as i16) + 1
    }

    // Updates the text of a TextRow.
    pub fn update_text(&mut self, text: String) {
        self.raw_text = text.clone();
        self.text = text;
    }

    // Returns a list of all indices in the row where a given substr begins.
    // I have a feeling there's going to be something wrong with this method, since it's not using a grapheme representation.
    pub fn find_idx_at_substr(&self, substr: &String) -> Vec<(usize, &str)> {
        let substr_len = substr.len();
        self.raw_text.match_indices(substr)
            .map(|(idx, match_str)| (idx + substr_len, match_str))
            .collect::<Vec<(_, _)>>()
    }
}

impl fmt::Display for TextRow {
    // We display a TextRow by printing out its text.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
