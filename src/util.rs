
#[derive(Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub enum RotDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy)]
pub struct Pos2d {
    pub x: usize,
    pub y: usize,
}
