use piston_window::*;

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

pub struct Rustris {
    board: Board,
    randomizer: Randomizer,
    current_piece: Piece,
    next_piece: Piece,
    stats: GameStats,
    time_since_moved: f64
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
            time_since_moved: 0.0
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn get_game_stats(&self) -> &GameStats {
        &self.stats
    }

    pub fn set_current_piece(&mut self, piece: Piece) {
        self.current_piece = piece;
    }

    fn iteration_delay(&self) -> f64 {
        (11 - self.stats.get_level()) as f64 * 0.10
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
        self.set_current_piece(next);
        self.next_piece = self.randomizer.create_piece();
    }

    fn update(&mut self) {
        let moved = self.current_piece.moved(Direction::Down);
        if !self.is_valid_board_position(&moved) {
            self.lock_current_piece();
        } else {
            self.set_current_piece(moved);
        }
    }
}
impl Game for Rustris {
    fn on_input(&mut self, input: Input) {
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

    fn on_update(&mut self, update_args: UpdateArgs) {
        self.time_since_moved += update_args.dt;
        let delay = self.iteration_delay();
        if self.time_since_moved >= delay {
            self.time_since_moved -= delay;
            self.update();
        }
    }

    fn render(&mut self, context: Context, graphics: &mut G2d) {
        self.board.render(context, graphics);
        self.calculate_ghost_piece().render_ghost(context, graphics);
        self.current_piece.render_normal(context, graphics);
    }
}

// TODO: Move this into its own module
pub fn render_square_in_grid(x: i32, y: i32, rect: Rectangle, context: Context, graphics: &mut G2d) {
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
    rect.draw(square, &Default::default(), transform, graphics);
}
