use piston_window::*;
use piston_window::types::Color;

use board::Board;
use randomizer::Randomizer;
use tetromino::*;
use settings::*;
use stats::*;

enum GameState {
    Playing,
    Paused
}

pub trait Game {
    fn on_input(&mut self, input: Input);
    fn on_update(&mut self, update_args: UpdateArgs);
    fn render(&mut self, context: Context, graphics: &mut G2d);
}

pub trait Renderer {
    fn render(&self, context: Context, graphics: &mut G2d);
}

pub trait GridRenderer {
    fn render(&self, x: i32, y: i32, context: Context, graphics: &mut G2d);
}

pub struct Rustris {
    board: Board,
    randomizer: Randomizer,
    current_piece: Piece,
    next_piece: Piece,
    stats: GameStats,
    time_since_moved: f64,
    time_per_tick: f64
    // state: GameState
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
            time_per_tick: 1.0  //For testing
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn get_game_stats(&self) -> &GameStats {
        &self.stats
    }

    pub fn set_current_piece(&mut self, piece: Piece) {
        // TODO: Add tests for this
        if self.is_valid_board_position(&piece) {
            self.current_piece = piece;
        }
    }

    fn is_valid_board_position(&self, piece: &Piece) -> bool {
        // TODO: Add tests for this
        piece.blocks_iter().all(|block| {
            (block.x >= 0 && block.x < WIDTH_IN_BLOCKS &&
             block.y >= 0 && block.y < HEIGHT_IN_BLOCKS &&
             !self.board.is_space_occupied(block))
        })
    }

    fn should_lock(&self, piece: &Piece) -> bool {
        // TODO: Add tests for this
        piece.blocks_iter().any(|block| {
            block.y >= HEIGHT_IN_BLOCKS ||
            self.board.is_space_occupied(block)
        })
    }

    fn lock_current_piece(&mut self) {
        self.board.set_piece(&self.current_piece);
    }

    fn remove_completed_lines(&mut self) {
        let number_removed = self.board.remove_completed_rows();
        self.stats.score_completed_lines(number_removed);
    }

    fn get_new_piece(&mut self) {
        let next = self.next_piece;
        self.set_current_piece(next);
        self.next_piece = self.randomizer.create_piece();
    }

    fn update(&mut self) {
        let moved = self.current_piece.moved(Direction::Down);
        if self.should_lock(&moved) {
            self.lock_current_piece();
            self.remove_completed_lines();
            self.get_new_piece();
        } else {
            self.set_current_piece(moved);
        }
    }
}
impl Game for Rustris {
    fn on_input(&mut self, input: Input) {
        match input {
            Input::Press(key) => {
                match key {
                    Button::Keyboard(Key::Up) => {
                        let rotated = self.current_piece.rotated();
                        self.set_current_piece(rotated);
                    }
                    Button::Keyboard(Key::Down) => {
                        let moved = self.current_piece.moved(Direction::Down);
                        self.set_current_piece(moved);
                        self.time_since_moved = 0.0;
                        self.stats.score_soft_drop();
                    }
                    Button::Keyboard(Key::Left) => {
                        let moved = self.current_piece.moved(Direction::Left);
                        self.set_current_piece(moved);
                    }
                    Button::Keyboard(Key::Right) => {
                        let moved = self.current_piece.moved(Direction::Right);
                        self.set_current_piece(moved);
                    }
                    Button::Keyboard(Key::Space) => {
                        // TODO: Implement hard drop code
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn on_update(&mut self, update_args: UpdateArgs) {
        self.time_since_moved += update_args.dt;
        // TODO: The threshold needs to be able to be updated
        // as the game progresses
        if self.time_since_moved >= self.time_per_tick {
            self.time_since_moved -= self.time_per_tick;
            self.update();
        }
    }

    fn render(&mut self, context: Context, graphics: &mut G2d) {
        self.board.render(context, graphics);
        self.current_piece.render(context, graphics);
    }
}

// TODO: Move this into its own module
pub fn render_square_in_grid(x: i32, y: i32, color: Color, context: Context, graphics: &mut G2d) {
    let square = rectangle::square(
        0.0, 0.0, BLOCK_SIZE - (2.0 * GRID_LINE_WIDTH)
    );
    let transform = context.transform
        // Translate the origin of the transform based on where the grid starts
        .trans(GRID_X_OFFSET, GRID_Y_OFFSET)
        // Translate the square into the first grid cell (off of the bounding box)
        .trans(GRID_LINE_WIDTH, GRID_LINE_WIDTH)
        // Translate the square into the proper grid cell based on the grid x and y position
        .trans((x as f64)*BLOCK_SIZE, (y as f64)*BLOCK_SIZE);
    rectangle(color, square, transform, graphics);
}
