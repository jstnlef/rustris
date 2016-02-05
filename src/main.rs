#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate fps_counter;
extern crate piston_window;

use conrod::{Theme, Widget};
use fps_counter::{FPSCounter};
use piston_window::*;

mod tetromino;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

// Constants for the various widgets and canvases
widget_ids! {
    // Canvas IDs
    MASTER,
    MENU,
    PLAY_AREA,
    // Widget IDs
    FPS_COUNTER
}

type Ui = conrod::Ui<Glyphs>;

fn main() {
    // Setup
    let window_title = format!("Rustris {}", VERSION);
    let mut fps_counter = FPSCounter::new();

    // Game Window
    let window: PistonWindow =
        WindowSettings::new(window_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    // UI Struct
    let mut ui = create_ui(&window);

    // Game Loop
    for e in window.ups(60) {
        let fps_count = fps_counter.tick();
        ui.handle_event(&e);
        e.update(|_| ui.set_widgets(|ui| {
            set_ui(ui, fps_count);
        }));
        e.draw_2d(|c, g| ui.draw_if_changed(c, g));
    }
}

fn create_ui(window: &PistonWindow) -> Ui {
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
    Ui::new(glyph_cache.unwrap(), theme)
}

fn set_ui(ui: &mut Ui, fps_count: usize) {
    use conrod::{Canvas, color, Colorable, Positionable, Scalar, Sizeable, Text};

    Canvas::new().flow_right(&[
        (MENU, Canvas::new().color(color::BLUE).pad(50.0)),
        (PLAY_AREA, Canvas::new().color(color::BLACK).pad(50.0))
    ]).set(MASTER, ui);

    const PAD: Scalar = 10.0;

    let fps_text: &str = &("FPS: ".to_string() + &fps_count.to_string());
    Text::new(fps_text)
        .color(color::BLACK)
        .font_size(14)
        .padded_w_of(MASTER, PAD)
        .mid_top_with_margin_on(MASTER, PAD)
        .align_text_left()
        .line_spacing(5.0)
        .set(FPS_COUNTER, ui);
}
