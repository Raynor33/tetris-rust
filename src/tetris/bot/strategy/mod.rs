use std::thread::scope;
use crate::tetris::bot::analysis::{analyse_dead_blocks, DeadBlocksAnalysis};
use crate::tetris::{Action, Tetris};
use crate::tetris::Action::Drop;
use crate::tetris::ActionResult::{GameOver, NextShape};

pub mod random;

pub trait Strategy {
    fn current_shape_actions(&self, tetris: &Tetris, actions_vectors: &Vec<Vec<Action>>) -> usize {
        let mut best_actions_index = 0;
        let mut best_score = 0.0;
        for i in 0..actions_vectors.len() {
            let mut clone = tetris.clone();
            for action in actions_vectors.get(i).unwrap() {
                let result = clone.input(action);
                if result == NextShape || result == GameOver {
                    let score = self.score(&analyse_dead_blocks(tetris));
                    if score > best_score {
                        best_score = score;
                        best_actions_index = i;
                    }
                    break
                }
            }
        }
        best_actions_index
    }
    fn score(&self, outcome: &DeadBlocksAnalysis) -> f32;
}