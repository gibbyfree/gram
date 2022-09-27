use std::io::{BufWriter, Stdout, Write, Error, stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use termsize::Size;
use crate::data::TextRow;

pub struct RenderDriver {
    pub(in crate::gfx) rows: u16,
    pub(in crate::gfx) cols: u16,
    pub(in crate::gfx) cx: u16,
    pub(in crate::gfx) cy: u16,
    buf: BufWriter<RawTerminal<Stdout>>,
    text: Vec<TextRow>,
}

impl RenderDriver {
    pub fn new() -> Self {
        let size_rc = get_window_size();
        let mut s = Self {
            rows: size_rc.rows,
            cols: size_rc.cols,
            cx: 0,
            cy: 0,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
            text: vec![TextRow::default()],
        };
        s.tick_screen().unwrap();
        s
    }

    fn draw_footer(&mut self) {
        let welcome = format!("Gram editor -- v{}\r\n", VERSION);
        let mut padding = (self.cols - welcome.len() as u16) / 2;
        write!(self.buf, "~").expect(WRITE_ERR_MSG);
        while padding > 0 {
            write!(self.buf, " ").expect(WRITE_ERR_MSG);
            padding = padding - 1;
        }
        write!(self.buf, "{}", welcome).expect(WRITE_ERR_MSG);
    }

    fn set_screen(&mut self) {
        for n in 0..self.rows - 2 {
            // clear the current line
            write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
            // render text if necessary, else render border
            if n < self.text.len() as u16 {
                write!(self.buf, "{}\r\n", self.text[0]).expect(WRITE_ERR_MSG);
            } else {
                write!(self.buf, "~\r\n",).expect(WRITE_ERR_MSG);
            }
        }
        self.draw_footer();
    }

    pub(in crate::gfx) fn set_text(&mut self, text: Vec<TextRow>) {
        self.text = text;
    }

    pub(in crate::gfx) fn set_cursor(&mut self, x: bool, val: u16) {
        if x {
            self.cx = val;
        } else {
            self.cy = val;
        }
    }

    pub(in crate::gfx) fn exit(&mut self) {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    pub(in crate::gfx) fn tick_screen(&mut self) -> Result<(), Error> {
        write!(self.buf, "{}", termion::cursor::Hide).expect(WRITE_ERR_MSG);
        self.set_screen();
        write!(self.buf, "{}{}", termion::cursor::Show, termion::cursor::Goto(self.cx + 1, self.cy + 1)).expect(WRITE_ERR_MSG); 

        self.buf.flush()
    }
}

fn get_window_size() -> Size {
    termsize::get().unwrap()
}

const WRITE_ERR_MSG: &'static str = "Failed to write to console.";
const VERSION: &'static str = "0.1";