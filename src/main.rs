mod gfx;
mod gram;
mod input;
mod data;
use crate::gram::Gram;

fn main() {
    let mut editor = Gram::new();
    editor.tick();
}
