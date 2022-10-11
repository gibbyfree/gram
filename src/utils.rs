use termsize::Size;

pub fn get_window_size() -> Size {
    termsize::get().unwrap()
}