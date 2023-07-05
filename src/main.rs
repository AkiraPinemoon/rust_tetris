
mod util;
mod tetromino;
mod gamestate;
mod io;

fn main() {
    let fps = 5;
    let millispf = 1000 / fps;

    let mut g = gamestate::GameState::new();

    while g.state == gamestate::State::Running {
        g.step();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", g);

        let keys = io::get_keys_pressed();
        if keys.contains(&device_query::Keycode::Right) { g.move_right() }
        if keys.contains(&device_query::Keycode::Left) { g.move_left() }

        std::thread::sleep(std::time::Duration::from_millis(millispf));
    }
}
