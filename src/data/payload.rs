// Cursor State
#[derive(Copy, Clone)]
pub struct CursorState {
    pub cx: i16,
    pub cy: i16,
    pub row_offset: i16,
    pub col_offset: i16,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            cx: 0,
            cy: 0,
            row_offset: 0,
            col_offset: 0,
        }
    }

    pub fn update(&mut self, new_cx: i16, new_cy: i16, new_roffset: i16, new_coffset: i16) -> CursorState {
        self.cx = new_cx;
        self.cy = new_cy;
        self.row_offset = new_roffset;
        self.col_offset = new_coffset;
        *self
    }
}
