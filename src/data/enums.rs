// Different forms of user-inputted inputs. 
// Currently supported:
// Quit (Ctrl+Q) - Close the editor
// Save (Ctrl+S) - Save the current file
// Move (Arrow keys) - Move the cursor in the editor
// Page (Home/End/PgUp/PgDn) - Snap cursor to the left/right/top/bottom of the editor
// Write - Input a character into a line of text
// Delete (Backspace / Del / Ctrl+H) - Delete a character in the line of text. Delete left or right of the cursor.
pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
    Write(char),
    Delete(Direction),
    Save
}

// Directions. Used to classify InputEvents.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Status Content. Used to classify the Content of a StatusMessage.
// Help - Keybind help message shown on launch
// SaveSuccess - Shown on file write success
// DirtyWarning - Shown when closing a modified, unsaved file. i16 for # of force quit inputs.
pub enum StatusContent {
    Help,
    SaveSuccess,
    DirtyWarning(i16),
}