use crate::data::{textrow::TextRow, payload::CursorState};

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

    pub(in crate::gfx) fn get_state(&mut self) -> CursorState {
        self.state
    }

    pub(in crate::gfx) fn handle_cursor(&mut self, x: bool, val: i16, data: &Vec<TextRow>) {
        if x {
            self.handle_x_move(val, data);
        } else {
            self.handle_y_move(val, data);
        }
    }

    fn update_state(&mut self) {
        self.state = self.state.update(self.cx, self.cy, self.row_offset, self.col_offset);
    }

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

    fn handle_x_move(&mut self, val: i16, data: &Vec<TextRow>) {
        if val == -1 && self.col_offset > 0 {
            self.col_offset = self.col_offset - 1;
        }
        if val == (self.cols - 1).try_into().unwrap() && data[self.cy as usize].length() >= val + self.col_offset {
            self.col_offset = self.col_offset + 1;
        }
        if val != -1 && val != (self.cols - 1).try_into().unwrap() {
            self.cx = val;
        }

        self.update_state();
    }
}
