mod gfx;
mod gram;
mod input;
use std::io::stdout;
use crate::gram::Gram;
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    let editor = Gram::new();
    editor.tick();
}
