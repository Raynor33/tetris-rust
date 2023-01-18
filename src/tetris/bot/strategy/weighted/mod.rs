use crate::tetris::bot::strategy::{BestActions, Strategy};
use crate::tetris::{Action, ActionResult, Tetris};
use crate::tetris::bot::analysis::analyse;

pub struct Weighted {
}

impl Strategy for Weighted {
    fn choose_actions(&mut self, tetris: &Tetris) -> Vec<Action> {
        self.best_actions(tetris).actions
    }
    fn best_actions(&self, tetris: &Tetris) -> BestActions {
        todo!()
    }

    fn score(&self, tetris: &Tetris) -> f64 {
        let analysis = analyse(tetris);
        let central_columns_height_score = if analysis.central_columns_max_height < 16 { 200.0 } else { 200.0 - (analysis.central_columns_max_height as f64) };
        let total_neighbour_diff_score = 200.0 - (analysis.total_neighbour_diff as f64);
        let gaps_score = 200.0 - (analysis.gaps as f64);
        let low_edges_score = 200.0 - (analysis.low_edges as f64);
        let central_columns_height_weight = 500.0;
        let total_neighbour_diff_weight = 15.0;
        let gaps_weight = 100.0;
        let low_edges_weight = 50.0;
        central_columns_height_score * central_columns_height_weight +
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