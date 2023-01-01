use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Tetris};
use crate::tetris::bot::analysis::analyse;

pub struct Fixed {}

impl Strategy for Fixed {
    fn score(&self, tetris: &Tetris) -> f64 {
        let analysis = analyse(tetris);
        // let height_score = if analysis.max_height < 16 {
        let height_score = 20.0 - (analysis.max_height as f64);
        // } else {
        //     100000000.0 * (20.0 - (analysis.max_height as f64))
        // };
        let deep_hole_score = 100.0 * (200.0 - (analysis.deep_hole_blocks as f64));
        let gaps_score = 100000.0 * (200.0 - (analysis.gaps as f64));
        height_score + deep_hole_score + gaps_score
    }
}

impl Fixed {
    pub fn new() -> Fixed {
        Fixed {}
    }
}