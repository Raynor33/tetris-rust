use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, Tetris};
use crate::tetris::bot::analysis::DeadBlocksAnalysis;

pub struct AlwaysDrop {
}

impl Strategy for AlwaysDrop {

    fn shape_actions(&self, tetris: &Tetris) -> Vec<Action> {
        vec![Action::Drop]
    }

    fn score(&self, outcome: &DeadBlocksAnalysis) -> f32 {
        0.0
    }
}

impl AlwaysDrop {
    pub fn new() -> AlwaysDrop {
        AlwaysDrop{}
    }
}