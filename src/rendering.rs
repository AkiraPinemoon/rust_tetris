use sdl2::ttf::Font;

use crate::tetromino::Color;

pub trait Renderer {
    fn draw(&mut self, gamestate: &mut crate::gamestate::GameState);
    fn get_events(&mut self) -> Vec<crate::util::Event>;
}

use lazy_static::lazy_static;

lazy_static! {
    static ref TTF_CONTEXT: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

pub struct SdlRenderer<'a> {
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::WindowCanvas,
    font: sdl2::ttf::Font<'a, 'a>,
}

impl<'a> SdlRenderer<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust Tetris", 300, 600)
            .position_centered()
            .allow_highdpi()
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump().unwrap();

        let ttf_context = &TTF_CONTEXT;

        let font: Font<'a, 'a> = ttf_context.load_font("./res/VT323-Regular.ttf", 64).unwrap();

        Self { event_pump, canvas, font }
    }

    fn get_draw_transforms(&self) -> (crate::util::Pos2d, f32) {
        let max_x_tilesize = self.canvas.output_size().unwrap().0 as f32 / 10.0;
        let max_y_tilesize = self.canvas.output_size().unwrap().1 as f32 / 20.0;

        let tilesize = if max_x_tilesize < max_y_tilesize { max_x_tilesize } else { max_y_tilesize };

        let x = ((self.canvas.output_size().unwrap().0 as f32 - 10.0 * tilesize) / 2.0) as isize;
        let y = ((self.canvas.output_size().unwrap().1 as f32 - 20.0 * tilesize) / 2.0) as isize;

        (crate::util::Pos2d{ x, y }, tilesize)
    }

    fn draw_grid(&mut self) {
        let (pos, tilesize) = self.get_draw_transforms();

        let color = sdl2::pixels::Color::RGB(25, 25, 25);
        self.canvas.set_draw_color(color);

        for y in 0..21 {
            self.canvas.fill_rect(sdl2::rect::Rect::new(
                pos.x as i32 - 1, pos.y as i32 + (y as f32 * tilesize) as i32 -1, (10 as f32 * tilesize) as u32, 2
            )).unwrap();
        }

        for x in 0..11 {
            self.canvas.fill_rect(sdl2::rect::Rect::new(
                pos.x as i32 + (x as f32 * tilesize) as i32 -1, pos.y as i32 - 1, 2, (20 as f32 * tilesize) as u32
            )).unwrap();
        }
    }

    fn draw_tiles(&mut self, gamestate: &mut crate::gamestate::GameState) {
        let (pos, tilesize) = self.get_draw_transforms();

        for (i, row) in gamestate.grid.into_iter().rev().take(20).rev().enumerate() {
            for (j, col) in row.into_iter().enumerate() {

                let (r, g, b) = match col {
                    Some(Color::Red) => (255, 0, 0),
                    Some(Color::Blue) => (0, 0, 255),
                    Some(Color::Green) => (0, 255, 0),
                    Some(Color::Orange) => (235, 69, 17),
                    Some(Color::Purple) => (56, 2, 59),
                    Some(Color::Teal) => (34, 124, 157),
                    Some(Color::Yellow) => (255, 255, 0),
                    None => { continue; },
                };

                let (x, y) = (
                    (j as f32 * tilesize) as i32,
                    (i as f32 * tilesize) as i32,
                );

                let (w, h) = (
                    (((j + 1) as f32 * tilesize) as i32 - x) as u32,
                    (((i + 1) as f32 * tilesize) as i32 - y) as u32,
                );

                let color = sdl2::pixels::Color::RGB(r, g, b);
                self.canvas.set_draw_color(color);
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(pos.x as i32 + x, pos.y as i32 + y, w, h))
                    .unwrap();
            }
        }
    }

    fn draw_tetro(&mut self, gamestate: &mut crate::gamestate::GameState) {
        if gamestate.current.is_none() { return; }
        let (tetro, pos) = gamestate.current.unwrap();

        let (draw_pos, tilesize) = self.get_draw_transforms();

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

            let (x, y) = (
                ((pos.x + tile.x as isize) as f32 * tilesize) as i32,
                ((pos.y + tile.y as isize - 2) as f32 * tilesize) as i32,
            );

            let (w, h) = (
                (((pos.x + tile.x as isize + 1) as f32 * tilesize) as i32 - x) as u32,
                (((pos.y + tile.y as isize - 1) as f32 * tilesize) as i32 - y) as u32,
            );

            let color = sdl2::pixels::Color::RGB(r, g, b);
            self.canvas.set_draw_color(color);
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(draw_pos.x as i32 + x, draw_pos.y as i32 + y, w, h))
                .unwrap();
        }
    }

    fn draw_score(&mut self, gamestate: &mut crate::gamestate::GameState) {
        let text_surface = self.font.render(&gamestate.score.to_string())
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let texture_creator = self.canvas.texture_creator();

        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .map_err(|e| e.to_string()).unwrap();

        let sdl2::render::TextureQuery { width, height, .. } = text_texture.query();
        let canvas_width = self.canvas.output_size().unwrap().0;

        let _ = self.canvas.copy(&text_texture, None, Some(sdl2::rect::Rect::new((canvas_width as i32 - width as i32) / 2, 0, width, height)));
    }
}

impl Renderer for SdlRenderer<'_> {
    fn draw(&mut self, gamestate: &mut crate::gamestate::GameState) {

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.draw_tiles(gamestate);
        self.draw_tetro(gamestate);
        self.draw_grid();
        self.draw_score(gamestate);

        self.canvas.present();
    }

    fn get_events(&mut self) -> Vec<crate::util::Event> {
        let mut events = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => { events.push(crate::util::Event::Quit) },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } => { events.push(crate::util::Event::KeyDown(crate::util::Keycode::W)) },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } => { events.push(crate::util::Event::KeyDown(crate::util::Keycode::A)) },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } => { events.push(crate::util::Event::KeyDown(crate::util::Keycode::S)) },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } => { events.push(crate::util::Event::KeyDown(crate::util::Keycode::D)) },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), repeat: false, .. } => { events.push(crate::util::Event::KeyDown(crate::util::Keycode::Space)) },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } => { events.push(crate::util::Event::KeyDown(crate::util::Keycode::Shift)) },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } => { events.push(crate::util::Event::KeyUp(crate::util::Keycode::W)) },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } => { events.push(crate::util::Event::KeyUp(crate::util::Keycode::A)) },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } => { events.push(crate::util::Event::KeyUp(crate::util::Keycode::S)) },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } => { events.push(crate::util::Event::KeyUp(crate::util::Keycode::D)) },
                _ => {}
            }
        };
        events
    }
}
