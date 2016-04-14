
use std::collections::VecDeque;

use piston_window::{Context, G2d, Line, Transformed, types, Rectangle};
use piston_window::rectangle;
use piston_window::grid::Grid;

use colors::GREY;
use rendering::render_block_in_grid;
use tetromino::{Piece, Block};
use settings::*;

type GridRow = [CellState; WIDTH_IN_BLOCKS as usize];

pub struct Board {
    grid: VecDeque<GridRow>
}
impl Board {
    pub fn new() -> Board {
        Board {
            grid: Self::create_empty_grid()
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
    pub fn remove_completed_rows(&mut self) -> u32 {
        let completed_row_indexes = self.find_completed_row_indexes();
        for i in &completed_row_indexes {
            self.grid.remove(*i);
        }
        for _ in &completed_row_indexes {
            self.grid.push_front(Self::create_empty_row());
        }
        debug_assert!(self.grid.len() == HEIGHT_IN_BLOCKS as usize);
        completed_row_indexes.len() as u32
    }

    // Finds and returns the indexes of completed rows in reverse order
    fn find_completed_row_indexes(&self) -> Vec<usize> {
        let mut completed_row_indexes = Vec::new();
        let rows = self.grid.iter().rev().take_while(|&row| !Self::row_is_empty(row));
        for (i, row) in rows.enumerate() {
            if Self::row_is_complete(row) {
                let grid_index = (HEIGHT_IN_BLOCKS - 1) as usize - i;
                completed_row_indexes.push(grid_index);
            }
        }
        completed_row_indexes
    }

    fn get_cell_state(&self, x: i32, y: i32) -> CellState {
        self.grid[y as usize][x as usize]
    }

    fn set_cell_state(&mut self, block: Block, cell_state: CellState) {
        self.grid[block.y as usize][block.x as usize] = cell_state;
    }

    fn create_empty_grid() -> VecDeque<GridRow> {
        let mut grid = VecDeque::with_capacity(HEIGHT_IN_BLOCKS as usize);
        for _ in 0..HEIGHT_IN_BLOCKS {
            grid.push_back(Self::create_empty_row());
        }
        grid
    }

    fn create_empty_row() -> GridRow {
        [CellState::Empty; WIDTH_IN_BLOCKS as usize]
    }

    fn row_is_empty(row: &GridRow) -> bool {
        row.iter().all(|&block| block == CellState::Empty)
    }

    fn row_is_complete(row: &GridRow) -> bool {
        row.iter().all(|&block| block != CellState::Empty)
    }

    pub fn render(&self, context: Context, graphics: &mut G2d) {
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
impl CellState {
    fn render(&self, x: i32, y: i32, context: Context, graphics: &mut G2d) {
        match *self {
            CellState::Block(color) => {
                let rect = Rectangle {
                    color: color,
                    shape: rectangle::Shape::Square,
                    border: None
                };
                Block::new(x, y).render_in_grid(rect, context, graphics);
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
    use settings::*;

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

    #[test]
    fn test_row_is_empty() {
        let mut empty_row = [CellState::Empty; WIDTH_IN_BLOCKS as usize];
        assert!(Board::row_is_empty(&empty_row));
        empty_row[1] = CellState::Block(RED);
        assert!(!Board::row_is_empty(&empty_row));
    }

    #[test]
    fn test_row_is_complete() {
        let mut complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        assert!(Board::row_is_complete(&complete_row));
        complete_row[1] = CellState::Empty;
        assert!(!Board::row_is_complete(&complete_row));
    }

    #[test]
    fn test_create_empty_row() {
        let row = Board::create_empty_row();
        assert!(Board::row_is_empty(&row));
    }

    #[test]
    fn test_find_completed_row_indexes_simple() {
        let i = (HEIGHT_IN_BLOCKS - 1) as usize;
        let mut board = Board::new();
        let complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        board.grid[i] = complete_row;
        let result = board.find_completed_row_indexes();
        let expected = vec![i];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_completed_row_indexes_multiple() {
        let i = (HEIGHT_IN_BLOCKS - 1) as usize;
        let mut board = Board::new();
        let complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        board.grid[i] = complete_row;
        board.grid[i - 1] = complete_row;
        let result = board.find_completed_row_indexes();
        let expected = vec![i, i - 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_completed_row_indexes_skip_row() {
        let i = (HEIGHT_IN_BLOCKS - 1) as usize;
        let mut board = Board::new();
        let complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        let mut incomplete_row = complete_row;
        incomplete_row[4] = CellState::Empty;
        board.grid[i] = complete_row;
        board.grid[i - 1] = incomplete_row;
        board.grid[i - 2] = complete_row;
        let result = board.find_completed_row_indexes();
        let expected = vec![i, i - 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_completed_rows_simple() {
        let i = (HEIGHT_IN_BLOCKS - 1) as usize;
        let mut board = Board::new();
        let complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        board.grid[i] = complete_row;
        let n = board.remove_completed_rows();

        assert_eq!(n, 1);
        assert_eq!(board.grid, Board::create_empty_grid());
    }

    #[test]
    fn test_remove_completed_rows_moves_down() {
        let i = (HEIGHT_IN_BLOCKS - 1) as usize;
        let mut board = Board::new();
        let complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        board.grid[i] = complete_row;
        board.grid[i-1][0] = CellState::Block(RED);
        let n = board.remove_completed_rows();

        let mut expected_grid = Board::create_empty_grid();
        expected_grid[i][0] = CellState::Block(RED);

        assert_eq!(n, 1);
        assert_eq!(board.grid, expected_grid);
    }
    #[test]
    fn test_remove_completed_rows_moves_two_down() {
        let i = (HEIGHT_IN_BLOCKS - 1) as usize;
        let mut board = Board::new();
        let complete_row = [CellState::Block(RED); WIDTH_IN_BLOCKS as usize];
        board.grid[i] = complete_row;
        board.grid[i-1][0] = CellState::Block(RED);
        board.grid[i-2] = complete_row;
        board.grid[i-3][1] = CellState::Block(RED);
        let n = board.remove_completed_rows();

        let mut expected_grid = Board::create_empty_grid();
        expected_grid[i][0] = CellState::Block(RED);
        expected_grid[i-1][1] = CellState::Block(RED);

        assert_eq!(n, 2);
        assert_eq!(board.grid, expected_grid);
    }
}
