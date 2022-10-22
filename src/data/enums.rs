// Different forms of user-inputted inputs. 
// Currently supported:
// Quit (Ctrl+Q) - Close the editor
// Move (Arrow keys) - Move the cursor in the editor
// Page (Home/End/PgUp/PgDn) - Snap cursor to the left/right/top/bottom of the editor
// 
pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
    Write(char),
}

// Directions. Used to classify InputEvents.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}