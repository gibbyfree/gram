use crate::{gfx::{render::RenderDriver}, data::{enums::Direction, textrow::TextRow}, utils, backend::{cursor::CursorHandler, operations::OperationsHandler}};
use std::{io::{Error, BufReader, BufRead}, fs::File};

// RenderController. Parses user input and calls the appropriate processing / rendering methods within the crate.
// Contains a RenderDriver, CursorHandler, and terminal window size upon initialization.
// Note: No support whatsoever for mid-session window resizing (yet).
pub struct RenderController {
    cursor: CursorHandler,
    operations: OperationsHandler
}

impl RenderController {
    // Create a CursorHandler under current window size. Pass its blank CursorState to the RenderDriver.
    pub fn new() -> Self {
        let window_size = utils::get_window_size();
        let mut cursor = CursorHandler::new(window_size.rows, window_size.cols);
        let state = cursor.get_state();
        let render = RenderDriver::new(state);
        Self {
            cursor,
            operations: OperationsHandler::new(render),
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

    // Read the contents of a file at a given path, line-by-line.
    // Pass the filename to the renderer, for use in the status bar.
    // Pass this vec of strings to the next method, for upload to the RenderDriver.
    pub fn read_file(&mut self, s: &str) {
        let file = File::open(s).expect("File not found at the given location.");
        let buf = BufReader::new(file);

        let mut vec: Vec<String> = Vec::new();
        for line in buf.lines() {
            vec.push(line.unwrap());
        }
        self.operations.set_file_name(s);
        self.queue_text_upload(&vec);
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

    // Tell the RenderDriver to shutdown the editor.
    pub fn exit(&mut self) {
        self.operations.exit();
    }

    // Tell the RenderDriver to continue processing.
    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.operations.tick_screen()
    }

}