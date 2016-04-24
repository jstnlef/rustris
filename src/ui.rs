use find_folder;
use conrod::{
    Button, Canvas, Colorable, Frameable, Positionable, Labelable, Sizeable, Theme,
    Ui, UiCell, Text, Widget, color
};
use piston_window::{G2d, Glyphs, Graphics, PistonWindow};

use game::{Rustris, GameState};
use stats::GameStats;
use settings::*;


pub type Backend = (<G2d<'static> as Graphics>::Texture, Glyphs);
pub type UI = Ui<Backend>;
pub type UICell<'a> = UiCell<'a, Backend>;


pub fn create_ui(window: &PistonWindow) -> UI {
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
    UI::new(glyph_cache.unwrap(), theme)
}

pub fn set_ui(ref mut ui: UICell, game: &mut Rustris) {
    Canvas::new().flow_right(&[
        (LEFT_COLUMN, Canvas::new().color(color::DARK_CHARCOAL).pad(20.0)),
        (MIDDLE_COLUMN, Canvas::new().color(color::TRANSPARENT).length(300.0)),
        (RIGHT_COLUMN, Canvas::new().color(color::DARK_CHARCOAL).pad(20.0)),
    ]).set(MASTER, ui);
    set_scoreboard(ui, game.get_game_stats());
    set_next_piece(ui);

    if game.is_paused() {
        set_pause_menu(ui, game);
    } else if game.is_game_over() {
        set_game_over_menu(ui, game);
    }

}

fn set_pause_menu(ui: &mut UICell, game: &mut Rustris) {
    Canvas::new()
      .w_h(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64)
      .floating(true)
      .middle_of(MASTER)
      .rgba(0.0, 0.0, 0.0, 0.3)
      .set(PAUSE_OVERLAY, ui);

    Canvas::new().flow_down(&[
        (RESUME_CANVAS, Canvas::new())
    ]).label("Paused")
      .label_color(color::WHITE)
      .w_h(200.0, 200.0)
      .frame(1.0)
      .frame_color(color::WHITE)
      .pad(1.0)
      .middle_of(PAUSE_OVERLAY)
      .set(PAUSE_MENU, ui);

    Button::new()
        .label("Resume")
        .label_color(color::WHITE)
        .color(color::CHARCOAL)
        .middle_of(RESUME_CANVAS)
        .w_h(150.0, 30.0)
        .react(|| {
            game.set_game_state(GameState::Playing);
        })
        .set(RESUME_BUTTON, ui);
}

fn set_game_over_menu(ui: &mut UICell, game: &mut Rustris) {
    Canvas::new()
      .w_h(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64)
      .floating(true)
      .middle_of(MASTER)
      .rgba(0.0, 0.0, 0.0, 0.3)
      .set(GAME_OVER_OVERLAY, ui);

    Canvas::new().flow_down(&[
        (FINAL_SCORE_CANVAS, Canvas::new()),
        (RESTART_CANVAS, Canvas::new())
    ]).label("Game Over")
      .label_color(color::WHITE)
      .w_h(200.0, 200.0)
      .frame(1.0)
      .frame_color(color::WHITE)
      .pad(1.0)
      .middle_of(GAME_OVER_OVERLAY)
      .set(GAME_OVER_MENU, ui);

    {
        let stats = game.get_game_stats();
        Text::new(
            &format!("Final Score: {}", stats.get_score().to_string()))
            .color(color::WHITE)
            .middle_of(FINAL_SCORE_CANVAS)
            .set(FINAL_SCORE_TEXT, ui);
    }

    Button::new()
        .label("New Game?")
        .label_color(color::WHITE)
        .color(color::CHARCOAL)
        .middle_of(RESTART_CANVAS)
        .w_h(150.0, 30.0)
        .react(|| {
            game.reset();
        })
        .set(NEW_GAME_BUTTON, ui);
}

fn set_scoreboard(ui: &mut UICell, stats: &GameStats) {
    Canvas::new().flow_down(&[
        (SCORE_CANVAS, Canvas::new().label("Score").label_color(color::WHITE)),
        (LEVEL_CANVAS, Canvas::new().label("Level").label_color(color::WHITE)),
        (LINES_CANVAS, Canvas::new().label("Lines").label_color(color::WHITE))
    ]).w_h(150.0, 250.0)
      .frame(1.0)
      .frame_color(color::WHITE)
      .pad(1.0)
      .mid_bottom_of(LEFT_COLUMN)
      .set(SCOREBOARD, ui);

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

fn set_next_piece(ui: &mut UICell) {
    Canvas::new()
        .label("Next Piece")
        .label_color(color::WHITE)
        .w_h(NEXT_PIECE_WIDTH, NEXT_PIECE_HEIGHT)
        .frame(1.0)
        .frame_color(color::WHITE)
        .pad(1.0)
        .mid_top_of(RIGHT_COLUMN)
        .set(NEXT_PIECE, ui);
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
    LINES,

    // Pause Menu
    PAUSE_OVERLAY,
    PAUSE_MENU,
    RESUME_CANVAS,
    RESUME_BUTTON,

    // Game Over Menu
    GAME_OVER_OVERLAY,
    GAME_OVER_MENU,
    NEW_GAME_BUTTON,
    RESTART_CANVAS,
    FINAL_SCORE_CANVAS,
    FINAL_SCORE_TEXT,

    // Next Piece IDs
    NEXT_PIECE
}
