use crate::tetromino::Color;


pub trait Renderer {
    fn draw(&self, gamestate: &crate::gamestate::GameState);
}

pub struct ConsoleRenderer {

}

impl Renderer for ConsoleRenderer {
    fn draw(&self, gamestate: &crate::gamestate::GameState) {
        let mut dsp_grid = gamestate.grid.clone();

        if let Some((tetro, pos)) = gamestate.current {
            for tile in tetro.get_tiles() {
                dsp_grid[(tile.y as isize + pos.y) as usize][(tile.x as isize + pos.x) as usize] = Some(tetro.color);
            }
        }

        let out = dsp_grid.into_iter().map(|line| {
            line.into_iter().map(|cell| {
                match cell {
                    None => "  ".to_string(),
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
        }).collect::<Vec<String>>().join("▌\n▐");

        print!("{esc}[1;1H", esc = 27 as char);
        println!("▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌\n▐       Tetris       ▌\n▐▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌\n▐{}▌\n▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘", out);
    }
}
