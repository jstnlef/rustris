use conrod::{color, Color};

// Tetromino colors
const CYAN: Color = Color::Rgba(0x00 as f32, 0xFF as f32, 0xff as f32, 1.0);
const BLUE: Color = color::BLUE;
const ORANGE: Color = color::ORANGE;
const YELLOW: Color = color::YELLOW;
const LIME: Color = Color::Rgba(0x80 as f32, 0xFF as f32, 0x00 as f32, 1.0);
const PURPLE: Color = color::PURPLE;
const RED: Color = color::RED;

struct Position {
    x: u32,
    y: u32
}

/*
Orientation is defined as a 16 bit unsigned integer which maps to a binary representation of the
tetromino on a 4 x 4 grid. e.g. Technique can be found here:
http://codeincomplete.com/posts/2011/10/10/javascript_tetris/
    0 0 0 0  -> 0
    1 1 1 1  -> F
    0 0 0 0  -> 0
    0 0 0 0  -> 0
    -------
    0 F 0 0
*/
type Orientation = u16;

#[derive(Clone, Debug, PartialEq)]
pub struct Tetromino {
    orientations: [Orientation; 4],
    color: Color
}
impl Tetromino {
    fn get_orientation(&self, index: usize) -> Orientation {
        self.orientations[index]
    }
}

pub const I: Tetromino = Tetromino {orientations: [0x0F00, 0x2222, 0x00F0, 0x4444], color: CYAN};
pub const J: Tetromino = Tetromino {orientations: [0x44C0, 0x8E00, 0x6440, 0x0E20], color: BLUE};
pub const L: Tetromino = Tetromino {orientations: [0x4460, 0x0E80, 0xC440, 0x2E00], color: ORANGE};
pub const O: Tetromino = Tetromino {orientations: [0xCC00, 0xCC00, 0xCC00, 0xCC00], color: YELLOW};
pub const S: Tetromino = Tetromino {orientations: [0x06C0, 0x8C40, 0x6C00, 0x4620], color: LIME};
pub const T: Tetromino = Tetromino {orientations: [0x0E40, 0x4C40, 0x4E00, 0x4640], color: PURPLE};
pub const Z: Tetromino = Tetromino {orientations: [0x0C60, 0x4C80, 0xC600, 0x2640], color: RED};

#[cfg(test)]
mod tetromino_tests {
    use super::*;

    #[test]
    fn test_get_orientation() {
        // TODO: Flesh out these tests
        assert_eq!(J.get_orientation(3), 0x0E20);
        assert_eq!(L.get_orientation(2), 0xC440);
    }
}
