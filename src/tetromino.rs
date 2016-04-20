use piston_window::{Context, G2d, Rectangle, Transformed, color, rectangle};
use piston_window::types::Color;

use colors::*;
use game::ScreenPosition;
use settings::*;


pub static I: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:1}, Block{x:1, y:1}, Block{x:2, y:1}, Block{x:3, y:1}],
        [Block{x:2, y:0}, Block{x:2, y:1}, Block{x:2, y:2}, Block{x:2, y:3}],
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:2, y:2}, Block{x:3, y:2}],
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:1, y:2}, Block{x:1, y:3}]
    ],
    color: CYAN
};

pub static J: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:0}, Block{x:0, y:1}, Block{x:1, y:1}, Block{x:2, y:1}],
        [Block{x:2, y:0}, Block{x:1, y:0}, Block{x:1, y:1}, Block{x:1, y:2}],
        [Block{x:2, y:2}, Block{x:2, y:1}, Block{x:1, y:1}, Block{x:0, y:1}],
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:1, y:0}]
    ],
    color: BLUE
};

pub static L: Tetromino = Tetromino {
    configurations: [
        [Block{x:2, y:0}, Block{x:2, y:1}, Block{x:1, y:1}, Block{x:0, y:1}],
        [Block{x:2, y:2}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:1, y:0}],
        [Block{x:0, y:2}, Block{x:0, y:1}, Block{x:1, y:1}, Block{x:2, y:1}],
        [Block{x:0, y:0}, Block{x:1, y:0}, Block{x:1, y:1}, Block{x:1, y:2}]
    ],
    color: ORANGE
};

pub static O: Tetromino = Tetromino {
    configurations: [
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:2, y:0}, Block{x:2, y:1}],
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:2, y:0}, Block{x:2, y:1}],
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:2, y:0}, Block{x:2, y:1}],
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:2, y:0}, Block{x:2, y:1}]
    ],
    color: YELLOW
};

pub static S: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:1}, Block{x:1, y:1}, Block{x:1, y:0}, Block{x:2, y:0}],
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:2, y:1}, Block{x:2, y:2}],
        [Block{x:2, y:1}, Block{x:1, y:1}, Block{x:1, y:2}, Block{x:0, y:2}],
        [Block{x:1, y:2}, Block{x:1, y:1}, Block{x:0, y:1}, Block{x:0, y:0}]
    ],
    color: LIME
};

pub static T: Tetromino = Tetromino {
    configurations: [
        [Block{x:1, y:0}, Block{x:0, y:1}, Block{x:1, y:1}, Block{x:2, y:1}],
        [Block{x:2, y:1}, Block{x:1, y:0}, Block{x:1, y:1}, Block{x:1, y:2}],
        [Block{x:1, y:2}, Block{x:2, y:1}, Block{x:1, y:1}, Block{x:0, y:1}],
        [Block{x:0, y:1}, Block{x:1, y:0}, Block{x:1, y:1}, Block{x:1, y:2}]
    ],
    color: PURPLE
};

pub static Z: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:0}, Block{x:1, y:0}, Block{x:1, y:1}, Block{x:2, y:1}],
        [Block{x:2, y:0}, Block{x:2, y:1}, Block{x:1, y:1}, Block{x:1, y:2}],
        [Block{x:2, y:2}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:0, y:1}],
        [Block{x:0, y:2}, Block{x:0, y:1}, Block{x:1, y:1}, Block{x:1, y:0}]
    ],
    color: RED
};

pub type Configuration = [Block; 4];
type Rotation = usize;

#[derive(Clone, Copy)]
pub struct Piece {
    // TODO: Replace with position
    // TODO: Make this private again
    pub x: i32,
    pub y: i32,
    ptype: &'static Tetromino,
    rotation: Rotation
}
impl Piece {
    pub fn create(ptype: &'static Tetromino) -> Piece {
        let x = (WIDTH_IN_BLOCKS / 2) - 2;
        Piece::new(x, 0, ptype, 0)
    }

    fn new(x: i32, y: i32, ptype: &'static Tetromino, rotation: Rotation) -> Piece {
        Piece {
            x: x,
            y: y,
            ptype: ptype,
            rotation: rotation
        }
    }

    pub fn blocks_iter(&self) -> BlockIterator {
        let configuration = self.ptype.get_configuration(self.rotation);
        BlockIterator::new(self.x, self.y, configuration)
    }

    pub fn rotated(&self) -> Self {
        let new_rotation = (self.rotation + 1) % self.ptype.configurations.len();
        let mut rotated = Self::new(self.x, self.y, self.ptype, new_rotation);
        rotated.x -= rotated.wall_kick_translation();
        rotated
    }

    pub fn moved(&self, direction: Direction) -> Self {
        let (trans_x, trans_y) = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1)
        };
        Self::new(self.x + trans_x, self.y + trans_y, self.ptype, self.rotation)
    }

    fn wall_kick_translation(&self) -> i32 {
        let min_block = self.blocks_iter().min_by_key(|block| block.x).unwrap();
        let max_block = self.blocks_iter().max_by_key(|block| block.x).unwrap();
        if min_block.x < 0 {
            min_block.x
        } else if max_block.x >= WIDTH_IN_BLOCKS {
            max_block.x - WIDTH_IN_BLOCKS + 1
        } else {
            0
        }
    }

    pub fn get_color(&self) -> Color {
        self.ptype.color
    }

    pub fn render_in_grid(&self, render_type: RenderType, context: Context, graphics: &mut G2d) {
        let position = get_grid_position();
        self.render(position, render_type, context, graphics);
    }

    pub fn render_in_next_piece(&self, render_type: RenderType, context: Context,
                                graphics: &mut G2d) {
        let position = ScreenPosition::new(553.0, 80.0);
        self.render(position, render_type, context, graphics);
    }

    fn render(&self, position: ScreenPosition, render_type: RenderType, context: Context,
                  graphics: &mut G2d) {
        let rect = render_type.get_rectangle(self.get_color());
        for block in self.blocks_iter() {
            block.render(position, rect, context, graphics);
        }
    }
}

