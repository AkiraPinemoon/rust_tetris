use std::fmt::Display;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{tetromino::{self, Shape}, util::{self, Pos2d}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
    Lost,
}

pub struct GameState {
    pub grid: [[bool; 10]; 20],
    pub current: Option<(tetromino::Tetromino, util::Pos2d)>,
    pub state: State,
    pub next: Vec<tetromino::Shape>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            grid: [[false; 10]; 20],
            current: None,
            state: State::Running,
            next: Vec::new(),
        }
    }

    pub fn spawn(&mut self) {
        if self.next.len() == 0 {
            self.next = vec![Shape::O, Shape::I, Shape::L, Shape::J, Shape::S, Shape::Z, Shape::T];
            self.next.shuffle(&mut thread_rng());
        }

        let shape = self.next.pop().unwrap();
        self.current = Some((tetromino::Tetromino::new(shape, util::Orientation::North), Pos2d{ x: 3, y: 0 }))
    }

    pub fn step(&mut self) {
        match self.fall() {
            Ok(_) => (),
            Err(_) => {
                // put current in grid
                if let Some((tetro, pos)) = self.current {
                    for (x, y) in tetro.get_tiles() {
                        self.grid[y + pos.y][x + pos.x] = true;
                    }
                }

                self.current = None;

                self.spawn()
            },
        }

        if self.current.is_none() { self.spawn() }
    }

    fn fall(&mut self) -> Result<(), ()> {
        match self.current {
            None => Ok(()),
            Some((tetro, mut pos)) => {
                let mut res = true;
                for (x, y) in tetro.get_tiles() {
                    if y + pos.y == 19 { return Err(()) }
                    if self.grid[y + pos.y + 1][x + pos.x] { res = false }
                }

                if res {
                    self.current = Some((tetro, Pos2d{ x: pos.x, y: pos.y + 1 }));
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn move_right(&mut self) {
        match self.current {
            None => (),
            Some((tetro, pos)) => {
                for (x, y) in tetro.get_tiles() {
                    if y + pos.x == 9 { return }
                    if self.grid[y + pos.y][x + pos.x + 1] { return }
                }

                self.current = Some((tetro, Pos2d{ x: pos.x + 1, y: pos.y }));
            }
        }
    }

    pub fn move_left(&mut self) {
        match self.current {
            None => (),
            Some((tetro, pos)) => {
                let mut res = true;
                for (x, y) in tetro.get_tiles() {
                    if y + pos.x == 0 { return }
                    if self.grid[y + pos.y][x + pos.x - 1] { res = false }
                }

                if res { self.current = Some((tetro, Pos2d{ x: pos.x - 1, y: pos.y })) }
            }
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dsp_grid = self.grid.clone();

        if let Some((tetro, pos)) = self.current {
            for (x, y) in tetro.get_tiles() {
                dsp_grid[y + pos.y][x + pos.x] = true;
            }
        }

        let out = dsp_grid.into_iter().map(|line| {
            line.into_iter().map(|cell| {
                if cell { "[]".to_string() } else { ". ".to_string() }
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n");

        write!(f,"GameState:\n{}" , out)
    }
}
