use crate::{gamestate, tetromino::Color};

pub trait Renderer {
    fn draw(&mut self, gamestate: &mut crate::gamestate::GameState);
}

pub struct ConsoleRenderer {}

impl Renderer for ConsoleRenderer {
    fn draw(&mut self, gamestate: &mut crate::gamestate::GameState) {
        let mut dsp_grid = gamestate.grid.clone();

        if let Some((tetro, pos)) = gamestate.current {
            for tile in tetro.get_tiles() {
                dsp_grid[(tile.y as isize + pos.y) as usize][(tile.x as isize + pos.x) as usize] =
                    Some(tetro.color);
            }
        }

        let out = dsp_grid
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|cell| match cell {
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
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("▌\n▐");

        print!("{esc}[1;1H", esc = 27 as char);
        println!("▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌\n▐       Tetris       ▌\n▐▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌\n▐{}▌\n▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘", out);
    }
}

pub struct SdlRenderer {
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::WindowCanvas,
}

impl SdlRenderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();

        Self { event_pump, canvas }
    }

    fn draw_field(&mut self, gamestate: &mut crate::gamestate::GameState) {
        for (i, row) in gamestate.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let (r, g, b) = match col {
                    Some(Color::Red) => (255, 0, 0),
                    Some(Color::Blue) => (0, 0, 255),
                    Some(Color::Green) => (0, 255, 0),
                    Some(Color::Orange) => (235, 69, 17),
                    Some(Color::Purple) => (56, 2, 59),
                    Some(Color::Teal) => (34, 124, 157),
                    Some(Color::Yellow) => (255, 255, 0),
                    None => (0, 0, 0),
                };

                let color = sdl2::pixels::Color::RGB(r, g, b);
                self.canvas.set_draw_color(color);
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(j as i32 * 10, i as i32 * 10, 10, 10))
                    .unwrap();
            }
        }
    }

    fn draw_tetro(&mut self, gamestate: &mut crate::gamestate::GameState) {
        if gamestate.current.is_none() { return; }
        let (tetro, pos) = gamestate.current.unwrap();

        for tile in tetro.get_tiles().into_iter() {
            let (r, g, b) = match tetro.color {
                Color::Red => (255, 0, 0),
                Color::Blue => (0, 0, 255),
                Color::Green => (0, 255, 0),
                Color::Orange => (235, 69, 17),
                Color::Purple => (56, 2, 59),
                Color::Teal => (34, 124, 157),
                Color::Yellow => (255, 255, 0),
            };

            let color = sdl2::pixels::Color::RGB(r, g, b);
            self.canvas.set_draw_color(color);
            self.canvas
                .fill_rect(sdl2::rect::Rect::new((pos.x + tile.x as isize) as i32 * 10, (pos.y + tile.y as isize) as i32 * 10, 10, 10))
                .unwrap();
        }
    }
}

impl Renderer for SdlRenderer {
    fn draw(&mut self, gamestate: &mut crate::gamestate::GameState) {

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.draw_field(gamestate);
        self.draw_tetro(gamestate);

        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    gamestate.state = crate::gamestate::State::Lost;
                }
                _ => {}
            }
        }
        self.canvas.present();
    }
}
