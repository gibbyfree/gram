use crate::data::{
    enums::{Direction, InputEvent, PromptResult, StatusContent},
    payload::{SearchItem},
};
use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
};

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    backend::prompt::PromptProcessor,
    data::{payload::CursorState, textrow::TextRow},
    gfx::render::RenderDriver,
};

// OperationsHandler. Its purpose in life is to manipulate the fields of a RenderDriver.
pub struct OperationsHandler {
    render: RenderDriver,
    file_name: String,
    prompt: PromptProcessor,
}

impl OperationsHandler {
    pub fn new(render: RenderDriver) -> Self {
        Self {
            render,
            file_name: "[Untitled]".to_string(),
            prompt: PromptProcessor::new(),
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

    // Meant to be called after we've done something with PromptProc. We check its current status,
    // and if it's a data-bearing status (save-as prompt, find prompt) then we push this data for render.
    // Might return a PromptResult if this is an incremental prompt. Honestly, this doesn't have to be done here,
    // but I'd rather not have to sus out StatusContent type more than once for a single user input.
    fn check_and_update_prompt_status(&mut self) -> Option<PromptResult> {
        let status = &self.prompt.status;
        if let Some(content) = status {
            if let StatusContent::SaveAs(str) = content {
                self.render
                    .update_status_message(StatusContent::SaveAs(str.to_string()));
                None
            } else if let StatusContent::Find(s) = content {
                self.render
                    .update_status_message(StatusContent::Find(s.to_string()));
                Some(PromptResult::TextSearch(s.to_string()))
            } else {
                None
            }
        } else {
            None
        }
    }

    // Returns the length at a given line, in graphemes (best understood as a human readable character in this context).
    // Nothing about this is very operations-y, but this is the easiest way to surface line lengths to the controller.
    pub fn get_length_at_line(&mut self, idx: usize) -> usize {
        self.get_graphemes_at_line(idx).len()
    }

    // Initializes the PromptProc based on a given InputEvent.
    // Prompt is flushed, initial status is set, and render's status is updated if necessary.
    pub fn initialize_prompt(&mut self, kind: InputEvent) {
        match kind {
            InputEvent::Save => {
                self.prompt.flush();
                self.prompt
                    .set_status(StatusContent::SaveAs("".to_string()));
                self.check_and_update_prompt_status();
            }
            InputEvent::Find => {
                self.prompt.flush();
                self.prompt.set_status(StatusContent::Find("".to_string()));
                self.check_and_update_prompt_status();
            }
            _ => (),
        }
    }

    // Deletes a character at the current cursor position.
    // Given direction determines whether the character before or after the cursor is deleted.
    pub fn process_delete(&mut self, cursor: CursorState, d: Direction) {
        let idx = cursor.cy as usize;
        let mut g = self.get_graphemes_at_line(idx);

        let mut target = g.len();
        match d {
            Direction::Left => target = (cursor.cx - 1 + cursor.col_offset) as usize,
            Direction::Right => target = (cursor.cx + cursor.col_offset) as usize,
            _ => (),
        }

        if g.len() > target {
            g.remove(target);
            let updated: String = g.into_iter().map(String::from).collect();
            self.render.set_text_at_index(idx, updated);
        } else if target == (cursor.cx + cursor.col_offset).try_into().unwrap()
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
            }
            Direction::Right => {
                adj_row_idx = idx + 1;
                del_row_idx = idx + 1;
                insert_idx = idx;
            }
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

    // Inputs a newline. Split the string at the current cursor, pushes everything ahead of the cursor to the newline.
    pub fn process_newline(&mut self, cursor: CursorState) {
        let y = cursor.cy as usize;
        let x = cursor.cx as usize;
        let str = self.get_string_at_line(y).to_owned();
        let (left, right) = str.split_at(x);

        self.render.set_text_at_index(y, left.to_string());
        self.render
            .insert_row(y + 1, TextRow::new(right.to_string()));
    }

    // Insert a given character at the current cursor position.
    // Graphemes are used in inserting the new character, since this is the best representation of a human-readable character in a text editor.
    pub fn process_write(&mut self, cursor: CursorState, c: char) {
        let idx = cursor.cy as usize;
        let mut g = self.get_graphemes_at_line(idx);
        let mut tmp = [0u8; 4];
        // if we are starting insertion at the very end of the line, add a space
        if g.len() < cursor.cx.try_into().unwrap() {
            g.insert(g.len(), " ");
        }
        g.insert(
            (cursor.cx + cursor.col_offset) as usize,
            c.encode_utf8(&mut tmp),
        );
        let updated: String = g.into_iter().map(String::from).collect();

        self.render.set_text_at_index(idx, updated);
    }

    // Tears down all data stored in PromptProc, and clears whatever StatusMessage is currently rendered.
    // Currently does this by sending SaveAbort, but this will probably be changed when more prompt interactions are added.
    pub fn wipe_prompt(&mut self) {
        self.prompt.flush();
        self.render.update_status_message(StatusContent::SaveAbort);
    }

    // Processes current data collected in the prompt.
    // For a SaveAs prompt, user data should be used to set a new file name.
    // Sends a PromptResult to the controler, so that it can wrap-up any other processes as needed.
    pub fn process_prompt_confirm(&mut self) -> Option<PromptResult> {
        let status = &self.prompt.status;
        if let Some(content) = status {
            if let StatusContent::SaveAs(str) = content {
                self.file_name = str.to_string();
                self.render.set_file_name(str);
                self.render
                    .update_status_message(StatusContent::SaveSuccess);
                return Some(PromptResult::FileRename(str.to_string()));
            }
        }
        None
    }

    pub fn search_text(&mut self, query: &String) -> Vec<SearchItem> {
        let mut res: Vec<SearchItem> = Vec::new();
        let data: &mut Vec<TextRow> = self.render.get_text();

        for (i, row) in data.iter().enumerate() {
            let indices = row.find_idx_at_substr(&query);
            for j in indices {
                res.push(SearchItem::new(j.0, i));
            }
        }
        
        res
    }

    // Processes writes to the prompt. It's very similar to processing writes to render,
    // except the only text data for a prompt is a single TextRow. It's also necessary to update
    // render's status message based on the new prompt text content.
    // If this prompt interaction is related to an incremental prompt (like find), it might return a PromptResult to
    // pass relevant data back to the controller.
    pub fn process_prompt(&mut self, c: char) -> Option<PromptResult> {
        let mut g = self
            .prompt
            .text
            .raw_text
            .graphemes(true)
            .collect::<Vec<&str>>();
        let mut tmp = [0u8; 4];
        g.insert((self.prompt.cx) as usize, c.encode_utf8(&mut tmp));
        let updated: String = g.into_iter().map(String::from).collect();

        self.prompt.set_text(updated);
        let res = self.check_and_update_prompt_status();
        self.process_prompt_cursor(1);
        res
    }

    // Handles the prompt's basic cursor functionality. The prompt's text field is a single row,
    // and the cursor should adjust according to input. Currently, not really worrying about scrolling for
    // super long file names or anything like that.
    // x represents the # that should be added onto cx. Can be negative to represent backwards movement.
    pub fn process_prompt_cursor(&mut self, x: i16) {
        let idx = self.prompt.cx + x;
        let g = self
            .prompt
            .text
            .raw_text
            .graphemes(true)
            .collect::<Vec<&str>>();

        if idx >= 0 && idx as usize <= g.len() {
            self.prompt.set_cursor(idx);
        }
    }

    // Handles deletion in the prompt. Since text doesn't wrap in the prompt, this is a much simpler implementation than deletion in the editor.
    // Trundles along the cursor depending on which direction we're deleting from.
    pub fn process_prompt_delete(&mut self, left: bool) {
        let mut g = self
            .prompt
            .text
            .raw_text
            .graphemes(true)
            .collect::<Vec<&str>>();
        let idx = self.prompt.cx as usize;

        if left && idx > 0 && (idx - 1) < g.len() {
            g.remove(idx - 1);
        } else if !left && idx < g.len() {
            g.remove(idx + 1);
        }
        let updated: String = g.into_iter().map(String::from).collect();

        self.prompt.set_text(updated);
        self.check_and_update_prompt_status();
        if left {
            self.process_prompt_cursor(-1);
        }
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
            if !name.contains(".txt") {
                let path = format!("{}.txt", name);
                f = File::create(path).unwrap();
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

    pub fn check_file_name(&mut self) -> bool {
        self.file_name.eq("[Untitled]")
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
