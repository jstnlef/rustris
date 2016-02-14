use conrod::{color, Color};

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

// Tetromino colors
pub const CYAN: Color = Color::Rgba(0x00 as f32, 0xFF as f32, 0xff as f32, 1.0);
pub const BLUE: Color = color::BLUE;
pub const ORANGE: Color = color::ORANGE;
pub const YELLOW: Color = color::YELLOW;
pub const LIME: Color = Color::Rgba(0x80 as f32, 0xFF as f32, 0x00 as f32, 1.0);
pub const PURPLE: Color = color::PURPLE;
pub const RED: Color = color::RED;

pub struct Piece {
    // TODO: Replace with position
    x: u8,
    y: u8,
    ptype: Tetromino,
    rotation: Rotation
}
impl Piece {
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
    x: u8,
    y: u8,
    color: Color
}

#[derive(Debug, PartialEq)]
pub struct Position {
    x: u8,
    y: u8
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