use std::io::{BufWriter, Stdout, Write, Error, stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use crate::{data::{textrow::TextRow, payload::CursorState}, utils};

pub struct RenderDriver {
    rows: u16,
    cols: u16,
    buf: BufWriter<RawTerminal<Stdout>>,
    text: Vec<TextRow>,
    cursor: CursorState,
}

impl RenderDriver {
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

    pub(in crate::gfx) fn update_cursor_state(&mut self, state: CursorState) {
        self.cursor = state;
    }

    pub(in crate::gfx) fn get_text(&mut self) -> &Vec<TextRow> {
        &self.text
    }

    pub(in crate::gfx) fn set_text(&mut self, text: Vec<TextRow>) {
        self.text = text;
    }

    pub(in crate::gfx) fn exit(&mut self) {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    pub(in crate::gfx) fn tick_screen(&mut self) -> Result<(), Error> {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::cursor::Hide).expect(WRITE_ERR_MSG);
        self.set_screen();
        write!(self.buf, "{}{}", termion::cursor::Goto((self.cursor.cx + 1).try_into().unwrap(), (self.cursor.cy + 1).try_into().unwrap()), termion::cursor::Show).expect(WRITE_ERR_MSG); 

        self.buf.flush()
    }
}

const WRITE_ERR_MSG: &'static str = "Failed to write to console.";
const VERSION: &'static str = "0.1";