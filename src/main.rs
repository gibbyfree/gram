mod gfx;
mod gram;
mod input;
use crate::gram::Gram;

fn main() {
    let mut editor = Gram::new();
    editor.tick();
}
