use piston_window::{Context, G2d, Transformed, rectangle};
use piston_window::types::{Color, Matrix2d};

use colors::*;
use game::{Renderer, render_square_in_grid};
use settings::*;

pub const I: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:2, y:2}, Block{x:3, y:2}],
        [Block{x:2, y:0}, Block{x:2, y:1}, Block{x:2, y:2}, Block{x:2, y:3}],
        [Block{x:0, y:1}, Block{x:1, y:1}, Block{x:2, y:1}, Block{x:3, y:1}],
        [Block{x:1, y:0}, Block{x:1, y:1}, Block{x:1, y:2}, Block{x:1, y:3}]
    ],
    color: CYAN
};
pub const J: Tetromino = Tetromino {
    configurations: [
        [Block{x:1, y:3}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:0, y:1}],
        [Block{x:0, y:3}, Block{x:0, y:2}, Block{x:1, y:2}, Block{x:2, y:2}],
        [Block{x:1, y:3}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:2, y:3}],
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:2, y:2}, Block{x:2, y:1}]
    ],
    color: BLUE
};
pub const L: Tetromino = Tetromino {
    configurations: [
        [Block{x:1, y:3}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:2, y:1}],
        [Block{x:0, y:1}, Block{x:0, y:2}, Block{x:1, y:2}, Block{x:2, y:2}],
        [Block{x:0, y:3}, Block{x:1, y:3}, Block{x:1, y:2}, Block{x:1, y:1}],
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:2, y:2}, Block{x:2, y:3}]
    ],
    color: ORANGE
};
pub const O: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:3}, Block{x:0, y:2}, Block{x:1, y:3}, Block{x:1, y:2}],
        [Block{x:0, y:3}, Block{x:0, y:2}, Block{x:1, y:3}, Block{x:1, y:2}],
        [Block{x:0, y:3}, Block{x:0, y:2}, Block{x:1, y:3}, Block{x:1, y:2}],
        [Block{x:0, y:3}, Block{x:0, y:2}, Block{x:1, y:3}, Block{x:1, y:2}]
    ],
    color: YELLOW
};
pub const S: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:1}, Block{x:1, y:1}, Block{x:1, y:2}, Block{x:2, y:2}],
        [Block{x:0, y:3}, Block{x:0, y:2}, Block{x:1, y:2}, Block{x:1, y:1}],
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:1, y:3}, Block{x:2, y:3}],
        [Block{x:1, y:3}, Block{x:1, y:2}, Block{x:2, y:2}, Block{x:2, y:1}]
    ],
    color: LIME
};
pub const T: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:2, y:2}],
        [Block{x:0, y:2}, Block{x:1, y:3}, Block{x:1, y:2}, Block{x:1, y:1}],
        [Block{x:0, y:2}, Block{x:1, y:3}, Block{x:1, y:2}, Block{x:2, y:2}],
        [Block{x:1, y:3}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:2, y:2}]
    ],
    color: PURPLE
};
pub const Z: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:2}, Block{x:1, y:2}, Block{x:1, y:1}, Block{x:2, y:1}],
        [Block{x:0, y:2}, Block{x:0, y:1}, Block{x:1, y:2}, Block{x:1, y:3}],
        [Block{x:0, y:3}, Block{x:1, y:3}, Block{x:1, y:2}, Block{x:2, y:2}],
        [Block{x:1, y:1}, Block{x:1, y:2}, Block{x:2, y:2}, Block{x:2, y:3}]
    ],
    color: RED
};

pub struct Piece {
    // TODO: Replace with position
    x: u32,
    y: u32,
    ptype: Tetromino,
    rotation: Rotation
}
impl Piece {
    pub fn create(ptype: Tetromino) -> Piece {
        Piece::new(WIDTH_IN_BLOCKS / 2, 0, ptype, 0)
    }

    fn new(x: u32, y: u32, ptype: Tetromino, rotation: Rotation) -> Piece {
        Piece {
            x: x,
            y: y,
            ptype: ptype,
            rotation: rotation
        }
    }

    fn get_blocks(&self) -> &Configuration {
        self.ptype.get_configuration(self.rotation)
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % self.ptype.configurations.len();
    }

    pub fn move_piece(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                // TODO: Replace with in bounds check
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            Direction::Right => {
                if self.x < WIDTH_IN_BLOCKS - 1 {
                    self.x += 1;
                }
            }
        }
    }
}
impl Renderer for Piece {
    fn render(&self, context: Context, graphics: &mut G2d) {
        for block in self.get_blocks() {
            let x = self.x as i32 + block.x;
            let y = self.y as i32 + block.y;
            render_square_in_grid(x as u32, y as u32, self.ptype.color, context, graphics);
        }
    }
}

pub enum Direction {
    Left,
    Right
}

type Rotation = usize;

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

type Configuration = [Block; 4];

// Store the x and y position relative to rotation point
#[derive(Debug)]
pub struct Block {
    x: i32,
    y: i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_piece_rotation() {
        let mut p = Piece::create(I);
        p.rotate();
	    assert_eq!(p.rotation, 1);
        p.rotate();
	    assert_eq!(p.rotation, 2);
        p.rotate();
	    assert_eq!(p.rotation, 3);
        p.rotate();
	    assert_eq!(p.rotation, 0);
    }
}