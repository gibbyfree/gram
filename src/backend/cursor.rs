use crate::data::{payload::CursorState, textrow::TextRow};

// CursorHandler. Deals with all of the messy logic around scroller and cursor movement.
// cx and cy represent the x,y coords of the cursor's location.
// row_offset and col_offset represent the degree to which the cursor is moved 'off-screen' on either axis.
// Also stores the size of the terminal window (upon program initialization -- doesn't mutate) and its current state.
// saved_state is used for saving and restoring the cursor's state (for prompt cancellation).
pub struct CursorHandler {
    pub cx: i16,
    pub cy: i16,
    row_offset: i16,
    col_offset: i16,
    rows: u16,
    cols: u16,
    state: CursorState,
    saved_state: CursorState,
}

impl CursorHandler {
    // CursorHandlers are initialized in a zeroed out state, aside from the passed in terminal window size.
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            cx: 0,
            cy: 0,
            row_offset: 0,
            col_offset: 0,
            rows,
            cols,
            state: CursorState::new(),
            saved_state: CursorState::new(),
        }
    }

    // Returns the CursorHandler's current state.
    pub fn get_state(&mut self) -> CursorState {
        self.state
    }

    // Update this CursorHandler's current state, using the CursorHandler's current relevant values.
    fn update_state(&mut self) {
        self.state = self
            .state
            .update(self.cx, self.cy, self.row_offset, self.col_offset);
    }

    // Backup the current cursor state. Used on prompt initialization.
    pub fn save_state(&mut self) {
        self.saved_state = self.state;
    }

    // Restore the current cursor state from the saved state. Used on prompt cancellation.
    pub fn restore_state(&mut self) {
        self.cx = self.saved_state.cx;
        self.cy = self.saved_state.cy;
        self.row_offset = self.saved_state.row_offset;
        self.col_offset = self.saved_state.col_offset;
        self.update_state();
    }

    // Entry point for handling a cursor event.
    // x: whether this is a move on the x-axis or not (if not, then y-axis)
    // val: proposed new value for cx/cy
    // data: reference to the RenderDriver's current state of text
    pub fn handle_cursor(&mut self, x: bool, val: i16, data: &[TextRow]) {
        if x {
            self.handle_x_move(val, data);
        } else {
            self.handle_y_move(val, data);
        }
    }

    // More stripped-down version of handle_cursor, used to handle scroll events.
    // Necessary because the controller is unaware of data size / current lines. unlike CH.
    // Updates its CursorState after all values have been changed.
    pub fn handle_scroll(&mut self, x: bool, start: bool, data: &[TextRow]) {
        match (x, start) {
            (true, true) => {
                self.cx = 0;
                self.col_offset = 0;
            }
            (true, false) => self.wrap_cx_to_end(data),
            (false, true) => {
                self.cy = 0;
                self.row_offset = 0;
            }
            (false, false) => self.wrap_cy_to_end(data),
        }

        self.update_state();
    }

    // Handle a cursor move along the y-axis, with a proposed cy value and a reference to the RenderDriver's current data.
    // Will also correct cx if we skip from a long line to a shorter one.
    // Updates its CursorState after all values have been changed.
    fn handle_y_move(&mut self, val: i16, data: &[TextRow]) {
        if val == -1 {
            // moving offscreen to the top
            if self.row_offset > 0 {
                // any more rows to render?
                self.row_offset -= 1;
            }
        }
        if val == (self.rows - 1).try_into().unwrap() {
            // moving offscreen to the bottom
            if (self.row_offset + self.cy + 1) <= data.len().try_into().unwrap() {
                // more data to render?
                self.row_offset += 1;
            }
        }
        if val != -1
            && val != (self.rows - 1).try_into().unwrap()
            && val <= data.len().try_into().unwrap()
        {
            // moving within the document
            self.cy = val;
        }

        self.check_and_fix_cx(data);
        self.update_state();
    }

    // Handle a cursor move along the x-axis, with a proposed cx value and a reference to the RenderDriver's current data.
    // Updates its CursorState after all values have been changed.
    fn handle_x_move(&mut self, val: i16, data: &[TextRow]) {
        let mut has_wrapped = false;
        if val == -1 {
            // moving offscreen to the left
            if self.col_offset > 0 {
                // is there more data to show here?
                self.col_offset -= 1;
            } else if self.cy >= 1 {
                // is there a line we can wrap to?
                self.cy -= 1;
                self.col_offset = 0;
                self.wrap_cx_to_end(data);
                has_wrapped = true;
            }
        }
        if val == (self.cols - 1).try_into().unwrap() {
            // offscreen to the right
            if data[self.cy as usize].length() >= val + self.col_offset {
                // is there more data to show here?
                self.col_offset += 1;
            } else if data.len() + 1 > self.cy.try_into().unwrap() {
                // is there a line we can wrap to?
                self.cy += 1;
                self.cx = 0;
                self.col_offset = 0;
                has_wrapped = true;
            }
        }
        if val == data[self.cy as usize].length() + 1
            || (val + self.col_offset >= data[self.cy as usize].length() + 1)
        {
            // end of line
            if data.len() + 1 > self.cy.try_into().unwrap() {
                // is there a line we can wrap to?
                self.cy += 1;
                self.cx = 0;
                self.col_offset = 0;
                has_wrapped = true;
            }
        }
        if val > self.cols.try_into().unwrap() && val <= data[self.cy as usize].length() + 1 {
            // "teleport case" -- impossible to receive this val otherwise
            self.col_offset = val - (self.cols - 5) as i16;
            self.cx = val - self.col_offset;
            has_wrapped = true;
        }
        if val != -1 && val < (self.cols - 1).try_into().unwrap() && !has_wrapped {
            // not going offscreen
            self.cx = val;
        }

        self.update_state();
    }

    // Helper method for wrapping cx to the end of the line indexed by the handler's current cy.
    // Sets cx to the very end of this line, adding col_offset if the line is long enough.
    // Doesn't update its CursorState -- this should be done by the calling function.
    fn wrap_cx_to_end(&mut self, data: &[TextRow]) {
        let line_len = data[self.cy as usize].length();
        if line_len > self.rows.try_into().unwrap() {
            self.col_offset = line_len.wrapping_sub(self.rows.try_into().unwrap());
            self.cx = line_len - self.col_offset;
        } else {
            self.cx = line_len;
        }
    }

    // Helper method for wrapping cy to the end of the document.
    // Doesn't modify cx explicitly, but adjusts in helper function call.
    // Doesn't update its CursorState -- this should be done by the calling function.
    fn wrap_cy_to_end(&mut self, data: &[TextRow]) {
        let data_len: i16 = data.len().try_into().unwrap();
        if data_len > self.rows.try_into().unwrap() {
            self.row_offset = data_len.wrapping_sub(self.rows.try_into().unwrap());
            self.cy = data_len - self.row_offset;
        } else {
            self.cy = data_len;
        }
        self.check_and_fix_cx(data);
    }

    // Corrects cx if needed. Intended to be used as a helper method when there's a chance that cx exceeds the current line.
    // Mostly useful for use after cy is forcibly changed by a wrap or scroll event.
    // Doesn't update its CursorState -- this should be done by the calling function.
    fn check_and_fix_cx(&mut self, data: &[TextRow]) {
        if data.len() > self.cy.try_into().unwrap() && self.cx > data[self.cy as usize].length() {
            self.cx = data[self.cy as usize].length();
            self.col_offset = 0;
        }

        // if final line in editor
        if data.len() == self.cy.try_into().unwrap() {
            self.cx = 0;
            self.col_offset = 0;
        }
    }
}
