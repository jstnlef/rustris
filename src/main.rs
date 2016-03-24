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
    // Setup
    let window_title = format!("Rustris {}", VERSION);

    // Game Window
    let window: PistonWindow =
        WindowSettings::new(window_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    // UI Struct
    let mut ui = create_ui(&window);

    let mut game = Rustris::new();

    // Game Loop
    for e in window.ups(60) {
        // let the UI handle the event
        ui.handle_event(&e);

        match e.event {
            Some(Event::Input(input)) => {
                game.on_input(input);
            }
            Some(Event::Update(update_args)) => {
                game.on_update(update_args);
                e.update(|_| ui.set_widgets(|ui| set_ui(ui, &game)));
            }
            Some(Event::Render(_)) => {
                e.draw_2d(|c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    ui.draw(c, g);
                    // TODO: Make the game itself a custom widget.
                    game.render(c, g);
                });
            }
            _ => {}
        }
    }
}

// fn set_ui(ui: &mut Ui, fps_count: usize) {
//     use conrod::{Canvas, color, Colorable, Positionable, Scalar, Sizeable, Text};

//     Canvas::new().flow_right(&[
//         (MENU, Canvas::new().color(color::BLUE).pad(50.0)),
//         (PLAY_AREA, Canvas::new().color(color::BLACK).pad(50.0))
//     ]).set(MASTER, ui);

//     const PAD: Scalar = 10.0;

//     let fps_text: &str = &("FPS: ".to_string() + &fps_count.to_string());
//     Text::new(fps_text)
//         .color(color::BLACK)
//         .font_size(14)
//         .padded_w_of(MASTER, PAD)
//         .mid_top_with_margin_on(MASTER, PAD)
//         .align_text_left()
//         .line_spacing(5.0)
//         .set(FPS_COUNTER, ui);
// }
