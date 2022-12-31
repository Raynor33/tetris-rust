use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Tetris};
use crate::tetris::bot::analysis::analyse_dead_blocks;

pub struct NoGaps {
}

impl Strategy for NoGaps {
    fn score(&self, tetris: &Tetris) -> f64 {
        let analysis = analyse_dead_blocks(tetris);
        let height_score = 20.0 - (analysis.max_height as f64);
        let gaps_score = 100.0 * (200.0 - (analysis.gaps as f64));
        height_score + gaps_score
    }
}

impl NoGaps {
    pub fn new() -> NoGaps {
        NoGaps {}
    }
}