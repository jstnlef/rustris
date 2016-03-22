use std::collections::VecDeque;
use rand::{Rng, thread_rng};

use tetromino::*;

pub struct Randomizer {
    history: VecDeque<&'static Tetromino>
}
impl Randomizer {
    pub fn new() -> Randomizer {
        let mut rand = Randomizer {
            history: VecDeque::new()
        };
        rand.add_to_history(&Z);
        rand.add_to_history(&S);
        rand.add_to_history(&Z);
        rand.add_to_history(&S);
        rand
    }

    pub fn create_piece(&mut self) -> Piece {
        let all_tetrominos = [&I, &J, &L, &O, &S, &T, &Z];
        let mut random_ptype = None;
        for _ in 0..6 {
            random_ptype = thread_rng().choose(&all_tetrominos);
            match random_ptype {
                Some(ptype) => {
                    if self.history.iter().all(|&item| item != *ptype) {
                        break;
                    }
                },
                _ => {}
            }
        }
        let ptype = random_ptype.unwrap();
        self.add_to_history(ptype);
        Piece::create(ptype)
    }

    fn add_to_history(&mut self, ptype: &'static Tetromino) {
        self.history.push_back(ptype);
        if self.history.len() > 4 {
            self.history.pop_front();
        }
        debug_assert!(self.history.len() <= 4);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tetromino::*;

    #[test]
    fn test_add_to_history() {
        let mut rand = Randomizer::new();
        assert_eq!(rand.history.len(), 4);
        rand.add_to_history(&L);
        let length = rand.history.len();
        assert_eq!(length, 4);
        assert_eq!(rand.history[length - 1], &L);
    }
}
