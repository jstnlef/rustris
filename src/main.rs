extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new("Rustris", [640, 480])
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
