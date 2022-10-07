pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}