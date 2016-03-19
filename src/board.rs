
use piston_window::{Context, G2d, Line, Transformed, types};
use piston_window::grid::Grid;

use colors::GREY;
use game::{Renderer, GridRenderer, render_square_in_grid};
use tetromino::{Piece, Block};
use settings::*;

pub struct Board {
    grid: [[CellState; HEIGHT_IN_BLOCKS as usize]; WIDTH_IN_BLOCKS as usize]
}
impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[CellState::Empty; HEIGHT_IN_BLOCKS as usize]; WIDTH_IN_BLOCKS as usize]
        }
    }

    pub fn set_piece(&mut self, piece: &Piece) {
        // TODO: Add tests for this
        for block in piece.blocks_iter() {
            self.set_cell_state(block, CellState::Block(piece.get_color()));
        }
    }

    pub fn is_space_occupied(&self, block: Block) -> bool {
        // TODO: Add tests for this
        match self.get_cell_state(block.x, block.y) {
            &CellState::Block(_) => true,
            &CellState::Empty => false
        }
    }

    fn get_cell_state(&self, x: i32, y: i32) -> &CellState {
        // TODO: Add tests for this
        &self.grid[x as usize][y as usize]
    }

    fn set_cell_state(&mut self, block: Block, cell_state: CellState) {
        // TODO: Add tests for this
        self.grid[block.x as usize][block.y as usize] = cell_state;
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
        for x in 0..WIDTH_IN_BLOCKS as i32 {
            for y in 0..HEIGHT_IN_BLOCKS as i32 {
                self.get_cell_state(x, y).render(x as i32, y as i32, context, graphics);
            }
        }
    }
}

#[derive(Clone, Copy)]
enum CellState {
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
