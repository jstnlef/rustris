#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate rand;

// use conrod::{Theme, Widget};
use piston_window::*;

mod board;
mod colors;
mod game;
use game::{Rustris, Game};
mod settings;
use settings::*;
mod tetromino;

// Constants for the various widgets and canvases
// widget_ids! {
//     // Canvas IDs
//     MASTER,
//     MENU,
//     PLAY_AREA,
//     // Widget IDs
//     FPS_COUNTER
// }

type Ui = conrod::Ui<Glyphs>;

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
    // let mut ui = create_ui(&window);

    let mut game = Rustris::new();
    // Game Loop
    for e in window {
        match e.event {
            Some(Event::Input(input)) => {
                game.on_input(input);
            }
            Some(Event::Update(update_args)) => {
                game.on_update(update_args);
            }
            Some(Event::Render(render_args)) => {
                game.on_render(render_args, e);
            }
            _ => {}
        }
    }
}

// fn create_ui(window: &PistonWindow) -> Ui {
//     let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
//     let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
//     let theme = Theme::default();
//     let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
//     Ui::new(glyph_cache.unwrap(), theme)
// }

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
