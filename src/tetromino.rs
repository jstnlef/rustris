use piston_window::{Context, G2d, Transformed, rectangle};
use piston_window::types::{Color, Matrix2d};
use rand::{Rng, thread_rng};

use colors::*;
use game::{Renderer, render_square_in_grid};
use settings::WIDTH_IN_BLOCKS;


pub fn get_random_piece() -> Piece {
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
        self.ptype.get_configuration(self.rotation)
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % self.ptype.configurations.len();
    }

    pub fn move_piece(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }
}
impl Renderer for Piece {
    fn render(&self, context: Context, graphics: &mut G2d) {
        for block in self.get_blocks() {
            let x = self.x + block.x;
            let y = self.y + block.y;
            render_square_in_grid(x, y, self.ptype.color, context, graphics);
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

pub type Configuration = [Block; 4];

#[derive(Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32
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
