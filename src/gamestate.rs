use std::fmt::Display;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{tetromino::{self, Shape, Color, Tetromino}, util::{self, Pos2d}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
}

pub struct GameState {
    pub grid: [[Option<tetromino::Color>; 10]; 20],
    pub current: Option<(tetromino::Tetromino, util::Pos2d)>,
    pub state: State,
    pub next: Vec<tetromino::Shape>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            grid: [[None; 10]; 20],
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
                    for tile in tetro.get_tiles() {
                        self.grid[(tile.y as isize + pos.y) as usize][(tile.x as isize + pos.x) as usize] = Some(tetro.color);
                    }
                }

                self.current = None;

                self.spawn()
            },
        }

        self.remove_lines();

        if self.current.is_none() { self.spawn() }
    }

    pub fn fall(&mut self) -> Result<(), ()> {
        match self.current {
            None => Ok(()),
            Some((tetro, pos)) => {
                let test = self.fit_test(tetro, util::Pos2d{ x: pos.x, y: pos.y + 1});

                if test {
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
                if !self.fit_test(tetro, Pos2d{ x: pos.x + 1, y: pos.y}) { return }
                self.current = Some((tetro, Pos2d{ x: pos.x + 1, y: pos.y }));
            }
        }
    }

    pub fn move_left(&mut self) {
        match self.current {
            None => (),
            Some((tetro, pos)) => {
                if !self.fit_test(tetro, Pos2d{ x: pos.x - 1, y: pos.y}) { return }
                self.current = Some((tetro, Pos2d{ x: pos.x - 1, y: pos.y }));
            }
        }
    }

    pub fn rotate(&mut self, direction: util::RotDirection) {
        match self.current {
            None => (),
            Some((mut tetro, pos)) => {
                let offsets = tetromino::get_shape_offset_tests(tetro.shape, tetro.orientation, direction);
                tetro.rotate(direction);

                for offset in offsets.into_iter() {
                    let test = self.fit_test(tetro, Pos2d { x: pos.x + offset.x, y: pos.y + offset.y });
                    if test {
                        self.current = Some((tetro, Pos2d { x: pos.x + offset.x, y: pos.y + offset.y }));
                        return;
                    }
                }
            },
        }
    }

    fn remove_lines(&mut self) {
        for (y, &line) in self.grid.clone().iter().enumerate() {
            let full = line.into_iter().filter(|&cell| { cell.is_none() }).collect::<Vec<Option<Color>>>().len() == 0;

            if full {
                for ymov in (1..y).rev() {
                    self.grid[ymov + 1] = self.grid[ymov];
                }
                self.grid[0] = [None; 10];
            }
        }
    }

    fn fit_test(&self, tetro: Tetromino, pos: util::Pos2d) -> bool {
        for tile in tetro.get_tiles() {
            if (tile.x as isize + pos.x) > 9 { return false }
            if (tile.x as isize + pos.x) < 0 { return false }
            if (tile.y as isize + pos.y) > 19 { return false }
            if (tile.y as isize + pos.y) < 0 { return false }
            if self.grid[(tile.y as isize + pos.y) as usize][(tile.x as isize + pos.x) as usize].is_some() { return false }
        }
        true
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dsp_grid = self.grid.clone();

        if let Some((tetro, pos)) = self.current {
            for tile in tetro.get_tiles() {
                dsp_grid[(tile.y as isize + pos.y) as usize][(tile.x as isize + pos.x) as usize] = Some(tetro.color);
            }
        }

        let out = dsp_grid.into_iter().map(|line| {
            line.into_iter().map(|cell| {
                match cell {
                    None => ". ".to_string(),
                    Some(col) => match col {
                        Color::Teal => "[]".to_string(),
                        Color::Blue => "██".to_string(),
                        Color::Orange => "▒▒".to_string(),
                        Color::Yellow => "()".to_string(),
                        Color::Green => "{}".to_string(),
                        Color::Purple => "▓▓".to_string(),
                        Color::Red => "░░".to_string(),
                    },

                }
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n");

        write!(f,"GameState:\n{}" , out)
    }
}
