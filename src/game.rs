use piston_window::*;

use board::Board;
use randomizer::Randomizer;
use tetromino::*;
use settings::*;
use stats::GameStats;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver
}

pub trait Game {
    fn on_input(&mut self, input: Input);
    fn on_update(&mut self, update_args: UpdateArgs);
    fn render(&mut self, context: Context, graphics: &mut G2d);
}

pub struct Rustris {
    board: Board,
    randomizer: Randomizer,
    current_piece: Piece,
    next_piece: Piece,
    stats: GameStats,
    time_since_moved: f64,
    state: GameState
}
impl Rustris {
    pub fn new() -> Rustris {
        let mut randomizer = Randomizer::new();
        let current_piece = randomizer.create_piece();
        let next_piece = randomizer.create_piece();
        Rustris {
            board: Board::new(),
            randomizer: randomizer,
            current_piece: current_piece,
            next_piece: next_piece,
            stats: GameStats::new(),
            time_since_moved: 0.0,
            state: GameState::Playing
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        let mut randomizer = Randomizer::new();
        self.current_piece = randomizer.create_piece();
        self.next_piece = randomizer.create_piece();
        self.randomizer = randomizer;
        self.stats = GameStats::new();
        self.time_since_moved = 0.0;
        self.set_game_state(GameState::Playing);
    }

    pub fn get_game_stats(&self) -> &GameStats {
        &self.stats
    }

    pub fn set_current_piece(&mut self, piece: Piece) {
        self.current_piece = piece;
    }

    pub fn is_playing(&self) -> bool {
        self.state == GameState::Playing
    }

    pub fn is_paused(&self) -> bool {
        self.state == GameState::Paused
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.state = state;
    }

    fn drop_delay(&self) -> f64 {
        ((MAX_GAME_LEVEL + 1) - self.stats.get_level()) as f64 * 0.10
    }

    fn is_valid_board_position(&self, piece: &Piece) -> bool {
        // TODO: Add tests for this
        piece.blocks_iter().all(|block| {
            (block.x >= 0 && block.x < WIDTH_IN_BLOCKS &&
             block.y >= 0 && block.y < HEIGHT_IN_BLOCKS &&
             !self.board.is_space_occupied(block))
        })
    }

    fn lock_current_piece(&mut self) {
        self.board.set_piece(&self.current_piece);
        self.remove_completed_lines();
        self.get_new_piece();
    }

    fn remove_completed_lines(&mut self) {
        let number_removed = self.board.remove_completed_rows();
        self.stats.score_completed_lines(number_removed);
    }

    fn calculate_ghost_piece(&self) -> Piece {
        let mut ghost = self.current_piece;
        while self.is_valid_board_position(&ghost) {
            ghost.y += 1;
        }
        ghost.y -= 1;
        ghost
    }

    fn get_new_piece(&mut self) {
        let next = self.next_piece;
        if self.is_valid_board_position(&next){
            self.set_current_piece(next);
            self.next_piece = self.randomizer.create_piece();
        } else {
            self.state = GameState::GameOver;
        }
    }

    fn update(&mut self) {
        let moved = self.current_piece.moved(Direction::Down);
        if !self.is_valid_board_position(&moved) {
            self.lock_current_piece();
        } else {
            self.set_current_piece(moved);
        }
    }

    fn handle_playing_input(&mut self, input: Input) {
        let mut moved: Option<Piece> = None;
        match input {
            Input::Press(key) => {
                match key {
                    Button::Keyboard(Key::Up) => {
                        moved = Some(self.current_piece.rotated());
                    }
                    Button::Keyboard(Key::Down) => {
                        let move_down = self.current_piece.moved(Direction::Down);
                        if self.is_valid_board_position(&move_down) {
                            self.time_since_moved = 0.0;
                            self.stats.score_soft_drop();
                            self.set_current_piece(move_down);
                        }
                    }
                    Button::Keyboard(Key::Left) => {
                        moved = Some(self.current_piece.moved(Direction::Left));
                    }
                    Button::Keyboard(Key::Right) => {
                        moved = Some(self.current_piece.moved(Direction::Right));
                    }
                    Button::Keyboard(Key::Space) => {
                        let ghost = self.calculate_ghost_piece();
                        let rows_dropped = (ghost.y - self.current_piece.y) as u32;
                        self.stats.score_hard_drop(rows_dropped);
                        self.set_current_piece(ghost);
                        self.lock_current_piece();
                    }
                    Button::Keyboard(Key::P) => {
                        self.state = GameState::Paused
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if let Some(piece) = moved {
            if self.is_valid_board_position(&piece) {
                self.set_current_piece(piece);
            }
        }
    }

    fn handle_paused_input(&mut self, input: Input) {
        match input {
            Input::Press(key) => {
                match key {
                    Button::Keyboard(Key::P) => {
                        self.state = GameState::Playing
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
impl Game for Rustris {
    fn on_input(&mut self, input: Input) {
        match self.state {
            GameState::Playing => self.handle_playing_input(input),
            GameState::Paused => self.handle_paused_input(input),
            _ => {}
        }
    }

    fn on_update(&mut self, update_args: UpdateArgs) {
        match self.state {
            GameState::Playing => {
                self.time_since_moved += update_args.dt;
                let delay = self.drop_delay();
                if self.time_since_moved >= delay {
                    self.time_since_moved -= delay;
                    self.update();
                }
            },
            _ => {}
        }
    }

    fn render(&mut self, context: Context, graphics: &mut G2d) {
        self.board.render(context, graphics);
        self.calculate_ghost_piece().render_in_grid(RenderType::Ghost, context, graphics);
        self.current_piece.render_in_grid(RenderType::Normal, context, graphics);
        self.next_piece.render_in_next_piece(RenderType::Normal, context, graphics);
    }
}

#[derive(Clone, Copy)]
pub struct ScreenPosition {
    pub x: f64,
    pub y: f64
}
impl ScreenPosition {
    pub fn new(x: f64, y: f64) -> ScreenPosition {
        ScreenPosition {x: x, y: y}
    }
}
