use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Tetris};
use crate::tetris::bot::analysis::analyse;

pub struct Weighted {}

impl Strategy for Weighted {
    fn score(&self, tetris: &Tetris) -> f64 {
        let analysis = analyse(tetris);
        let height_score = 200.0 - (analysis.max_height as f64);
        let total_neighbour_diff_score = 200.0 - (analysis.total_neighbour_diff as f64);
        let gaps_score = 200.0 - (analysis.gaps as f64);
        let low_edges_score = 200.0 - (analysis.low_edges as f64);
        let height_weight = 1.0;
        let total_neighbour_diff_weight = 15.0;
        let gaps_weight = 100.0;
        let low_edges_weight = 50.0;
        height_score * height_weight +
            total_neighbour_diff_score * total_neighbour_diff_weight +
            low_edges_score * low_edges_weight +
            gaps_score * gaps_weight
    }
}

impl Weighted {
    pub fn new() -> Weighted {
        Weighted {}
    }
}