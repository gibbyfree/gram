use crate::{
    backend::{cursor::CursorHandler, operations::OperationsHandler},
    data::{enums::Direction, textrow::TextRow},
    gfx::render::RenderDriver,
    utils,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

// RenderController. Parses user input and calls the appropriate processing / rendering methods within the crate.
// Contains a CursorHandler and OperationsHandler on initialization.
pub struct RenderController {
    cursor: CursorHandler,
    operations: OperationsHandler,
    file_name: String,
}

impl RenderController {
    // Create a CursorHandler under current window size. Pass its blank CursorState to a RenderDriver.
    // Uses this RenderDriver to construct an OperationsHandler.
    // Note: No support whatsoever for mid-session window resizing (yet).
    pub fn new() -> Self {
        let window_size = utils::get_window_size();
        let mut cursor = CursorHandler::new(window_size.rows, window_size.cols);
        let state = cursor.get_state();
        let render = RenderDriver::new(state);
        Self {
            cursor,
            operations: OperationsHandler::new(render),
            file_name: "".to_string(),
        }
    }

    // From a Move InputEvent's Direction, tell the CursorHandler to handle cursor movement.
    // Then give the updated CursorState to the RenderDriver.
    pub fn queue_move(&mut self, d: Direction) {
        let data = self.operations.get_text();
        match d {
            Direction::Down => self.cursor.handle_cursor(false, self.cursor.cy + 1, data),
            Direction::Up => self.cursor.handle_cursor(false, self.cursor.cy - 1, data),
            Direction::Left => self.cursor.handle_cursor(true, self.cursor.cx - 1, data),
            Direction::Right => self.cursor.handle_cursor(true, self.cursor.cx + 1, data),
            _ => (),
        }
        self.operations.update_cursor_state(self.cursor.get_state());
    }

    // From a Page InputEvent's Direction, tell the CursorHandler to handle cursor movement.
    // Then give the updated CursorState to the RenderDriver.
    pub fn queue_scroll(&mut self, d: Direction) {
        let data = self.operations.get_text();
        match d {
            Direction::Up => self.cursor.handle_scroll(false, true, data),
            Direction::Down => self.cursor.handle_scroll(false, false, data),
            Direction::Left => self.cursor.handle_scroll(true, true, data),
            Direction::Right => self.cursor.handle_scroll(true, false, data),
            _ => (),
        }
        self.operations.update_cursor_state(self.cursor.get_state());
    }

    // From a Write InputEvent's character, input the given character at the current cursor position.
    // Increment cursor with each write. Update CursorState for the RenderDriver.
    pub fn queue_write(&mut self, c: char) {
        self.operations.process_write(self.cursor.get_state(), c);
        self.cursor
            .handle_cursor(true, self.cursor.cx + 1, self.operations.get_text());
        self.operations.update_cursor_state(self.cursor.get_state());
    }

    // Queues a delete in the operation handler, and updates the cursor upon delete.
    pub fn queue_delete(&mut self, d: Direction) {
        match d {
            Direction::Left => {
                if self.cursor.cx > 0 {
                    self.operations.process_delete(self.cursor.get_state(), d);
                    self.cursor
                        .handle_cursor(true, self.cursor.cx - 1, self.operations.get_text());
                    self.operations.update_cursor_state(self.cursor.get_state());
                } else if self.cursor.cx == 0 && self.cursor.cy > 0 {
                    self.operations.process_wrap_delete(self.cursor.get_state(), d);
                    self.cursor.handle_cursor(false, self.cursor.cy - 1, self.operations.get_text());
                    self.operations.update_cursor_state(self.cursor.get_state())
                }
            },
            Direction::Right => {
                // currently, the controller isn't wired to conditionally call process_wrap_delete directly
                // use of this method relies on an understanding of the current line length,
                // and the operations handler is probably better suited to process this for now.
                self.operations.process_delete(self.cursor.get_state(), d);
            }
            _ => (),
        }
    }

    // Read the contents of a file at a given path, line-by-line.
    // Pass the filename to the renderer, for use in the status bar. (Save a copy of it, for when we want to save the file later.)
    // Pass this vec of strings to the next method, for upload to the RenderDriver.
    pub fn read_file(&mut self, s: &str) {
        let file = File::open(s).expect("File not found at the given location.");
        let buf = BufReader::new(file);

        let mut vec: Vec<String> = Vec::new();
        for line in buf.lines() {
            vec.push(line.unwrap());
        }
        let ctrl_cpy = s.clone();
        self.operations.set_file_name(s);
        self.file_name = ctrl_cpy.to_string();

        self.queue_text_upload(&vec);
    }

    pub fn write_file(&mut self) {
        self.operations.write_file(&self.file_name)
    }

    // Parse a vec of strings into a vec of TextRows.
    // Pass this vec of TextRows to the RenderDriver.
    pub fn queue_text_upload(&mut self, vec: &Vec<String>) {
        let mut output: Vec<TextRow> = Vec::new();
        for text in vec {
            let str = String::from(text.trim());
            let row = TextRow::new(str);
            output.push(row);
        }
        self.operations.set_text(output)
    }

    // Complete RenderDriver initialization without a text upload.
    pub fn finish_early(&mut self) {
        self.operations.complete_init();
    }

    // Tell the RenderDriver to shutdown the editor.
    // Return a bool that represents whether to shutdown the editor.
    pub fn exit(&mut self) -> bool {
        // self.write_file(); autosave disabled for now i guess
        self.operations.exit()
    }

    // Tell the RenderDriver to continue processing.
    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.operations.tick_screen()
    }
}
