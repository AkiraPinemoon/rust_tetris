
mod util;
mod tetromino;
mod gamestate;
mod io;
mod rendering;

fn main() {
    let fps = 15;
    let millispf = 1000 / fps;

    let mut g = gamestate::GameState::new();
    let i = io::IO::new();
    print!("{esc}[2J", esc = 27 as char);

    let mut rothold = false;

    let mut frame: usize = 0;

    let renderer: Box<dyn rendering::Renderer> = Box::new(rendering::ConsoleRenderer{});
    while g.state == gamestate::State::Running {
        if frame % 5 == 0 { g.step(); }

        renderer.draw(&g);

        let keys = i.get_keys();
        if keys.contains(&device_query::Keycode::D) { g.move_right() }
        if keys.contains(&device_query::Keycode::A) { g.move_left() }
        if keys.contains(&device_query::Keycode::S) { let _ = g.fall(); }
        if keys.contains(&device_query::Keycode::Space) {
            if !rothold { g.rotate(util::RotDirection::Clockwise) }
            rothold = true;
        } else {
            rothold = false;
        }

        std::thread::sleep(std::time::Duration::from_millis(millispf));
        frame += 1;
    }

    println!("lost :(");
}
