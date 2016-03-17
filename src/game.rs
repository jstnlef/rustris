use piston_window::*;
use piston_window::types::Color;

use board::Board;
use tetromino::*;
use settings::*;

enum GameState {
    Playing,
    Paused
}

pub trait Game {
    fn on_input(&mut self, input: Input);
    fn on_update(&mut self, update_args: UpdateArgs);
    fn on_render(&mut self, render_args: RenderArgs, e: PistonWindow);
}

pub trait Renderer {
    fn render(&self, context: Context, graphics: &mut G2d);
}

pub trait GridRenderer {
    fn render(&self, x: i32, y: i32, context: Context, graphics: &mut G2d);
}

pub struct Rustris {
    board: Board,
    current_piece: Piece,
    next_piece: Piece,
    score: u32,
    level: u32,
    time_since_moved: f64
    // state: GameState
}
impl Rustris {
    pub fn new() -> Rustris {
        Rustris {
            board: Board::new(),
            current_piece: create_random_piece(),
            next_piece: create_random_piece(),
            score: 0,
            level: 1,
            time_since_moved: 0.0
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn set_current_piece(&mut self, piece: Piece) {
        if self.is_valid_board_position(&piece) {
            self.current_piece = piece;
        }
    }

    fn is_valid_board_position(&self, piece: &Piece) -> bool {
        piece.blocks_iter().all(|block| {
            (block.x >= 0 && block.x < WIDTH_IN_BLOCKS as i32 &&
             block.y >= 0 && block.y < HEIGHT_IN_BLOCKS as i32)
        })
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
        if self.time_since_moved >= 1.0 {
            let moved = self.current_piece.moved(Direction::Down);
            self.set_current_piece(moved);
            self.time_since_moved -= 1.0;
        }
    }

    fn on_render(&mut self, render_args: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            self.board.render(c, g);
            self.current_piece.render(c, g);
        });
    }
}

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




// pub struct TheGame {
//     rotation: f64,
//     x: f64,
//     y: f64,
//     up_d: bool, down_d: bool, left_d: bool, right_d: bool
// }
// impl TheGame {
//     pub fn new() -> TheGame {
//         TheGame { rotation : 0.0, x : 0.0, y : 0.0, up_d: false, down_d: false, left_d: false, right_d: false }
//     }
// }
// impl Game for TheGame {
//     fn on_update(&mut self, update_args: UpdateArgs) {
//         self.rotation += 3.0 * update_args.dt;
//         if self.up_d {
//             self.y += (-50.0) * update_args.dt;
//         }
//         if self.down_d {
//             self.y += (50.0) * update_args.dt;
//         }
//         if self.left_d {
//             self.x += (-50.0) * update_args.dt;
//         }
//         if self.right_d {
//             self.x += (50.0) * update_args.dt;
//         }
//     }
//     fn on_render(&mut self, render_args: RenderArgs, e: PistonWindow) {
//         e.draw_2d(|c, g| {
//             clear([0.0, 0.0, 0.0, 1.0], g);
//             let center = c.transform.trans((render_args.width / 2) as f64, (render_args.height / 2) as f64);
//             let square = rectangle::square(0.0, 0.0, 100.0);
//             let red = [1.0, 0.0, 0.0, 1.0];
//             rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g);
//         });
//     }
//     fn on_input(&mut self, inp: Input) {
//         match inp {
//             Input::Press(but) => {
//                 match but {
//                     Button::Keyboard(Key::Up) => {
//                         self.up_d = true;
//                     }
//                     Button::Keyboard(Key::Down) => {
//                         self.down_d = true;
//                     }
//                     Button::Keyboard(Key::Left) => {
//                         self.left_d = true;
//                     }
//                     Button::Keyboard(Key::Right) => {
//                         self.right_d = true;
//                     }
//                     _ => {}
//                 }
//             }
//             Input::Release(but) => {
//                 match but {
//                     Button::Keyboard(Key::Up) => {
//                         self.up_d = false;
//                     }
//                     Button::Keyboard(Key::Down) => {
//                         self.down_d = false;
//                     }
//                     Button::Keyboard(Key::Left) => {
//                         self.left_d = false;
//                     }
//                     Button::Keyboard(Key::Right) => {
//                         self.right_d = false;
//                     }
//                     _ => {}
//                 }
//             }
//             _ => {}
//         }
//     }
// }
