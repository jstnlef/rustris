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
        self.ptype.get_configuration(self.rotation)
    }

    pub fn rotated(&self) -> Self {
        let rotation = (self.rotation + 1) % self.ptype.configurations.len();
        Self::new(self.x, self.y, self.ptype, rotation)
    }

    pub fn moved(&self, direction: Direction) -> Self {
        let translated_x = match direction {
            Direction::Left => self.x - 1,
            Direction::Right => self.x + 1
        };
        Self::new(translated_x, self.y, self.ptype, self.rotation)
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.get_blocks().iter().any(|block| {
            let x = self.x + block.x;
            x < 0 || x >= WIDTH_IN_BLOCKS as i32
        })
    }

    pub fn wall_kick(&self) -> Self {
        let mut translation = 0;
        let min_block = self.get_blocks().iter().min_by_key(|block| self.x + block.x).unwrap();
        let max_block = self.get_blocks().iter().max_by_key(|block| self.x + block.x).unwrap();
        if self.x + min_block.x < 0 {
            translation = self.x + min_block.x;
        } else if self.x + max_block.x >= WIDTH_IN_BLOCKS as i32 {
            translation = (self.x + max_block.x) - (WIDTH_IN_BLOCKS - 1) as i32;
        }
        Self::new(self.x - translation, self.y, self.ptype, self.rotation)
    }

    fn get_color(&self) -> Color {
        self.ptype.color
    }
}
impl Renderer for Piece {
    fn render(&self, context: Context, graphics: &mut G2d) {
        for block in self.get_blocks() {
            let x = self.x + block.x;
            let y = self.y + block.y;
            render_square_in_grid(x, y, self.get_color(), context, graphics);
        }
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

#[derive(Debug)]
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
    fn test_wall_kick() {
        assert_eq!(false, true)
    }
}
