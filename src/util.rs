
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

impl Orientation {
    pub fn rotate(&self, direction: RotDirection) -> Self {
        match direction {
            RotDirection::Clockwise => {
                match self {
                    Orientation::North => Orientation::East,
                    Orientation::East => Orientation::South,
                    Orientation::South => Orientation::West,
                    Orientation::West => Orientation::North,
                }
            },
            RotDirection::CounterClockwise => {
                match self {
                    Orientation::North => Orientation::West,
                    Orientation::East => Orientation::North,
                    Orientation::South => Orientation::East,
                    Orientation::West => Orientation::South,
                }
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct UPos2d {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub struct Pos2d {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Event {
    KeyDown(Keycode),
    KeyUp(Keycode),
    Quit,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Keycode {
    W, A, S, D, Space, Shift
}
