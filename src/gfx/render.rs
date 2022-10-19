use std::io::{BufWriter, Stdout, Write, Error, stdout};
use termion::{raw::{IntoRawMode, RawTerminal}, color};
use crate::{data::{textrow::TextRow, payload::CursorState}, utils};

// RenderDriver. Primarily responsible for everything we draw to the editor window.
// Contains an understanding of editor window size, based upon window size at program initialization.
// Contains a BufWriter for writing efficiently to stdout in Raw Mode.
// Might contain a vector of TextRows, and holds a reference to the current state of the CursorHandler.
pub struct RenderDriver {
    rows: u16,
    cols: u16,
    buf: BufWriter<RawTerminal<Stdout>>,
    text: Vec<TextRow>,
    cursor: CursorState,
    file_name: String,
    status_info: String,
}

impl RenderDriver {
    // A RenderDriver is created with a fresh CursorState.
    // Initially, text is set to an empty vector of textrows. This is replaced with text if the program is run with a file path.
    // Filename and statusinfo are also set to empty values. These are replaced if the program is run with a file path.
    pub fn new(cursor: CursorState) -> Self {
        let size_rc = utils::get_window_size();
        Self {
            rows: size_rc.rows,
            cols: size_rc.cols,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
            text: vec![TextRow::default()],
            cursor,
            file_name: "".to_string(),
            status_info: "".to_string(),
        }
    }

    // Draws the editor's footer, centered within the editor window.
    // Deprecated method? Maybe can be reused for something else.
    fn draw_footer(&mut self) {
        let welcome = format!("Gram editor -- v{}\r", VERSION);
        let mut padding = (self.cols - welcome.len() as u16) / 2;
        write!(self.buf, "~").expect(WRITE_ERR_MSG);
        while padding > 0 {
            write!(self.buf, " ").expect(WRITE_ERR_MSG);
            padding = padding - 1;
        }
        write!(self.buf, "{}", welcome).expect(WRITE_ERR_MSG);
    }

    // Draw the editor's status bar, which spans the bottom-most line of the editor.
    // Contains the filename, # of lines in the file, and the current line.
    fn draw_status(&mut self) {
        write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
        write!(self.buf, "{}{}{}", color::Bg(color::White), color::Fg(color::Black), self.status_info).unwrap();

        // only exclude length of text written -- termion:color borks str len
        for _n in 0..self.cols.wrapping_sub(self.status_info.len().try_into().unwrap()).wrapping_sub((self.cursor.line_num().len()).try_into().unwrap()) {
            write!(self.buf, "{}", " ").unwrap();
        }

        write!(self.buf, "{}\r{}{}", self.cursor.line_num(), color::Bg(color::Black), color::Fg(color::White)).unwrap();
    }

    // Sets the screen for this current tick.
    // Iterates through all rows of the window, clearing its old contents and replacing with either rendered text or a blank line.
    // Uses row and col offset to determine which textrows are rendered, and how.
    // Renders the footer as the last line.
    fn set_screen(&mut self) {
        for n in 0..self.rows - 1 {
            // clear the current line
            write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
            let row_idx = n.wrapping_add(self.cursor.row_offset as u16);
            // render text if necessary, else render edge
            if row_idx < self.text.len() as u16 {
                let render_str = self.text[row_idx as usize].substring(self.cursor.col_offset);
                writeln!(self.buf, "{}\r", render_str.truncate(self.cols)).expect(WRITE_ERR_MSG);
            } else {
                writeln!(self.buf, "~\r").expect(WRITE_ERR_MSG);
            }
        }
        self.draw_status();
    }

    // Sets the static status info of this file -- file name and # of lines in the file.
    fn set_status_info(&mut self) {
        let file: String;
        if self.file_name.eq("") {
            file = "[Untitled]".to_string();
        } else if self.file_name.len() > 20 {
            file = self.file_name[..20].to_string();
        } else {
            file = format!("{}", self.file_name); // i hate this
        }
        let lines = (self.text.len() + 1).to_string() + " lines";
        self.status_info = format!("{} - {}", file, lines);
    }

    // Updates this RenderDriver's current CursorState.
    pub(in crate::gfx) fn update_cursor_state(&mut self, state: CursorState) {
        self.cursor = state;
    }

    // Returns a reference to this RenderDriver's current text data.
    pub(in crate::gfx) fn get_text(&mut self) -> &Vec<TextRow> {
        &self.text
    }

    // Sets the text data of the RenderDriver.
    // At this point, the renderer should have both the file's text and its name. Set status info according to these values.
    pub(in crate::gfx) fn set_text(&mut self, text: Vec<TextRow>) {
        self.text = text;
        self.set_status_info();
    }

    // Saves the file name of the opened file.
    // Could potentially be refactored out, but waiting to see if this is useful to keep.
    pub(in crate::gfx) fn set_file_name(&mut self, name: &str) {
        self.file_name = name.to_string();
    }

    // Exits the editor, clearing the entire window and resetting the cursor position.
    pub(in crate::gfx) fn exit(&mut self) {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    // Ticks the screen by moving the cursor out of the way and hiding it, then drawing, then replacing the cursor and unhiding.
    pub(in crate::gfx) fn tick_screen(&mut self) -> Result<(), Error> {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::cursor::Hide).expect(WRITE_ERR_MSG);
        self.set_screen();
        write!(self.buf, "{}{}", termion::cursor::Goto((self.cursor.cx + 1).try_into().unwrap(), (self.cursor.cy + 1).try_into().unwrap()), termion::cursor::Show).expect(WRITE_ERR_MSG); 

        self.buf.flush()
    }
}

const WRITE_ERR_MSG: &'static str = "Failed to write to console.";
const VERSION: &'static str = "0.1";