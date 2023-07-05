use std::fmt::Display;
use crate::util;

#[derive(Clone, Copy)]
pub enum Shape { O, I, L, J, S, Z, T }

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub shape: Shape,
    pub orientation: util::Orientation,
}

impl Tetromino {
    pub fn new(shape: Shape, orientation: util::Orientation) -> Self {
        Self {
            shape,
            orientation,
        }
    }

    pub fn rotate(&mut self, direction: util::RotDirection) {
        match direction {
            util::RotDirection::Clockwise => {
                self.orientation = match self.orientation {
                    util::Orientation::North => util::Orientation::East,
                    util::Orientation::East => util::Orientation::South,
                    util::Orientation::South => util::Orientation::West,
                    util::Orientation::West => util::Orientation::North,
                }
            },
            util::RotDirection::CounterClockwise => {
                self.orientation = match self.orientation {
                    util::Orientation::North => util::Orientation::West,
                    util::Orientation::East => util::Orientation::North,
                    util::Orientation::South => util::Orientation::East,
                    util::Orientation::West => util::Orientation::South,
                }
            },
        }
    }

    pub fn get_tiles(&self) -> [(usize, usize); 4] {
        get_tiles(self.shape, self.orientation)
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut shape = [[false; 4]; 4];
        for tile in self.get_tiles().iter() {
            shape[tile.0][tile.1] = true;
        }

        let out = shape.into_iter().map(|line| {
            line.into_iter().map(|cell| {
                if cell { "[]".to_string() } else { ". ".to_string() }
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n");

        write!(f,"Tetromino:\n{}" , out)
    }
}

fn get_tiles(shape: Shape, orientation: util::Orientation) -> [(usize, usize); 4] {
    use Shape::*;
    use util::Orientation::*;

    match (shape, orientation) {
        (O, _) => {
            [(0, 1), (0, 2), (1, 1), (1, 2)]
        },

        (I, North) => {
            [(1, 0), (1, 1), (1, 2), (1, 3)]
        },
        (I, East) => {
            [(0, 2), (1, 2), (2, 2), (3, 2)]
        },
        (I, South) => {
            [(2, 0), (2, 1), (2, 2), (2, 3)]
        },
        (I, West) => {
            [(0, 1), (1, 1), (2, 1), (3, 1)]
        },

        (L, North) => {
            [(1, 0), (1, 1), (1, 2), (0, 2)]
        },
        (L, East) => {
            [(0, 1), (1, 1), (2, 1), (2, 2)]
        },
        (L, South) => {
            [(1, 0), (1, 1), (1, 2), (2, 0)]
        },
        (L, West) => {
            [(0, 1), (1, 1), (2, 1), (0, 0)]
        },

        (J, North) => {
            [(1, 0), (1, 1), (1, 2), (0, 0)]
        },
        (J, East) => {
            [(0, 1), (1, 1), (2, 1), (0, 2)]
        },
        (J, South) => {
            [(1, 0), (1, 1), (1, 2), (2, 2)]
        },
        (J, West) => {
            [(0, 1), (1, 1), (2, 1), (2, 0)]
        },

        (S, North) => {
            [(1, 0), (1, 1), (0, 1), (0, 2)]
        },
        (S, East) => {
            [(0, 1), (1, 1), (1, 2), (2, 2)]
        },
        (S, South) => {
            [(2, 0), (2, 1), (1, 1), (1, 2)]
        },
        (S, West) => {
            [(0, 0), (1, 0), (1, 1), (2, 1)]
        },

        (Z, North) => {
            [(0, 0), (0, 1), (1, 1), (1, 2)]
        },
        (Z, East) => {
            [(0, 2), (1, 2), (1, 1), (2, 1)]
        },
        (Z, South) => {
            [(1, 0), (1, 1), (2, 1), (2, 2)]
        },
        (Z, West) => {
            [(0, 1), (1, 1), (1, 0), (2, 0)]
        },

        (T, North) => {
            [(1, 0), (1, 1), (1, 2), (0, 1)]
        },
        (T, East) => {
            [(0, 1), (1, 1), (2, 1), (1, 2)]
        },
        (T, South) => {
            [(1, 0), (1, 1), (1, 2), (2, 1)]
        },
        (T, West) => {
            [(0, 1), (1, 1), (2, 1), (1, 0)]
        },
    }
}
