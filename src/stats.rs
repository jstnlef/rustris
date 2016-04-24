use std::cmp::min;

use settings::MAX_GAME_LEVEL;

const LEVEL_THRESHOLD: u32 = 10;

pub struct GameStats {
    score: u32,
    lines: u32,
}
impl GameStats {
    pub fn new() -> GameStats {
        GameStats {
            score: 0,
            lines: 0
        }
    }

    pub fn score_soft_drop(&mut self) {
        self.score += 1;
    }

    pub fn score_hard_drop(&mut self, rows_dropped: u32) {
        self.score += 2 * rows_dropped;
    }

    pub fn score_completed_lines(&mut self, lines: u32) {
        self.score += match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0
        } * self.get_level();

        self.lines += lines;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_lines(&self) -> u32 {
        self.lines
    }

    pub fn get_level(&self) -> u32 {
        let level = (self.lines / LEVEL_THRESHOLD) + 1;
        min(level, MAX_GAME_LEVEL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings::MAX_GAME_LEVEL;

    #[test]
    fn test_score_completed_lines() {
        let mut stats = GameStats::new();
        stats.score_completed_lines(1);
        assert_eq!(stats.get_score(), 100);
        assert_eq!(stats.get_level(), 1);
        assert_eq!(stats.get_lines(), 1);

        stats.score_completed_lines(2);
        assert_eq!(stats.get_score(), 400);
        assert_eq!(stats.get_level(), 1);
        assert_eq!(stats.get_lines(), 3);

        stats.score_completed_lines(3);
        assert_eq!(stats.get_score(), 900);
        assert_eq!(stats.get_level(), 1);
        assert_eq!(stats.get_lines(), 6);

        stats.score_completed_lines(4);
        assert_eq!(stats.get_score(), 1700);
        assert_eq!(stats.get_level(), 2);
        assert_eq!(stats.get_lines(), 10);
    }

    #[test]
    fn test_score_soft_drop() {
        let mut stats = GameStats::new();
        stats.score_soft_drop();
        assert_eq!(stats.get_score(), 1);
        for _ in 0..20 {
            stats.score_soft_drop();
        }
        assert_eq!(stats.get_score(), 21);
    }

    #[test]
    fn test_score_hard_drop() {
        let mut stats = GameStats::new();
        stats.score_hard_drop(10);
        assert_eq!(stats.get_score(), 20);
    }

    #[test]
    fn test_get_level() {
        let mut stats = GameStats::new();
        assert_eq!(stats.get_level(), 1);
        stats.lines = 89;
        assert_eq!(stats.get_level(), 9);
        stats.lines = 150;
        assert_eq!(stats.get_level(), MAX_GAME_LEVEL);
    }
}

