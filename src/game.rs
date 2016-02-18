use piston_window::*;
use piston_window::grid::Grid;

use colors::GREY;
use tetromino::*;
use settings::*;

enum GameState {
    Playing,
    Paused
}

#[derive(Clone, Copy)]
enum CellState {
    Empty,
    Block(types::Color)
}
impl CellState {
    fn draw(&self, x: u32, y: u32, context: Context, graphics: &mut G2d) {
        match *self {
            CellState::Block(color) => {
                let square = rectangle::square(
                    0.0, 0.0, BLOCK_SIZE - (2.0 * GRID_LINE_WIDTH)
                );
                let transform = context.transform
                    // Move the origin of the transform based on where the grid starts
                    .trans(GRID_X_OFFSET, GRID_Y_OFFSET)
                    // Move the square into the first grid cell (off of the bounding box)
                    .trans(GRID_LINE_WIDTH, GRID_LINE_WIDTH)
                    // Move the square into the proper grid cell based on x and y
                    .trans((x as f64)*BLOCK_SIZE, (y as f64)*BLOCK_SIZE);
                rectangle(color, square, transform, graphics);
            },
            CellState::Empty => {}
        }
    }
}

pub trait Game {
    fn on_input(&mut self, input: Input);
    fn on_update(&mut self, update_args: UpdateArgs);
    fn on_render(&mut self, render_args: RenderArgs, e: PistonWindow);
}

pub trait Renderer {
    fn render(&self, context: Context, graphics: &mut G2d);
}

// TODO: Move this to its own module
pub struct Board {
    grid: [[CellState; HEIGHT_IN_BLOCKS as usize]; WIDTH_IN_BLOCKS as usize]
}
impl Board {
    pub fn new() -> Board {
        // TODO: Get rid of this the hacky CellState stuff I put in
        // for testing
        use colors::*;
        let mut board = Board {
            grid: [[CellState::Empty; HEIGHT_IN_BLOCKS as usize]; WIDTH_IN_BLOCKS as usize]
        };
        board.grid[8][1] = CellState::Block(RED);
        board.grid[3][4] = CellState::Block(CYAN);
        board
    }
    fn get_cell_state(&self, x: u32, y: u32) -> CellState {
        self.grid[x as usize][y as usize]
    }
}
impl Renderer for Board {
    fn render(&self, context: Context, graphics: &mut G2d) {
        let grid = Grid {
            cols: WIDTH_IN_BLOCKS,
            rows: HEIGHT_IN_BLOCKS,
            units: BLOCK_SIZE
        };
        let line = Line::new(GREY, GRID_LINE_WIDTH);
        let transform = context.transform.trans(GRID_X_OFFSET, GRID_Y_OFFSET);
        grid.draw(&line, &Default::default(), transform, graphics);
        for (x, y) in grid.cells() {
            // TODO: Figure out why this only goes up to 9, 9...
            self.get_cell_state(x, y).draw(x, y, context, graphics);
        }
    }
}

pub struct Rustris {
    board: Board,
    current_piece: Piece,
    // next_piece: Piece,
    // score: u32,
    // level: u32,
    // state: GameState
}
impl Rustris {
    pub fn new() -> Rustris {
        Rustris {
            board: Board::new(),
            current_piece: Piece::create(J)
        }
    }
    pub fn reset(&mut self) {

    }
}
impl Game for Rustris {
    fn on_input(&mut self, input: Input) {

    }
    fn on_update(&mut self, update_args: UpdateArgs) {

    }
    fn on_render(&mut self, render_args: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            self.board.render(c, g);
        });
    }
}



pub struct TheGame {
    rotation: f64,
    x: f64,
    y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}
impl TheGame {
    pub fn new() -> TheGame {
        TheGame { rotation : 0.0, x : 0.0, y : 0.0, up_d: false, down_d: false, left_d: false, right_d: false }
    }
}
impl Game for TheGame {
    fn on_update(&mut self, update_args: UpdateArgs) {
        self.rotation += 3.0 * update_args.dt;
        if self.up_d {
            self.y += (-50.0) * update_args.dt;
        }
        if self.down_d {
            self.y += (50.0) * update_args.dt;
        }
        if self.left_d {
            self.x += (-50.0) * update_args.dt;
        }
        if self.right_d {
            self.x += (50.0) * update_args.dt;
        }
    }
    fn on_render(&mut self, render_args: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((render_args.width / 2) as f64, (render_args.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }
    fn on_input(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = true;
                    }
                    _ => {}
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
