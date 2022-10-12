use std::io::{BufWriter, Stdout, Write, Error, stdout};
use termion::raw::{IntoRawMode, RawTerminal};
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
}

impl RenderDriver {
    // A RenderDriver is created with a fresh CursorState.
    // Initially, text is set to an empty vector of textrows. This is replaced with text if the program is run with a file path.
    pub fn new(cursor: CursorState) -> Self {
        let size_rc = utils::get_window_size();
        Self {
            rows: size_rc.rows,
            cols: size_rc.cols,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
            text: vec![TextRow::default()],
            cursor,
        }
    }

    // Draws the editor's footer, centered within the editor window.
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
        self.draw_footer();
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
    pub(in crate::gfx) fn set_text(&mut self, text: Vec<TextRow>) {
        self.text = text;
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