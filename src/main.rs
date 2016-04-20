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

    let mut window: PistonWindow =
        WindowSettings::new(window_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut ui = create_ui(&window);
    let mut game = Rustris::new();

    window.set_ups(60);

    while let Some(event) = window.next() {
        // let the UI handle the event
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|ui| set_ui(ui, &mut game)));

        match event {
            Event::Input(input) => {
                game.on_input(input);
            }
            Event::Update(update_args) => {
                game.on_update(update_args);
            }
            Event::Render(_) => {
                window.draw_2d(&event, |c, g| {
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
