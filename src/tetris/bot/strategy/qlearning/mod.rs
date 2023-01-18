use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, Tetris};

pub struct QLearning{
    // training_buffer_capacity: i32,
    // training_buffer: Vec<TrainingItem>,
    // training_frequency: i32,
}

struct TrainingItem {
    input: [f64; 4],
    output: [f64; 1],
}

impl QLearning {
    pub fn new() -> QLearning {
        QLearning{}
    }

    fn should_experiment(&self) -> bool {
        false
    }

    fn train(&self, tetris: &Tetris, best_action_score: f64) {
    }
}

impl Strategy for QLearning {
    fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
        let best_action = self.best_actions(tetris);
        self.train(tetris, best_action.1);
        if self.should_experiment() {
            todo!()
        }
        best_action.0
    }

    fn score(&self, outcome: &Tetris) -> f64 {
        todo!()
    }
}