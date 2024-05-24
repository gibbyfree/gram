// Different forms of user-inputted inputs.
// Currently supported:
// Quit (Ctrl+Q) - Close the editor
// Save (Ctrl+S) - Save the current file
// Move (Arrow keys) - Move the cursor in the editor
// Page (Home/End/PgUp/PgDn) - Snap cursor to the left/right/top/bottom of the editor
// Write - Input a character into a line of text
// Delete (Backspace / Del / Ctrl+H) - Delete a character in the line of text. Delete left or right of the cursor.
// Cancel - Used for exiting any prompt interactions.
// Find - Used to initialize a 'find' prompt interaction.
pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
    Write(char),
    Delete(Direction),
    Save,
    Cancel,
    Find,
}

// Directions. Used to classify InputEvents.
#[derive(Clone, Copy)]
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
// SaveAs - Shown when closing an unnamed file. String for user inputted file name.
// SaveAbort - Shown when SaveAs is aborted.
// Find - Shown when CTRL+F is used to search the opened file. String for user inputted query.
// FindAbort - Shown when Find is aborted.
#[derive(PartialEq)]
pub enum StatusContent {
    Help,
    SaveSuccess,
    DirtyWarning(i16),
    SaveAs(String),
    SaveAbort,
    Find(String),
    FindAbort,
}

// Write Mode. Specifies different areas that we might process writes to.
// Prompt - Processing writes to the prompt (i.e. Save-as functionality)
// Editor - Processing writes to the main editor.
pub enum WriteMode {
    Prompt,
    Editor,
}

// Prompt Result. Contains some kind of data for the OH to pass to the controller.
// FileRename - Sent after a successful file rename. Contains the new file name.
// TextSearch - Incremental. Sent on each query input. Contains the query.
pub enum PromptResult {
    FileRename(String),
    TextSearch(String),
}