fn get_grid_position() -> ScreenPosition {
    ScreenPosition::new(
        GRID_X_OFFSET + GRID_LINE_WIDTH,
        GRID_Y_OFFSET + GRID_LINE_WIDTH
    )
}

pub enum RenderType {
    Normal,
    Ghost
}
impl RenderType {
    pub fn get_rectangle(&self, color: Color) -> Rectangle {
        match *self {
            RenderType::Ghost => Rectangle {
                color: color::BLACK,
                shape: rectangle::Shape::Square,
                border: Some(rectangle::Border {
                    color: color,
                    radius: GHOST_BORDER_WIDTH
                })
            },
            RenderType::Normal => Rectangle {
                color: color,
                shape: rectangle::Shape::Square,
                border: None
            }
        }
    }
}

pub struct BlockIterator {
    x: i32,
    y: i32,
    index: usize,
    blocks: &'static Configuration
}
impl BlockIterator {
    pub fn new(x: i32, y: i32, blocks: &'static Configuration) -> BlockIterator {
        BlockIterator {
            x: x,
            y: y,
            index: 0,
            blocks: blocks
        }
    }
}
impl Iterator for BlockIterator {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        if self.index >= self.blocks.len() {
            return None;
        }
        let ref block = self.blocks[self.index];
        self.index += 1;
        let translated_x = self.x + block.x;
        let translated_y = self.y + block.y;
        Some(Block{x: translated_x, y: translated_y})
    }
}

pub enum Direction {
    Left,
    Right,
    Down
}

#[derive(Debug, PartialEq)]
pub struct Tetromino {
    configurations: [Configuration; 4],
    color: Color
}
impl Tetromino {
    fn get_configuration(&self, rotation: Rotation) -> &Configuration {
        &self.configurations[rotation]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block {
    pub x: i32,
    pub y: i32
}
impl Block {
    pub fn new(x: i32, y: i32) -> Block {
        Block {x: x, y: y}
    }

    pub fn render_in_grid(&self, rect: Rectangle, context: Context, graphics: &mut G2d) {
        self.render(get_grid_position(), rect, context, graphics);
    }

    pub fn render(&self, position: ScreenPosition, rect: Rectangle, context: Context,
                  graphics: &mut G2d) {
        let square = rectangle::square(
            position.x, position.y, BLOCK_SIZE - (2.0 * GRID_LINE_WIDTH)
        );
        let transform = context.transform.trans(
            (self.x as f64) * BLOCK_SIZE,
            (self.y as f64) * BLOCK_SIZE
        );
        rect.draw(square, &Default::default(), transform, graphics);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_rotated() {
        let original = Piece::create(&I);
        assert_eq!(original.rotation, 0);
        let mut rotated = original.rotated();
        assert_eq!(rotated.rotation, 1);
        assert_eq!(original.x, rotated.x);
        rotated = rotated.rotated();
        assert_eq!(rotated.rotation, 2);
        assert_eq!(original.x, rotated.x);
        rotated = rotated.rotated();
        assert_eq!(rotated.rotation, 3);
        assert_eq!(original.x, rotated.x);
        rotated = rotated.rotated();
        assert_eq!(rotated.rotation, 0);
        assert_eq!(original.x, rotated.x);
    }

    #[test]
    fn test_piece_rotated_kicked() {
        let mut p = Piece::create(&I);
        p.x = 9;
        let rotated = p.rotated();
        assert_eq!(rotated.x, 7)
    }

    #[test]
    fn test_piece_moved_left() {
        let p = Piece::create(&Z);
        let result = p.moved(Direction::Left);
        assert_eq!(result.x, p.x - 1);
    }

    #[test]
    fn test_piece_moved_right() {
        let p = Piece::create(&Z);
        let result = p.moved(Direction::Right);
        assert_eq!(result.x, p.x + 1);
    }

    #[test]
    fn test_wall_kick_in_bounds() {
        let p = Piece::create(&I);
        let kicked_translation = p.wall_kick_translation();
        assert_eq!(kicked_translation, 0);
    }

    #[test]
    fn test_wall_kick_out_of_bounds_right() {
        let mut p = Piece::create(&I);
        p.x = 9;
        let kicked_translation = p.wall_kick_translation();
        assert_eq!(kicked_translation, 3);
    }

    #[test]
    fn test_wall_kick_out_of_bounds_left() {
        let mut p = Piece::create(&I);
        p.x = -2;
        let kicked_translation = p.wall_kick_translation();
        assert_eq!(kicked_translation, -2);
    }

    #[test]
    fn test_block_iterator() {
        let mut block_iter = BlockIterator::new(2, 2, &I.configurations[0]);
        assert_eq!(block_iter.next(), Some(Block{x: 2, y: 3}));
        assert_eq!(block_iter.next(), Some(Block{x: 3, y: 3}));
        assert_eq!(block_iter.next(), Some(Block{x: 4, y: 3}));
        assert_eq!(block_iter.next(), Some(Block{x: 5, y: 3}));
        assert_eq!(block_iter.next(), None);
    }
}
