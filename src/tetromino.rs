use piston_window::{Context, G2d, Transformed, rectangle};
use piston_window::types::{Color, Matrix2d};

use colors::*;
use game::Renderer;
use settings::*;

// TODO: look into having a make-block! macro for when this inevitably needs to change
pub const I: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:2, color: CYAN}, Block{x:1, y:2, color: CYAN}, Block{x:2, y:2, color: CYAN}, Block{x:3, y:2, color: CYAN}],
        [Block{x:2, y:0, color: CYAN}, Block{x:2, y:1, color: CYAN}, Block{x:2, y:2, color: CYAN}, Block{x:2, y:3, color: CYAN}],
        [Block{x:0, y:1, color: CYAN}, Block{x:1, y:1, color: CYAN}, Block{x:2, y:1, color: CYAN}, Block{x:3, y:1, color: CYAN}],
        [Block{x:1, y:0, color: CYAN}, Block{x:1, y:1, color: CYAN}, Block{x:1, y:2, color: CYAN}, Block{x:1, y:3, color: CYAN}]
    ]
};
pub const J: Tetromino = Tetromino {
    configurations: [
        [Block{x:1, y:3, color: BLUE}, Block{x:1, y:2, color: BLUE}, Block{x:1, y:1, color: BLUE}, Block{x:0, y:1, color: BLUE}],
        [Block{x:0, y:3, color: BLUE}, Block{x:0, y:2, color: BLUE}, Block{x:1, y:2, color: BLUE}, Block{x:2, y:2, color: BLUE}],
        [Block{x:1, y:3, color: BLUE}, Block{x:1, y:2, color: BLUE}, Block{x:1, y:1, color: BLUE}, Block{x:2, y:3, color: BLUE}],
        [Block{x:0, y:2, color: BLUE}, Block{x:1, y:2, color: BLUE}, Block{x:2, y:2, color: BLUE}, Block{x:2, y:1, color: BLUE}]
    ]
};
pub const L: Tetromino = Tetromino {
    configurations: [
        [Block{x:1, y:3, color: ORANGE}, Block{x:1, y:2, color: ORANGE}, Block{x:1, y:1, color: ORANGE}, Block{x:2, y:1, color: ORANGE}],
        [Block{x:0, y:1, color: ORANGE}, Block{x:0, y:2, color: ORANGE}, Block{x:1, y:2, color: ORANGE}, Block{x:2, y:2, color: ORANGE}],
        [Block{x:0, y:3, color: ORANGE}, Block{x:1, y:3, color: ORANGE}, Block{x:1, y:2, color: ORANGE}, Block{x:1, y:1, color: ORANGE}],
        [Block{x:0, y:2, color: ORANGE}, Block{x:1, y:2, color: ORANGE}, Block{x:2, y:2, color: ORANGE}, Block{x:2, y:3, color: ORANGE}]
    ]
};
pub const O: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:3, color: YELLOW}, Block{x:0, y:2, color: YELLOW}, Block{x:1, y:3, color: YELLOW}, Block{x:1, y:2, color: YELLOW}],
        [Block{x:0, y:3, color: YELLOW}, Block{x:0, y:2, color: YELLOW}, Block{x:1, y:3, color: YELLOW}, Block{x:1, y:2, color: YELLOW}],
        [Block{x:0, y:3, color: YELLOW}, Block{x:0, y:2, color: YELLOW}, Block{x:1, y:3, color: YELLOW}, Block{x:1, y:2, color: YELLOW}],
        [Block{x:0, y:3, color: YELLOW}, Block{x:0, y:2, color: YELLOW}, Block{x:1, y:3, color: YELLOW}, Block{x:1, y:2, color: YELLOW}]
    ]
};
pub const S: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:1, color: LIME}, Block{x:1, y:1, color: LIME}, Block{x:1, y:2, color: LIME}, Block{x:2, y:2, color: LIME}],
        [Block{x:0, y:3, color: LIME}, Block{x:0, y:2, color: LIME}, Block{x:1, y:2, color: LIME}, Block{x:1, y:1, color: LIME}],
        [Block{x:0, y:2, color: LIME}, Block{x:1, y:2, color: LIME}, Block{x:1, y:3, color: LIME}, Block{x:2, y:3, color: LIME}],
        [Block{x:1, y:3, color: LIME}, Block{x:1, y:2, color: LIME}, Block{x:2, y:2, color: LIME}, Block{x:2, y:1, color: LIME}]
    ]
};
pub const T: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:2, color: PURPLE}, Block{x:1, y:2, color: PURPLE}, Block{x:1, y:1, color: PURPLE}, Block{x:2, y:2, color: PURPLE}],
        [Block{x:0, y:2, color: PURPLE}, Block{x:1, y:3, color: PURPLE}, Block{x:1, y:2, color: PURPLE}, Block{x:1, y:1, color: PURPLE}],
        [Block{x:0, y:2, color: PURPLE}, Block{x:1, y:3, color: PURPLE}, Block{x:1, y:2, color: PURPLE}, Block{x:2, y:2, color: PURPLE}],
        [Block{x:1, y:3, color: PURPLE}, Block{x:1, y:2, color: PURPLE}, Block{x:1, y:1, color: PURPLE}, Block{x:2, y:2, color: PURPLE}]
    ]
};
pub const Z: Tetromino = Tetromino {
    configurations: [
        [Block{x:0, y:2, color: RED}, Block{x:1, y:2, color: RED}, Block{x:1, y:1, color: RED}, Block{x:2, y:1, color: RED}],
        [Block{x:0, y:2, color: RED}, Block{x:0, y:1, color: RED}, Block{x:1, y:2, color: RED}, Block{x:1, y:3, color: RED}],
        [Block{x:0, y:3, color: RED}, Block{x:1, y:3, color: RED}, Block{x:1, y:2, color: RED}, Block{x:2, y:2, color: RED}],
        [Block{x:1, y:1, color: RED}, Block{x:1, y:2, color: RED}, Block{x:2, y:2, color: RED}, Block{x:2, y:3, color: RED}]
    ]
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
        let position = Position {x: WIDTH_IN_BLOCKS / 2, y: 0};
        Piece::new(position, ptype, Rotation::R0)
    }

    fn new(position: Position, ptype: Tetromino, rotation: Rotation) -> Piece {
        Piece {
            x: position.x,
            y: position.y,
            ptype: ptype,
            rotation: rotation
        }
    }

    fn get_blocks(&self) -> &Configuration {
        self.ptype.get_configuration(&self.rotation)
    }

    fn rotate(&mut self) {
        let new_rotation = match self.rotation {
            Rotation::R0 => Rotation::R1,
            Rotation::R1 => Rotation::R2,
            Rotation::R2 => Rotation::R3,
            Rotation::R3 => Rotation::R0
        };
        self.rotation = new_rotation;
    }
}

#[derive(Debug, PartialEq)]
pub enum Rotation {
    R0, R1, R2, R3
}

#[derive(Debug)]
pub struct Tetromino {
    configurations: [Configuration; 4]
}
impl Tetromino {
    fn get_configuration(&self, rotation: &Rotation) -> &Configuration {
        let index = match *rotation {
            Rotation::R0 => 0,
            Rotation::R1 => 1,
            Rotation::R2 => 2,
            Rotation::R3 => 3
        };
        &self.configurations[index]
    }
}

type Configuration = [Block; 4];

// Store the x and y position relative to local coordinates
#[derive(Debug, PartialEq)]
pub struct Block {
    x: u32,
    y: u32,
    color: Color
}

#[derive(Debug, PartialEq)]
pub struct Position {
    x: u32,
    y: u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_piece_rotation() {
        let position = Position{x:0, y:0};
        let mut p = Piece::new(position, I, Rotation::R0);
        p.rotate();
	    assert_eq!(p.rotation, Rotation::R1);
        p.rotate();
	    assert_eq!(p.rotation, Rotation::R2);
        p.rotate();
	    assert_eq!(p.rotation, Rotation::R3);
        p.rotate();
	    assert_eq!(p.rotation, Rotation::R0);
    }
}