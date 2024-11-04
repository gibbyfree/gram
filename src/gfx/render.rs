use crate::{
    data::{
        enums::StatusContent,
        payload::{CursorState, DirtyStatus, StatusMessage},
        textrow::TextRow,
    },
    utils,
};
use std::io::{stdout, BufWriter, Error, Stdout, Write};
use termion::{
    color::{self, Rgb},
    raw::{IntoRawMode, RawTerminal},
};

// RenderDriver. Primarily responsible for everything we draw to the editor window.
// Contains an understanding of editor window size, based upon window size at program initialization.
// Contains a BufWriter for writing efficiently to stdout in Raw Mode.
// Might contain a vector of TextRows, and holds a reference to the current state of the CursorHandler.
pub struct RenderDriver {
    rows: u16,
    cols: u16,
    buf: BufWriter<RawTerminal<Stdout>>,
    text: Vec<TextRow>,
    cursor: CursorState,
    file_name: String,
    status_info: String,
    status_message: StatusMessage,
    mod_status: DirtyStatus,
    status_kind: StatusContent,
}

impl RenderDriver {
    // A RenderDriver is created with a fresh CursorState.
    // Initially, text is set to an empty vector of textrows. This is replaced with text if the program is run with a file path.
    // Filename and statusinfo are also set to empty values. These are replaced if the program is run with a file path.
    pub fn new(cursor: CursorState) -> Self {
        let size_rc = utils::get_window_size();
        Self {
            rows: size_rc.rows,
            cols: size_rc.cols,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
            text: vec![TextRow::default()],
            cursor,
            file_name: "".to_string(),
            status_info: "".to_string(),
            status_message: StatusMessage::new(false),
            mod_status: DirtyStatus::new(),
            // arbitrary default
            status_kind: StatusContent::Help,
        }
    }

    // Draw the editor's status bar, which spans the bottom-most line of the editor.
    // Contains the filename, # of lines in the file, and the current line.
    fn draw_status_bar(&mut self) {
        write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
        write!(
            self.buf,
            "{}{}{}",
            color::Bg(color::White),
            color::Fg(color::Black),
            self.status_info
        )
        .unwrap();

        // only exclude length of text written -- termion:color borks str len
        for _n in 0..self
            .cols
            .wrapping_sub(self.status_info.len().try_into().unwrap())
            .wrapping_sub((self.cursor.line_num().len()).try_into().unwrap())
        {
            write!(self.buf, " ").unwrap();
        }

        write!(self.buf, "{}\r", self.cursor.line_num()).unwrap();
    }

    // Draws the status message, which appears below the status bar.
    // Only contains messages to the user for now.
    fn draw_status_message(&mut self) {
        writeln!(self.buf).expect(WRITE_ERR_MSG);
        write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
        write!(
            self.buf,
            "{}{}{}\r",
            color::Bg(color::White),
            color::Fg(color::Black),
            self.status_message
        )
        .unwrap();
    }

    // Reset color used in terminal printing. Necessary after drawing status components.
    fn reset_color(&mut self) {
        write!(
            self.buf,
            "{}{}",
            color::Bg(color::Black),
            color::Fg(color::White)
        )
        .unwrap();
    }

    // Sets the screen for this current tick.
    // Iterates through all rows of the window, clearing its old contents and replacing with either rendered text or a blank line.
    // Uses row and col offset to determine which textrows are rendered, and how - calls into process_tokens to render and color text.
    // Renders the status bar as the last line, unless there is a status message to print -- in which case we print both. Resets color afterwards.
    fn set_screen(&mut self) {
        let render_message = self.status_message.should_print();
        let end: u16 = if render_message {
            self.rows - 2
        } else {
            self.rows - 1
        };

        // black
        let other_fg = color::Fg(Rgb(255, 255, 255));
        let mut multiline_comment = false;

        for n in 0..end {
            // clear the current line
            write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
            let row_idx = n.wrapping_add(self.cursor.row_offset as u16);
            // render text if necessary, else render edge (or blank space for the final line)
            if row_idx < self.text.len() as u16 {
                let render_str = self.text[row_idx as usize]
                    .substring(self.cursor.col_offset)
                    .truncate(self.cols);
                let tokens: Vec<String> = tokenize_preserve_whitespace(&render_str.raw_text);

                // If we're in a find state, we need to highlight the search query.
                let q = if let StatusContent::Find(q) = &self.status_kind {
                    q
                } else {
                    ""
                };
                multiline_comment =
                    process_tokens(&mut self.buf, tokens, q, &self.file_name, multiline_comment);
                writeln!(self.buf, "\r{}", other_fg).expect(WRITE_ERR_MSG);
            } else {
                writeln!(self.buf, "~\r{}", other_fg).expect(WRITE_ERR_MSG);
            }
        }
        self.draw_status_bar();
        if render_message {
            self.draw_status_message();
        }
        self.reset_color();
    }

