use rand::Rng;
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, Tetris};
use crate::tetris::bot::analysis::DeadBlocksAnalysis;

pub struct Random {
}

impl Strategy for Random {

    fn score(&self, outcome: &DeadBlocksAnalysis) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..1.0)
    }
}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}