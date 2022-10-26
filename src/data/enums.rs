// Different forms of user-inputted inputs. 
// Currently supported:
// Quit (Ctrl+Q) - Close the editor
// Save (Ctrl+S) - Save the current file
// Move (Arrow keys) - Move the cursor in the editor
// Page (Home/End/PgUp/PgDn) - Snap cursor to the left/right/top/bottom of the editor
// Write - Input a character into a line of text
pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
    Write(char),
    Save
}

// Directions. Used to classify InputEvents.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}