    // Sets the static status info of this file -- file name and # of lines in the file.
    fn set_status_info(&mut self) {
        let mut file: String;
        if self.file_name.eq("") {
            file = "[Untitled]".to_string();
        } else if self.file_name.len() > 20 {
            file = self.file_name[..20].to_string();
        } else {
            file = self.file_name.to_string(); // i hate this
        }
        if self.mod_status.dirty {
            file += " (modified)";
        }

        let lines = self.text.len().to_string() + " lines";
        self.status_info = format!("{} - {}", file, lines);
    }

    // PUBLIC METHODS //
    // Final set-up method for the renderer. Sets status info and status message.
    pub fn complete_init(&mut self) {
        self.set_status_info();
        self.update_status_message(StatusContent::Help);
    }

    // Updates the status message of the editor based on a given StatusContent.
    // Each StatusContent type sets a content messge, and refreshes the status info bar.
    pub fn update_status_message(&mut self, t: StatusContent) {
        self.status_message.clean();
        match t {
            StatusContent::SaveSuccess => {
                self.mod_status.clean();
                self.status_message
                    .set_content(SAVE_SUCCESS_MSG.to_string());
                self.set_status_info();
            }
            StatusContent::DirtyWarning(q) => {
                let msg = format!(
                    "Warning! File has unsaved changes. Press Ctrl+Q {} more times to quit.",
                    3 - q
                );
                self.status_message.set_content(msg);
                self.set_status_info();
            }
            StatusContent::Help => self
                .status_message
                .set_content(KEYBIND_HELP_MSG.to_string()),
            StatusContent::SaveAs(f) => {
                self.status_message.live_forever_for_now();
                let msg = format!("Save as: {} (Use ESC to cancel)", f);
                self.status_message.set_content(msg);
            }
            StatusContent::Find(q) => {
                self.status_message.live_forever_for_now();
                self.status_kind = StatusContent::Find(q.clone());
                let msg = format!("Search: {} (Use ESC to cancel)", q);
                self.status_message.set_content(msg);
            }
            StatusContent::SaveAbort => self.status_message.set_content(SAVE_ABORT_MSG.to_string()),
            StatusContent::PromptAbort => {
                self.status_message.immortal = false;
                self.status_kind = StatusContent::Help;
            }
        }
    }

    // Updates this RenderDriver's current CursorState.
    pub fn update_cursor_state(&mut self, state: CursorState) {
        self.cursor = state;
        self.mod_status.reset();
    }

    // Returns a reference to this RenderDriver's current text data.
    pub fn get_text(&mut self) -> &mut Vec<TextRow> {
        &mut self.text
    }

    // Sets the text data of the RenderDriver.
    // At this point, the renderer should have everything that it needs to complete its initialization.
    pub fn set_text(&mut self, text: Vec<TextRow>) {
        self.text = text;
        self.complete_init();
    }

    pub fn delete_row(&mut self, idx: usize) {
        self.text.remove(idx);
    }

    pub fn insert_row(&mut self, idx: usize, text: TextRow) {
        self.text.insert(idx, text);
    }

    // Update the text contained at a given row index.
    // If this is a modification, insert the new text at the row.
    // If this is an insert, add whitespace lines as needed and then push the new text to the end.
    // Additionally, an insert might require us to update status info to include the new document length.
    pub fn set_text_at_index(&mut self, idx: usize, row: String) {
        if idx < self.text.len() {
            self.text[idx].update_text(row);
        } else {
            let dif = idx - self.text.len();
            for _i in 0..dif {
                self.text.push(TextRow::new("".to_string()));
            }
            self.text.push(TextRow::new(row));
        }
        self.mod_status.redirty();
        self.set_status_info();
    }

