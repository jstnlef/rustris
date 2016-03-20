pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const HEIGHT_IN_BLOCKS: i32 = 20;
pub const WIDTH_IN_BLOCKS: i32 = 10;
pub const BLOCK_SIZE: f64 = 27.0;
pub const GRID_LINE_WIDTH: f64 = 1.0;
pub const GRID_X_OFFSET: f64 = (
    WINDOW_WIDTH as f64 / 2.0 - (WIDTH_IN_BLOCKS as f64/2.0 * BLOCK_SIZE)
);
pub const GRID_Y_OFFSET: f64 = 25.0;
