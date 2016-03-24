use find_folder;
use conrod::{
    Canvas, Colorable, Frameable, Positionable, Labelable, Sizeable, Theme, Ui, UiCell,
    Text, Widget, color
};
use piston_window::{G2d, Glyphs, Graphics, PistonWindow};

use game::Rustris;


pub type Backend = (<G2d<'static> as Graphics>::Texture, Glyphs);
pub type UI = Ui<Backend>;
pub type UICell<'a> = UiCell<'a, Backend>;


pub fn create_ui(window: &PistonWindow) -> UI {
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
    UI::new(glyph_cache.unwrap(), theme)
}

pub fn set_ui(ref mut ui: UICell, game: &Rustris) {
    Canvas::new().flow_right(&[
        (LEFT_COLUMN, Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
        (MIDDLE_COLUMN, Canvas::new().length(300.0)),
        (RIGHT_COLUMN, Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
    ]).set(MASTER, ui);

    // Scoreboard
    Canvas::new().flow_down(&[
        (SCORE_CANVAS, Canvas::new().label("Score").label_color(color::WHITE)),
        (LEVEL_CANVAS, Canvas::new().label("Level").label_color(color::WHITE)),
        (LINES_CANVAS, Canvas::new().label("Lines").label_color(color::WHITE)),
    ]).w_h(150.0, 250.0)
      .mid_bottom_of(LEFT_COLUMN)
      .set(SCOREBOARD, ui);

    let stats = &game.stats;
    Text::new(&stats.get_score().to_string())
        .color(color::WHITE)
        .middle_of(SCORE_CANVAS)
        .set(SCORE, ui);
    Text::new(&stats.get_level().to_string())
        .color(color::WHITE)
        .middle_of(LEVEL_CANVAS)
        .set(LEVEL, ui);
    Text::new(&stats.get_lines().to_string())
        .color(color::WHITE)
        .middle_of(LINES_CANVAS)
        .set(LINES, ui);
}

widget_ids! {
    // Canvas IDs
    MASTER,
    LEFT_COLUMN,
    MIDDLE_COLUMN,
    RIGHT_COLUMN,

    // Scoreboard IDs
    SCOREBOARD,
    SCORE_CANVAS,
    LEVEL_CANVAS,
    LINES_CANVAS,
    SCORE,
    LEVEL,
    LINES
}
