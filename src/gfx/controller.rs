use crate::{
    backend::{cursor::CursorHandler, operations::OperationsHandler},
    data::{
        enums::{Direction, PromptResult, WriteMode},
        textrow::TextRow,
    },
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
    mode: WriteMode,
}

impl RenderController {
    // Create a CursorHandler under current window size. Pass its blank CursorState to a RenderDriver.
    // Uses this RenderDriver to construct an OperationsHandler.
    // Write mode is set to Editor initially.
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
            mode: WriteMode::Editor,
        }
    }

    // From a Move InputEvent's Direction, tell the CursorHandler to handle cursor movement.
    // Then give the updated CursorState to the RenderDriver.
    // It's also possible to move left and right in the prompt field.
    pub fn queue_move(&mut self, d: Direction) {
        let data = self.operations.get_text();
        let mode = &self.mode;
        match (d, mode) {
            (Direction::Down, WriteMode::Editor) => {
                self.cursor.handle_cursor(false, self.cursor.cy + 1, data)
            }
            (Direction::Up, WriteMode::Editor) => {
                self.cursor.handle_cursor(false, self.cursor.cy - 1, data)
            }
            (Direction::Left, WriteMode::Editor) => {
                self.cursor.handle_cursor(true, self.cursor.cx - 1, data)
            }
            (Direction::Right, WriteMode::Editor) => {
                self.cursor.handle_cursor(true, self.cursor.cx + 1, data)
            }
            (Direction::Left, WriteMode::Prompt) => self.operations.process_prompt_cursor(-1),
            (Direction::Right, WriteMode::Prompt) => self.operations.process_prompt_cursor(1),
            _ => (),
        }
        self.operations.update_cursor_state(self.cursor.get_state());
    }

    // From a Page InputEvent's Direction, tell the CursorHandler to handle cursor movement.
    // Then give the updated CursorState to the RenderDriver.
    // No support for scrolling in prompt mode yet.
    pub fn queue_scroll(&mut self, d: Direction) {
        let data = self.operations.get_text();
        let mode = &self.mode;
        match (d, mode) {
            (Direction::Up, WriteMode::Editor) => self.cursor.handle_scroll(false, true, data),
            (Direction::Down, WriteMode::Editor) => self.cursor.handle_scroll(false, false, data),
            (Direction::Left, WriteMode::Editor) => self.cursor.handle_scroll(true, true, data),
            (Direction::Right, WriteMode::Editor) => self.cursor.handle_scroll(true, false, data),
            _ => (),
        }
        self.operations.update_cursor_state(self.cursor.get_state());
    }

    // From a Write InputEvent's character, input the given character at the current cursor position.
    // Increment cursor with each write. Update CursorState for the RenderDriver.
    // Conditonally processes newline inputs if a newline char is input.
    // Depending on current WriteMode, writes are processed using different handler methods.
    // In prompt mode, newline input is interpreted as prompt confirmation.
    pub fn queue_write(&mut self, c: char) {
        let mode = &self.mode;
        match (c, mode) {
            ('\n', WriteMode::Editor) => {
                self.operations.process_newline(self.cursor.get_state());
                self.cursor
                    .handle_cursor(false, self.cursor.cy + 1, self.operations.get_text());
                self.cursor
                    .handle_scroll(true, true, self.operations.get_text());
                self.operations.update_cursor_state(self.cursor.get_state());
            }
            (_, WriteMode::Editor) => {
                self.operations.process_write(self.cursor.get_state(), c);
                self.cursor
                    .handle_cursor(true, self.cursor.cx + 1, self.operations.get_text());
                self.operations.update_cursor_state(self.cursor.get_state());
            }
            ('\n', WriteMode::Prompt) | ('\t', WriteMode::Prompt) => {
                let res = self.operations.process_prompt_confirm();
                if let Some(pr) = res {
                    if let PromptResult::FileRename(str) = pr {
                        self.file_name = str;
                        self.write_file();
                        self.mode = WriteMode::Editor;
                    }
                }
            }
            (_, WriteMode::Prompt) => self.operations.process_prompt(c),
        }
    }

    // Queues a delete in the operation handler, and updates the cursor upon delete.
    // The logic for this operation is a bit more complex with a standard delete (deleting to the left of the cursor)
    // since this involves more fine-grained manipulation of the cursor post-delete.
    // Deletes in prompt mode are significantly simpler, since prompt content is a single TextRow.
    pub fn queue_delete(&mut self, d: Direction) {
        let mode = &self.mode;
        match (d, mode) {
            (Direction::Left, WriteMode::Editor) => {
                if self.cursor.cx > 0 {
                    self.operations.process_delete(self.cursor.get_state(), d);
                    self.cursor
                        .handle_cursor(true, self.cursor.cx - 1, self.operations.get_text());
                    self.operations.update_cursor_state(self.cursor.get_state());
                } else if self.cursor.cx == 0 && self.cursor.cy > 0 {
                    let old_adj_len = self
                        .operations
                        .get_length_at_line((self.cursor.cy - 1) as usize);
                    self.operations
                        .process_wrap_delete(self.cursor.get_state(), d);

                    // update cursor to jump to previous line, set cx to end of previous line's original contents
                    let data = self.operations.get_text();
                    self.cursor.handle_cursor(false, self.cursor.cy - 1, data);
                    self.cursor
                        .handle_cursor(true, old_adj_len.try_into().unwrap(), data);
                    self.operations.update_cursor_state(self.cursor.get_state())
                }
            }
            (Direction::Right, WriteMode::Editor) => {
                // currently, the controller isn't wired to conditionally call process_wrap_delete directly
                // use of this process_wrap_delete relies on an understanding of the current line length,
                // and the operations handler is probably better suited to process this for now.
                self.operations.process_delete(self.cursor.get_state(), d);
            }
            (Direction::Left, WriteMode::Prompt) => self.operations.process_prompt_delete(true),
            (Direction::Right, WriteMode::Prompt) => self.operations.process_prompt_delete(false),
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

    // Tells the OperationHandler to write current terminal contents to a file with a known name.
    // If the file is untitled, the controller should start rerouting writes to PromptProc. The OH will render a save-as prompt.
    pub fn write_file(&mut self) {
        let save_as = self.operations.check_file_name();
        if !save_as {
            self.operations.write_file(&self.file_name);
        } else {
            self.mode = WriteMode::Prompt;
        }
    }

    // Called whenever a prompt is dismissed or exited. Set back WriteMode, clear any statuses, wipe the PromptProc.
    pub fn exit_prompt(&mut self) {
        self.mode = WriteMode::Editor;
        self.operations.wipe_prompt();
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
