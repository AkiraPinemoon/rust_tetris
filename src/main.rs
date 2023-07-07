
mod util;
mod tetromino;
mod gamestate;
mod rendering;

fn main() {
    let fps = 30;
    let millispf = 1000 / fps;

    let mut g = gamestate::GameState::new();

    let mut frame: usize = 0;
    let mut renderer: Box<dyn rendering::Renderer> = Box::new(rendering::SdlRenderer::new());

    let mut left = false;
    let mut right = false;
    let mut down = false;

    while g.state == gamestate::State::Running {
        if frame % (200 / millispf as usize) == 0 { g.step(); }

        renderer.draw(&mut g);

        let events = renderer.get_events();
        if events.contains(&util::Event::Quit) { break }
        if events.contains(&util::Event::KeyDown(util::Keycode::D)) { right = true }
        if events.contains(&util::Event::KeyDown(util::Keycode::A)) { left = true }
        if events.contains(&util::Event::KeyDown(util::Keycode::S)) { down = true }
        if events.contains(&util::Event::KeyDown(util::Keycode::Space)) { g.rotate(util::RotDirection::Clockwise) }
        if events.contains(&util::Event::KeyDown(util::Keycode::Shift)) { g.rotate(util::RotDirection::CounterClockwise) }
        if events.contains(&util::Event::KeyUp(util::Keycode::D)) { right = false }
        if events.contains(&util::Event::KeyUp(util::Keycode::A)) { left = false }
        if events.contains(&util::Event::KeyUp(util::Keycode::S)) { down = false }

        if frame % (100 / millispf as usize) == 0 {
            if right { g.move_right() }
            if left { g.move_left() }
        }

        if frame % (100 / millispf as usize) == 0 {
            if down { let _ = g.fall(); }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(millispf));
        frame += 1;
    }
}
