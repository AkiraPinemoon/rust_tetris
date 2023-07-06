use std::fmt::Display;
use crate::util::{self, Pos2d, RotDirection};

#[derive(Clone, Copy)]
pub enum Shape { O, I, L, J, S, Z, T }

#[derive(Clone, Copy)]
pub enum Color {
    Teal,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub shape: Shape,
    pub orientation: util::Orientation,
    pub color: Color,
}

impl Tetromino {
    pub fn new(shape: Shape, orientation: util::Orientation) -> Self {
        Self {
            shape,
            orientation,
            color: get_shape_color(shape),
        }
    }

    pub fn rotate(&mut self, direction: util::RotDirection) {
        self.orientation = self.orientation.rotate(direction);
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

fn get_tiles(shape: Shape, orientation: util::Orientation) -> [util::UPos2d; 4] {
    use Shape::*;
    use util::Orientation::*;

    match (shape, orientation) {
        (O, _) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 0, y: 2 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },

        (I, West) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 1, y: 3 }]
        },
        (I, South) => {
            [util::UPos2d{ x: 0, y: 2 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 2 }, util::UPos2d{ x: 3, y: 2 }]
        },
        (I, East) => {
            [util::UPos2d{ x: 2, y: 0 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 2 }, util::UPos2d{ x: 2, y: 3 }]
        },
        (I, North) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 3, y: 1 }]
        },

        (J, West) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 0, y: 2 }]
        },
        (J, South) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (J, East) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 0 }]
        },
        (J, North) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 0, y: 0 }]
        },

        (L, West) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 0, y: 0 }]
        },
        (L, South) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 0, y: 2 }]
        },
        (L, East) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (L, North) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 0 }]
        },

        (Z, West) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 0, y: 2 }]
        },
        (Z, South) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (Z, East) => {
            [util::UPos2d{ x: 2, y: 0 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },
        (Z, North) => {
            [util::UPos2d{ x: 0, y: 0 }, util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }]
        },

        (S, West) => {
            [util::UPos2d{ x: 0, y: 0 }, util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },
        (S, South) => {
            [util::UPos2d{ x: 0, y: 2 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }]
        },
        (S, East) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 2, y: 2 }]
        },
        (S, North) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 2, y: 0 }]
        },

        (T, West) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 0, y: 1 }]
        },
        (T, South) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 1, y: 2 }]
        },
        (T, East) => {
            [util::UPos2d{ x: 1, y: 0 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 1, y: 2 }, util::UPos2d{ x: 2, y: 1 }]
        },
        (T, North) => {
            [util::UPos2d{ x: 0, y: 1 }, util::UPos2d{ x: 1, y: 1 }, util::UPos2d{ x: 2, y: 1 }, util::UPos2d{ x: 1, y: 0 }]
        },
    }
}

fn get_shape_color(shape: Shape) -> Color {
    match shape {
        Shape::I => Color::Teal,
        Shape::J => Color::Blue,
        Shape::L => Color::Orange,
        Shape::O => Color::Yellow,
        Shape::S => Color::Green,
        Shape::T => Color::Purple,
        Shape::Z => Color::Red,
    }
}

pub fn get_shape_offset_tests(shape: Shape, orientation: util::Orientation, direction: util::RotDirection) -> [Pos2d; 5] {
    match direction {
        RotDirection::Clockwise => {
            match shape {
                Shape::J | Shape::L | Shape::S | Shape::T | Shape::Z => {
                    match orientation {
                        util::Orientation::North => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: -1, y: -1 },
                            Pos2d{ x: 0, y: 2 },
                            Pos2d{ x: -1, y: 2 },
                        ]},
                        util::Orientation::East => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: 1, y: 1 },
                            Pos2d{ x: 0, y: -2 },
                            Pos2d{ x: 1, y: -2 },
                        ]},
                        util::Orientation::South => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: 1, y: -1 },
                            Pos2d{ x: 0, y: 2 },
                            Pos2d{ x: 1, y: 2 },
                        ]},
                        util::Orientation::West => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: -1, y: 1 },
                            Pos2d{ x: 0, y: -2 },
                            Pos2d{ x: -1, y: -2 },
                        ]},
                    }
                },
                Shape::I => {
                    match orientation {
                        util::Orientation::North => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -2, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: -2, y: 1 },
                            Pos2d{ x: 1, y: -2 },
                        ]},
                        util::Orientation::East => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: 2, y: 0 },
                            Pos2d{ x: -1, y: -2 },
                            Pos2d{ x: 2, y: 1 },
                        ]},
                        util::Orientation::South => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 2, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: 2, y: -1 },
                            Pos2d{ x: -1, y: 2 },
                        ]},
                        util::Orientation::West => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: -2, y: 0 },
                            Pos2d{ x: 1, y: 2 },
                            Pos2d{ x: -1, y: 1 },
                        ]},
                    }
                },
                Shape::O => {
                    match orientation {
                        util::Orientation::North => {[Pos2d{ x: 0, y: 0 }; 5]},
                        util::Orientation::East => {[Pos2d{ x: 0, y: 0 }; 5]},
                        util::Orientation::South => {[Pos2d{ x: 0, y: 0 }; 5]},
                        util::Orientation::West => {[Pos2d{ x: 0, y: 0 }; 5]},
                    }
                },
            }
        },
        RotDirection::CounterClockwise => {
            match shape {
                Shape::J | Shape::L | Shape::S | Shape::T | Shape::Z => {
                    match orientation {
                        util::Orientation::North => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: 1, y: -1 },
                            Pos2d{ x: 0, y: 2 },
                            Pos2d{ x: 1, y: 2 },
                        ]},
                        util::Orientation::East => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: 1, y: 1 },
                            Pos2d{ x: 0, y: -2 },
                            Pos2d{ x: 1, y: -2 },
                        ]},
                        util::Orientation::South => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: -1, y: -1 },
                            Pos2d{ x: 0, y: 2 },
                            Pos2d{ x: -1, y: 2 },
                        ]},
                        util::Orientation::West => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: -1, y: 1 },
                            Pos2d{ x: 0, y: -2 },
                            Pos2d{ x: -1, y: -2 },
                        ]},
                    }
                },
                Shape::I => {
                    match orientation {
                        util::Orientation::North => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: 2, y: 0 },
                            Pos2d{ x: -1, y: -2 },
                            Pos2d{ x: 2, y: 1 },
                        ]},
                        util::Orientation::East => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 2, y: 0 },
                            Pos2d{ x: -1, y: 0 },
                            Pos2d{ x: 2, y: -1 },
                            Pos2d{ x: -1, y: 2 },
                        ]},
                        util::Orientation::South => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: -2, y: 0 },
                            Pos2d{ x: 1, y: 2 },
                            Pos2d{ x: -2, y: 1 },
                        ]},
                        util::Orientation::West => {[
                            Pos2d{ x: 0, y: 0 },
                            Pos2d{ x: -2, y: 0 },
                            Pos2d{ x: 1, y: 0 },
                            Pos2d{ x: -2, y: 1 },
                            Pos2d{ x: 1, y: -2 },
                        ]},
                    }
                },
                Shape::O => {
                    match orientation {
                        util::Orientation::North => {[Pos2d{ x: 0, y: 0 }; 5]},
                        util::Orientation::East => {[Pos2d{ x: 0, y: 0 }; 5]},
                        util::Orientation::South => {[Pos2d{ x: 0, y: 0 }; 5]},
                        util::Orientation::West => {[Pos2d{ x: 0, y: 0 }; 5]},
                    }
                },
            }
        },
    }
}
