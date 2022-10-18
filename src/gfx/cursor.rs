use crate::data::{textrow::TextRow, payload::CursorState};

// CursorHandler. Deals with all of the messy logic around scroller and cursor movement.
// cx and cy represent the x,y coords of the cursor's location.
// row_offset and col_offset represent the degree to which the cursor is moved 'off-screen' on either axis.
// Also stores the size of the terminal window (upon program initialization -- doesn't mutate) and its current state.
pub struct CursorHandler {
    pub(in crate::gfx)cx: i16,
    pub(in crate::gfx)cy: i16,
    row_offset: i16,
    col_offset: i16,
    rows: u16,
    cols: u16,
    state: CursorState,
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
        }
    }

    // Returns the CursorHandler's current state.
    pub(in crate::gfx) fn get_state(&mut self) -> CursorState {
        self.state
    }

    // Entry point for handling a cursor event.
    // x: whether this is a move on the x-axis or not (if not, then y-axis)
    // val: proposed new value for cx/cy
    // data: reference to the RenderDriver's current state of text
    pub(in crate::gfx) fn handle_cursor(&mut self, x: bool, val: i16, data: &Vec<TextRow>) {
        if x {
            self.handle_x_move(val, data);
        } else {
            self.handle_y_move(val, data);
        }
    }

    // Update this CursorHandler's current state, using the CursorHandler's current relevant values.
    fn update_state(&mut self) {
        self.state = self.state.update(self.cx, self.cy, self.row_offset, self.col_offset);
    }

    // Handle a cursor move along the y-axis, with a proposed cy value and a reference to the RenderDriver's current data.
    //
    // IF the value is less than 1 (we are trying to go off-screen to the top) AND there is row offset (we are off-screen to the bottom)
    // THEN: decrement row offset (move us back toward the beginning of the document)
    //
    // IF the value is equal to the end of the window (we are trying to go off-screen to the bottom) AND there is still more of the current document to view
    // THEN: increment row offset (move us further into the document)
    //
    // IF the value is not less than 1 or equal to the end of the window (we are not pushing the borders of the document)
    // THEN: just move cy around within the window
    //
    // Will also corrext cx if we skip from a long line to a shorter one.
    // Updates its CursorState after all values have been changed.
    fn handle_y_move(&mut self, val: i16, data: &Vec<TextRow>) {
        if val == -1 && self.row_offset > 0 {
            self.row_offset = self.row_offset - 1;
        }
        if val == (self.rows - 1).try_into().unwrap() && (self.row_offset + self.cy) < data.len().try_into().unwrap() {
            self.row_offset = self.row_offset + 1;
        }
        if val != -1 && val != (self.rows - 1).try_into().unwrap() {
            self.cy = val;
        }

        // correct cx if we just skipped to a shorter line
        if self.cy + 1 <= data.len().try_into().unwrap() && self.cx > data[self.cy as usize].length() {
            self.cx = data[self.cy as usize].length()
        }

        self.update_state();
    }

    // Handle a cursor move along the x-axis, with a proposed cx value and a reference to the RenderDriver's current data.
    // Updates its CursorState after all values have been changed.
    fn handle_x_move(&mut self, val: i16, data: &Vec<TextRow>) {
        let mut has_wrapped = false;
        if val == -1 { // moving offscreen to the left
            if self.col_offset > 0 {  // is there more data to show here?
                self.col_offset = self.col_offset - 1;
            } else if self.cy >= 1 { // is there a line we can wrap to?
                self.cy = self.cy - 1;
                self.wrap_cx_to_end(data);
                self.col_offset = 0;
                has_wrapped = true;
            }
        }
        if val == (self.cols - 1).try_into().unwrap() { // offscreen to the right
            if data[self.cy as usize].length() >= val + self.col_offset { // is there more data to show here?
                self.col_offset = self.col_offset + 1;
            } else if data.len() + 1 > self.cy.try_into().unwrap() { // is there a line we can wrap to?
                self.cy = self.cy + 1;
                self.cx = 0;
                self.col_offset = 0;
                has_wrapped = true;
            }
        }
        if val == data[self.cy as usize].length() + 1 { // end of line
            if data.len() + 1 > self.cy.try_into().unwrap() { // is there a line we can wrap to?
                self.cy = self.cy + 1;
                self.cx = 0;
                self.col_offset = 0;
                has_wrapped = true;
            }
        }
        if val != -1 && val != (self.cols - 1).try_into().unwrap() && !has_wrapped { // not going offscreen
            self.cx = val;
        }

        self.update_state();
    }

    // Helper method for wrapping cx to the end of the line indexed by the handler's current cy.
    // Sets cx to the very end of this line, adding col_offset if the line is long enough.
    // Doesn't update its CursorState -- this should be done by the calling function.
    fn wrap_cx_to_end(&mut self, data: &Vec<TextRow>) {
        let line_len = data[self.cy as usize].length();
        if line_len > self.rows.try_into().unwrap() {
            self.col_offset = line_len.wrapping_sub(self.rows.try_into().unwrap());
            self.cx = line_len - self.col_offset;
        } else {
            self.cx = line_len;
        }
    }
}
