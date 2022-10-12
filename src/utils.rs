use termsize::Size;

// Returns current window size of a terminal, in rows and columns.
// This is the best terminal size lib I've found so far -- termion was reporting weird numbers in my WSL setup.
pub fn get_window_size() -> Size {
    termsize::get().unwrap()
}