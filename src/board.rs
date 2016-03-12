
use piston_window::{Context, G2d, Line, rectangle, Transformed, types};
use piston_window::grid::Grid;
use piston_window::types::{Matrix2d};

use colors::GREY;
use game::{Renderer, GridRenderer, render_square_in_grid};
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

    fn get_cell_state(&self, x: u32, y: u32) -> &CellState {
        &self.grid[x as usize][y as usize]
    }

    fn set_cell_state(&mut self, x: u32, y: u32, cell_state: CellState) {
        self.grid[x as usize][y as usize] = cell_state;
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
        for x in 0..WIDTH_IN_BLOCKS {
            for y in 0..HEIGHT_IN_BLOCKS {
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
        match self {
            &CellState::Block(color) => {
                render_square_in_grid(x, y, color, context, graphics);
            },
            &CellState::Empty => {}
        }
    }
}
