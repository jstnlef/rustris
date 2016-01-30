#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate fps_counter;
extern crate piston_window;

use conrod::{Theme, Widget};
use fps_counter::{FPSCounter};
use piston_window::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

type Ui = conrod::Ui<Glyphs>;

fn main() {
    // Setup
    let window_title = format!("Rustris {}", VERSION);
    let mut fps_counter = FPSCounter::new();

    // Game Window
    let window: PistonWindow =
        WindowSettings::new(window_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // UI layer
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

    widget_ids!{
        MASTER,
        FPS_COUNTER
    };

    Canvas::new().set(MASTER, ui);

    const PAD: Scalar = 10.0;

    let fps_text: &str = &("FPS: ".to_string() + &fps_count.to_string());
    Text::new(fps_text)
        .color(color::LIGHT_BLUE)
        .font_size(14)
        .padded_w_of(MASTER, PAD)
        .mid_top_with_margin_on(MASTER, PAD)
        .align_text_left()
        .line_spacing(5.0)
        .set(FPS_COUNTER, ui);
}
