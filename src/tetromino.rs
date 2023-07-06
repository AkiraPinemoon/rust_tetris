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

    pub fn get_tiles(&self) -> [util::UPos2d; 4] {
        get_tiles(self.shape, self.orientation)
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut shape = [[false; 4]; 4];
        for tile in self.get_tiles().iter() {
            shape[tile.y][tile.x] = true;
        }

        let out = shape.into_iter().map(|line| {
            line.into_iter().map(|cell| {
                if cell { "[]".to_string() } else { ". ".to_string() }
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n");

        write!(f,"Tetromino:\n{}" , out)
    }
}

// todo: rotation reversed
fn get_tiles(shape: Shape, orientation: util::Orientation) -> [util::UPos2d; 4] {
    use Shape::*;
    use util::Orientation::*;

    match (shape, orientation) {
        (O, _) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 0, y: 2 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },

        (I, North) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 1, y: 3 }]
        },
        (I, East) => {
            [util::UPos2d{ x: 0, y: 2 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 2 }, util::UPos2d{ x: 3, y: 2 }]
        },
        (I, South) => {
            [util::UPos2d{ x: 2, y: 0 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 2 }, util::UPos2d{ x: 2, y: 3 }]
        },
        (I, West) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 3, y: 1 }]
        },

        (L, North) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 0, y: 2 }]
        },
        (L, East) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (L, South) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 0 }]
        },
        (L, West) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 0, y: 0 }]
        },

        (J, North) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 0, y: 0 }]
        },
        (J, East) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 0, y: 2 }]
        },
        (J, South) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (J, West) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 0 }]
        },

        (S, North) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 0, y: 2 }]
        },
        (S, East) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (S, South) => {
            [util::UPos2d{ x: 2, y: 0 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },
        (S, West) => {
            [util::UPos2d{ x: 0, y: 0 }, util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }]
        },

        (Z, North) => {
            [util::UPos2d{ x: 0, y: 0 }, util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },
        (Z, East) => {
            [util::UPos2d{ x: 0, y: 2 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }]
        },
        (Z, South) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (Z, West) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 2, y: 0 }]
        },

        (T, North) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 0, y: 1 }]
        },
        (T, East) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },
        (T, South) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 1 }]
        },
        (T, West) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 1, y: 0 }]
        },
    }
}
