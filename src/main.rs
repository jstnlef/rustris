extern crate piston_window;

use piston_window::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() {
    let window_title = format!("Rustris {}", VERSION);
    let window: PistonWindow = WindowSettings::new(window_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true).build().unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
