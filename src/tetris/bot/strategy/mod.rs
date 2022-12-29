use crate::tetris::bot::analysis::DeadBlocksAnalysis;
use crate::tetris::{Action, Tetris};

pub mod always_drop;

pub trait Strategy {
    fn shape_actions(&self, tetris: &Tetris) -> Vec<Action> {
        vec![]
    }
    fn score(&self, outcome: &DeadBlocksAnalysis) -> f32;
}