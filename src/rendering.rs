use piston_window::{Context, G2d, Rectangle, Transformed, rectangle};

use settings::*;
use tetromino::Block;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64
}
impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position {x: x, y: y}
    }
}

pub fn render_block_in_grid(block: Block, rect: Rectangle, context: Context, graphics: &mut G2d) {
    let grid_position = Position::new(
        GRID_X_OFFSET + GRID_LINE_WIDTH,
        GRID_Y_OFFSET + GRID_LINE_WIDTH
    );
    render_block(grid_position, block, rect, context, graphics);
}

pub fn render_block(screen_position: Position,
                    block: Block,
                    rect: Rectangle,
                    context: Context,
                    graphics: &mut G2d) {
    let square = rectangle::square(
        screen_position.x, screen_position.y, BLOCK_SIZE - (2.0 * GRID_LINE_WIDTH)
    );
    let transform = context.transform.trans(
        (block.x as f64)*BLOCK_SIZE,
        (block.y as f64)*BLOCK_SIZE
    );
    rect.draw(square, &Default::default(), transform, graphics);
}
