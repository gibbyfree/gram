use crate::data::enums::{Direction, StatusContent};
use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
};

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    data::{payload::CursorState, textrow::TextRow},
    gfx::render::RenderDriver,
};

// OperationsHandler. Its purpose in life is to manipulate the fields of a RenderDriver.
pub struct OperationsHandler {
    render: RenderDriver,
    file_name: String,
}

impl OperationsHandler {
    pub fn new(render: RenderDriver) -> Self {
        Self {
            render,
            file_name: "[Untitled]".to_string(),
        }
    }

    // Get the raw text String of a textrow at the given index.
    // If a textrow does not exist at that index, we assume an empty string is the base of any insertion, and return it.
    fn get_string_at_line(&mut self, idx: usize) -> &String {
        let data: &mut Vec<TextRow> = self.render.get_text();
        if data.len() <= idx.try_into().unwrap() {
            data.push(TextRow::new("".to_string()));
        }
        &data[idx].raw_text
    }

    // Get the row text in String form, then convert it to graphemes, then collect to a Vec.
    // Mostly a wrapper at this point.
    fn get_graphemes_at_line(&mut self, idx: usize) -> Vec<&str> {
        let row_text = self.get_string_at_line(idx);
        row_text.graphemes(true).collect::<Vec<&str>>()
    }

    pub fn get_length_at_line(&mut self, idx: usize) -> usize {
        self.get_graphemes_at_line(idx).len()
    }

    // Deletes a character at the current cursor position.
    // Given direction determines whether the character before or after the cursor is deleted.
    pub fn process_delete(&mut self, cursor: CursorState, d: Direction) {
        let idx = cursor.cy as usize;
        let mut g = self.get_graphemes_at_line(idx);

        let mut target = g.len();
        match d {
            Direction::Left => target = (cursor.cx - 1) as usize,
            Direction::Right => target = cursor.cx as usize,
            _ => (),
        }

        if g.len() > target {
            g.remove(target);
            let updated: String = g.into_iter().map(String::from).collect();
            self.render.set_text_at_index(idx, updated);
        } else if target == cursor.cx.try_into().unwrap()
            && (cursor.cy + 1) < self.render.get_text().len().try_into().unwrap()
        {
            self.process_wrap_delete(cursor, d);
        }
    }

    // Processes a "wrapping delete". A wrapping delete is a delete that results in the deletion of an adjacent row.
    // Wrapping deletes can go forwards or backwards (from DEL or BS).
    // Operation is basically the same in both directions, but with different indices.
    pub fn process_wrap_delete(&mut self, cursor: CursorState, d: Direction) {
        let idx = cursor.cy as usize;
        let curr_row = self.get_string_at_line(idx).to_owned();

        // conditionally determine relevant indices
        // maybe these definitions can be refactored out to some kind of const struct
        let mut adj_row_idx = idx;
        let mut del_row_idx = idx;
        let mut insert_idx = idx;
        match d {
            Direction::Left => {
                adj_row_idx = idx - 1;
                del_row_idx = idx;
                insert_idx = idx - 1;
            },
            Direction::Right => {
                adj_row_idx = idx + 1;
                del_row_idx = idx + 1;
                insert_idx = idx;
            },
            _ => (),
        }

        // get new row text
        let adj_row = self.get_string_at_line(adj_row_idx).to_owned();
        let new_row_text = if adj_row_idx == (idx + 1) {
            format!("{}{}", curr_row, adj_row)
        } else {
            format!("{}{}", adj_row, curr_row)
        };

        // delete, insert merged text
        self.render.delete_row(del_row_idx);
        self.render.set_text_at_index(insert_idx, new_row_text);
    }

    // Insert a given character at the current cursor position.
    // Graphemes are used in inserting the new character, since this is the best representation of a human-readable
    // character in a text editor.
    pub fn process_write(&mut self, cursor: CursorState, c: char) {
        let idx = cursor.cy as usize;
        let mut g = self.get_graphemes_at_line(idx);
        let mut tmp = [0u8; 4];
        // if we are starting insertion at the very end of the line, add a space
        if g.len() < cursor.cx.try_into().unwrap() {
            g.insert(g.len(), " ");
        }
        g.insert(cursor.cx as usize, c.encode_utf8(&mut tmp));
        let updated: String = g.into_iter().map(String::from).collect();

        self.render.set_text_at_index(idx, updated);
    }

    // Writes to a file at a given file name.
    // Collates all of RenderDriver's data to a string.
    // If a file name was set (i.e. arg mode), data is written to the modified file.
    // If a file name was not set, data is written to a new file. Filler file name for now.
    // After the file is written, update RenderDriver's status message to reflect the successful disk write.
    pub fn write_file(&mut self, name: &str) {
        if !self.render.is_quitting() {
            let data: &Vec<TextRow> = self.render.get_text();
            let mut output = String::from("");

            for t in data {
                output.push_str(&t.raw_text);
                output.push_str("\n");
            }

            let mut f: File;
            if self.file_name.eq("[Untitled]") {
                f = File::create("filler_file_name.txt").unwrap();
            } else {
                f = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(name)
                    .unwrap();
            }

            f.write_all(output.as_bytes()).unwrap();
            self.render
                .update_status_message(StatusContent::SaveSuccess);
        }
    }

    // WRAPPER METHODS //
    // Wrapper around RenderDriver's get_text.
    pub fn get_text(&mut self) -> &Vec<TextRow> {
        self.render.get_text()
    }

    // Wrapper around RenderDriver's set_text.
    pub fn set_text(&mut self, text: Vec<TextRow>) {
        self.render.set_text(text);
    }

    // Wrapper around RenderDriver's update_cursor_state.
    pub fn update_cursor_state(&mut self, state: CursorState) {
        self.render.update_cursor_state(state);
    }

    // Wrapper around RenderDriver's set_file_name.
    pub fn set_file_name(&mut self, name: &str) {
        self.render.set_file_name(name);
        self.file_name = name.to_string();
    }

    // Wrapper around RenderDriver's tick_screen.
    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

    // Wrapper around RenderDriver's exit.
    pub fn exit(&mut self) -> bool {
        self.render.exit()
    }

    // Wrapper around RenderDriver's complete_init.
    pub fn complete_init(&mut self) {
        self.render.complete_init();
    }
    // END OF WRAPPER METHODS //
}
