use piston_window::{Context, G2d};
use piston_window::types::Color;
use rand::{Rng, thread_rng};

use colors::*;
use game::{Renderer, render_square_in_grid};
use settings::WIDTH_IN_BLOCKS;


pub fn create_random_piece() -> Piece {
    let all_tetrominos = [&I, &J, &L, &O, &S, &T, &Z];
    let ptype = thread_rng().choose(&all_tetrominos).unwrap();
    Piece::create(*ptype)
}

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
        let x = (WIDTH_IN_BLOCKS as i32 / 2) - 2;
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

    pub fn get_blocks(&self) -> &Configuration {
        // TODO: Remove this in favor of blocks_iter
        self.ptype.get_configuration(self.rotation)
    }

    pub fn blocks_iter(&self) -> BlockIterator {
        let configuration = self.ptype.get_configuration(self.rotation);
        BlockIterator::new(self.x, self.y, configuration)
    }

    pub fn rotated(&self) -> Self {
        let rotation = (self.rotation + 1) % self.ptype.configurations.len();
        Self::new(self.x, self.y, self.ptype, rotation)
    }

    pub fn moved(&self, direction: Direction) -> Self {
        // TODO: Add direction Down and have it move here
        let translated_x = match direction {
            Direction::Left => self.x - 1,
            Direction::Right => self.x + 1
        };
        Self::new(translated_x, self.y, self.ptype, self.rotation)
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.blocks_iter().any(|block| {
            block.x < 0 || block.x >= WIDTH_IN_BLOCKS as i32
        })
    }

    pub fn wall_kick(&self) -> Self {
        // TODO: Refactor to use blocks_iter when test is written
        let mut translation = 0;
        let min_block = self.blocks_iter().min_by_key(|block| block.x).unwrap();
        let max_block = self.blocks_iter().max_by_key(|block| block.x).unwrap();
        if min_block.x < 0 {
            translation = min_block.x;
        } else if max_block.x >= WIDTH_IN_BLOCKS as i32 {
            translation = max_block.x - (WIDTH_IN_BLOCKS - 1) as i32;
        }
        Self::new(self.x - translation, self.y, self.ptype, self.rotation)
    }

    fn get_color(&self) -> Color {
        self.ptype.color
    }
}
impl Renderer for Piece {
    fn render(&self, context: Context, graphics: &mut G2d) {
        for block in self.blocks_iter() {
            render_square_in_grid(block.x, block.y, self.get_color(), context, graphics);
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
    Right
}

#[derive(Debug)]
pub struct Tetromino {
    configurations: [Configuration; 4],
    color: Color
}
impl Tetromino {
    fn get_configuration(&self, rotation: Rotation) -> &Configuration {
        &self.configurations[rotation]
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub x: i32,
    pub y: i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_rotated() {
        let mut p = Piece::create(&I);
        assert_eq!(p.rotation, 0);
        p = p.rotated();
        assert_eq!(p.rotation, 1);
        p = p.rotated();
        assert_eq!(p.rotation, 2);
        p = p.rotated();
        assert_eq!(p.rotation, 3);
        p = p.rotated();
        assert_eq!(p.rotation, 0);
    }

    #[test]
    fn test_piece_moved_left() {
        let mut p = Piece::create(&Z);
        let result = p.moved(Direction::Left);
        assert_eq!(result.x, p.x - 1);
    }

    #[test]
    fn test_piece_moved_right() {
        let mut p = Piece::create(&Z);
        let result = p.moved(Direction::Right);
        assert_eq!(result.x, p.x + 1);
    }

    #[test]
    fn test_piece_out_of_bounds() {
        let mut p = Piece::create(&I);
        p.x = 6;
        assert_eq!(p.is_out_of_bounds(), false);
        p.x = 7;
        assert_eq!(p.is_out_of_bounds(), true);
        p.x = 0;
        assert_eq!(p.is_out_of_bounds(), false);
        p.x = -1;
        assert_eq!(p.is_out_of_bounds(), true);
    }

    #[test]
    fn test_wall_kick_in_bounds() {
        let p = Piece::create(&I);
        let kicked = p.wall_kick();
        assert_eq!(kicked.x, p.x);
        assert_eq!(kicked.y, p.y);
    }

    #[test]
    fn test_wall_kick_out_of_bounds() {
        let mut p = Piece::create(&I);
        p.x = 7;
        let kicked = p.wall_kick();
        assert_eq!(kicked.x, 6);
        assert_eq!(kicked.y, p.y);
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