    // Whether or not the user is currently inputting force quits.
    pub fn is_quitting(&mut self) -> bool {
        self.mod_status.quit_count > 0
    }

    // Saves the file name of the opened file.
    // Could potentially be refactored out, but waiting to see if this is useful to keep.
    pub fn set_file_name(&mut self, name: &str) {
        self.file_name = name.to_string();
    }

    // Exits the editor, clearing the entire window and resetting the cursor position.
    // If the editor is currently dirty, and the user has not force quit enough times, render a warning and do nothing.
    // Confirm shutdown only with sufficient force quits, or with a clean editor.
    pub fn exit(&mut self) -> bool {
        if self.mod_status.dirty && self.mod_status.quit_count < 3 {
            self.update_status_message(StatusContent::DirtyWarning(self.mod_status.quit_count));
            self.mod_status.quit_count += 1;
            false
        } else {
            write!(
                self.buf,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
            .expect(WRITE_ERR_MSG);
            self.buf.flush().unwrap();
            true
        }
    }

    // Ticks the screen by moving the cursor out of the way and hiding it, then drawing, then replacing the cursor and unhiding.
    pub fn tick_screen(&mut self) -> Result<(), Error> {
        write!(
            self.buf,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .expect(WRITE_ERR_MSG);
        self.set_screen();
        write!(
            self.buf,
            "{}{}",
            termion::cursor::Goto(
                (self.cursor.cx + 1).try_into().unwrap(),
                (self.cursor.cy + 1).try_into().unwrap()
            ),
            termion::cursor::Show
        )
        .expect(WRITE_ERR_MSG);

        self.buf.flush()
    }
    // END OF PUBLIC METHODS //
}

// SYNTAX HIGHLIGHTING //

// Determines the correct color for a token given its content.
fn determine_color(token: &str) -> color::Fg<color::Rgb> {
    if token.parse::<f64>().is_ok() {
        // digits are red
        color::Fg(Rgb(255, 0, 0))
    } else if C_KEYWORDS.contains(&token) {
        // keywords are yellow
        color::Fg(Rgb(255, 255, 0))
    } else if C_TYPES.contains(&token) {
        // types are green
        color::Fg(Rgb(0, 255, 0))
    } else {
        // rest is white
        color::Fg(Rgb(255, 255, 255))
    }
}

// Write a token using the correct color.
// If a color is provided, use that color. Otherwise, determine the color based on the token.
// If the token is in a string, color it magenta. If the token is in a comment, color it green.
fn write_token(
    buf: &mut BufWriter<RawTerminal<Stdout>>,
    token: &str,
    fg: Option<color::Fg<color::Rgb>>,
    in_string: bool,
    in_comment: bool,
) {
    let mut color = match fg {
        Some(f) => f,
        None => determine_color(token),
    };

    if in_string {
        color = color::Fg(Rgb(255, 0, 255));
    } else if in_comment {
        color = color::Fg(Rgb(0, 255, 0));
    }

    write!(buf, "{}{}", color, token).expect(WRITE_ERR_MSG);
}

// Tokenizes a textrow, preserving whitespace as separate tokens.
fn tokenize_preserve_whitespace(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_whitespace = s.chars().next().map_or(false, |c| c.is_whitespace());

    for c in s.chars() {
        if c.is_whitespace() {
            if !in_whitespace {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            in_whitespace = true;
        } else {
            if in_whitespace {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            in_whitespace = false;
        }
        current_token.push(c);
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

// Processes a list of tokens, determining the correct color for each token and writing it to the buffer.
fn process_tokens(
    buf: &mut BufWriter<RawTerminal<Stdout>>,
    tokens: Vec<String>,
    q: &str,
    file_name: &str,
    multiline_comment: bool,
) -> bool {
    // blue
    let find_fg = color::Fg(Rgb(0, 0, 255));
    // in_string and in_comment are used to track whether we've been processing a string or a comment.
    // (Both strings and comments can span multiple tokens.)
    let mut in_string = false;
    let mut in_comment = multiline_comment;

    // Always highlight find results, but only highlight syntax in C files.
    let should_highlight =
        file_name.ends_with(".c") || file_name.ends_with(".h") || file_name.ends_with(".cpp");

    // If the first token starts with '//', the line is a comment and should be colored cyan.
    if !tokens.is_empty() && tokens[0].starts_with("//") {
        let comment = tokens.join("");
        write!(buf, "{}", color::Fg(Rgb(0, 255, 255))).expect(WRITE_ERR_MSG);
        write!(buf, "{} ", comment).expect(WRITE_ERR_MSG);
        return in_comment;
    }

    // If the first token starts with '/*', assume a multi-line comment has begun.
    if !tokens.is_empty() && tokens[0].starts_with("/*") {
        in_comment = true;
    }

    for token in &tokens {
        let magenta_start = token.find('"');
        let magenta_end = token.rfind('"');

        // Blue highlighting for find query should override all other highlighting.
        if !q.is_empty() && token.contains(q) {
            let blue_start = token.find(q).unwrap();
            let blue_end = blue_start + q.len();
            let blue_token = &token[blue_start..blue_end];
            let before_token = &token[..blue_start];
            let after_token = &token[blue_end..];

            let before_fg = determine_color(before_token);
            let after_fg = determine_color(after_token);

            write!(buf, "{}{}", before_fg, before_token).expect(WRITE_ERR_MSG);
            write!(buf, "{}{}", find_fg, blue_token).expect(WRITE_ERR_MSG);
            write!(buf, "{}{}", after_fg, after_token).expect(WRITE_ERR_MSG);
        } else if should_highlight && magenta_start.is_some() && magenta_end.is_some() {
            if let (Some(start), Some(end)) = (magenta_start, magenta_end) {
                // A quote appears in this token.
                if start == end {
                    // Single quote in the token.
                    if in_string {
                        // Ending quote.
                        let magenta_token = &token[..end + 1];
                        let after_token = &token[end + 1..];

                        write!(buf, "{}{}", color::Fg(Rgb(255, 0, 255)), magenta_token)
                            .expect(WRITE_ERR_MSG);
                        write!(buf, "{}{}", determine_color(after_token), after_token)
                            .expect(WRITE_ERR_MSG);
                        in_string = false;
                    } else {
                        // Starting quote.
                        let before_token = &token[..start];
                        let magenta_token = &token[start..];

                        write!(buf, "{}{}", determine_color(before_token), before_token)
                            .expect(WRITE_ERR_MSG);
                        write!(buf, "{}{}", color::Fg(Rgb(255, 0, 255)), magenta_token)
                            .expect(WRITE_ERR_MSG);
                        in_string = true;
                    }
                } else {
                    // Two quotes in the token.
                    let before_token = &token[..start];
                    let magenta_token = &token[start..end + 1];
                    let after_token = &token[end + 1..];

                    write!(buf, "{}{}", determine_color(before_token), before_token)
                        .expect(WRITE_ERR_MSG);
                    write!(buf, "{}{}", color::Fg(Rgb(255, 0, 255)), magenta_token)
                        .expect(WRITE_ERR_MSG);
                    write!(buf, "{}{}", determine_color(after_token), after_token)
                        .expect(WRITE_ERR_MSG);
                }
            }
        } else {
            // Syntax highlighting is only enabled for C files.
            let fg = if should_highlight
            {
                // Determine fg via determine_color later
                None
            } else {
                // white
                Some(color::Fg(Rgb(255, 255, 255)))
            };

            write_token(buf, token, fg, in_string, in_comment);
        }

        // If the token contains '*/', assume a multi-line comment has ended.
        if token.contains("*/") {
            in_comment = false;
        }
    }

    // Return whether we're rendering a multi-line comment, so the next line can continue it if necessary.
    in_comment
}

// CONSTS //

// Const strings for error messages and help messages.
const WRITE_ERR_MSG: &str = "Failed to write to console.";
const KEYBIND_HELP_MSG: &str = "HELP: Ctrl+Q - exit | Ctrl+S - save | Ctrl+F - find";
const SAVE_SUCCESS_MSG: &str = "Wrote file to disk.";
const SAVE_ABORT_MSG: &str = "Save aborted.";

// Const lists for syntax highlighting.
const C_KEYWORDS: &[&str] = &[
    "switch", "if", "while", "for", "break", "continue", "return", "else", "struct", "union",
    "typedef", "static", "enum", "class", "case",
];
const C_TYPES: &[&str] = &[
    "int", "long", "double", "float", "char", "unsigned", "signed", "void", "#include",
];
