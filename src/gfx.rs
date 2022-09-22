use std::io::{self, Write};

#[derive(Debug)]
pub struct GfxDriver {
    rows: u16,
    cols: u16,
}

impl GfxDriver {
    pub fn new() -> Self {
        let size_rc = get_window_size();
        Self {
            rows: size_rc.0,
            cols: size_rc.1,
        }
    }

    fn draw_lhs_edge(&self) {
        for _n in 1..self.rows + 1 {
            println!("~\r")
        }
    }

    pub fn tick_screen(&self) -> u8 {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.draw_lhs_edge();
        print!("{}", termion::cursor::Goto(1, 1));
        let res = io::stdout().flush();
        match res {
            Ok(_) => return 1,
            Err(_) => return 0,
        };
    }
}

fn get_window_size() -> (u16, u16) {
    termion::terminal_size().unwrap()
}
