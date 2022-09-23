pub enum InputEvent {
    Quit,
    Move(Direction),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}