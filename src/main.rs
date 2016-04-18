#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::{EventLoop, PistonWindow, WindowSettings, Event, UpdateEvent, clear};

mod board;
mod colors;
mod game;
mod randomizer;
mod tetromino;
mod settings;
mod stats;
mod ui;

use game::{Rustris, Game};
use settings::*;
use ui::{create_ui, set_ui};


fn main() {
    let window_title = format!("Rustris {}", VERSION);

    let window: PistonWindow =
        WindowSettings::new(window_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut ui = create_ui(&window);
    let mut game = Rustris::new();

    for e in window.ups(60) {
        // let the UI handle the event
        ui.handle_event(&e);
        e.update(|_| ui.set_widgets(|ui| set_ui(ui, &mut game)));

        match e.event {
            Some(Event::Input(input)) => {
                game.on_input(input);
            }
            Some(Event::Update(update_args)) => {
                game.on_update(update_args);
            }
            Some(Event::Render(_)) => {
                e.draw_2d(|c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    ui.draw(c, g);
                    if game.is_playing() {
                        game.render(c, g);
                    }
                });
            }
            _ => {}
        }
    }
}
