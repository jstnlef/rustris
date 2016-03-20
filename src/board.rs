
use piston_window::{Context, G2d, Line, Transformed, types};
use piston_window::grid::Grid;

use colors::GREY;
use game::{Renderer, GridRenderer, render_square_in_grid};
use tetromino::{Piece, Block};
use settings::*;

pub struct Board {
    grid: [[CellState; WIDTH_IN_BLOCKS as usize]; HEIGHT_IN_BLOCKS as usize]
}
impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[CellState::Empty; WIDTH_IN_BLOCKS as usize]; HEIGHT_IN_BLOCKS as usize]
        }
    }

    pub fn set_piece(&mut self, piece: &Piece) {
        for block in piece.blocks_iter() {
            self.set_cell_state(block, CellState::Block(piece.get_color()));
        }
    }

    pub fn is_space_occupied(&self, block: Block) -> bool {
        match self.get_cell_state(block.x, block.y) {
            CellState::Block(_) => true,
            CellState::Empty => false
        }
    }

    // Removes completed rows from the board and returns the number
    // of rows completed.
    pub fn remove_completed_rows(&mut self) -> i32 {
        0
    }

    fn get_cell_state(&self, x: i32, y: i32) -> CellState {
        self.grid[y as usize][x as usize]
    }

    fn set_cell_state(&mut self, block: Block, cell_state: CellState) {
        self.grid[block.y as usize][block.x as usize] = cell_state;
    }
}
impl Renderer for Board {
    fn render(&self, context: Context, graphics: &mut G2d) {
        let grid = Grid {
            cols: WIDTH_IN_BLOCKS as u32,
            rows: HEIGHT_IN_BLOCKS as u32,
            units: BLOCK_SIZE
        };
        let line = Line::new(GREY, GRID_LINE_WIDTH);
        let transform = context.transform.trans(GRID_X_OFFSET, GRID_Y_OFFSET);
        grid.draw(&line, &Default::default(), transform, graphics);
        for x in 0..WIDTH_IN_BLOCKS {
            for y in 0..HEIGHT_IN_BLOCKS {
                self.get_cell_state(x, y).render(x, y, context, graphics);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Empty,
    Block(types::Color)
}
impl GridRenderer for CellState {
    fn render(&self, x: i32, y: i32, context: Context, graphics: &mut G2d) {
        match *self {
            CellState::Block(color) => {
                render_square_in_grid(x, y, color, context, graphics);
            },
            CellState::Empty => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use colors::{RED, CYAN};
    use tetromino::{Block, Piece, I};

    #[test]
    fn test_set_piece() {
        let mut board = Board::new();
        let piece = Piece::create(&I);
        board.set_piece(&piece);
        assert_eq!(board.get_cell_state(2, 1), CellState::Empty);
        assert_eq!(board.get_cell_state(3, 1), CellState::Block(CYAN));
        assert_eq!(board.get_cell_state(4, 1), CellState::Block(CYAN));
        assert_eq!(board.get_cell_state(5, 1), CellState::Block(CYAN));
        assert_eq!(board.get_cell_state(6, 1), CellState::Block(CYAN));
        assert_eq!(board.get_cell_state(7, 1), CellState::Empty);
    }

    #[test]
    fn test_is_space_occupied() {
        let mut board = Board::new();
        let block = Block{x: 2, y: 2};
        board.set_cell_state(block, CellState::Block(RED));
        assert!(board.is_space_occupied(block));
        assert!(!board.is_space_occupied(Block{x: 0, y: 0}));
    }

    #[test]
    fn test_set_cell_state() {
        let mut board = Board::new();
        assert_eq!(board.get_cell_state(0, 0), CellState::Empty);
        board.set_cell_state(Block{x: 2, y: 2}, CellState::Block(RED));
        assert_eq!(board.get_cell_state(2, 2), CellState::Block(RED));
    }
}
