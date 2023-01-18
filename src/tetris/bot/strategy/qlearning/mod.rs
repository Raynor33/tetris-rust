use std::cell::Cell;
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, Tetris};
use crate::tetris::ActionResult::GameOver;
use crate::tetris::bot::analysis::analyse;
use crate::tetris::bot::strategy::random::Random;

const GAME_OVER_PENALTY: f64 = 1.0;
const DISCOUNT_RATE: f64 = 0.99;

pub struct QLearning {
    random_strategy: Random,
    training_input_count: usize,
    training_buffer_capacity: usize,
    training_buffer: Vec<TrainingItem>,
    training_frequency: usize,
}

struct TrainingItem {
    input: [f64; 4],
    output: f64,
}

impl QLearning {
    pub fn new() -> QLearning {
        QLearning {
            random_strategy : Random::new(),
            training_input_count: 0,
            training_buffer_capacity: 10000,
            training_buffer: vec![],
            training_frequency: 10
        }
    }

    fn should_experiment(&self) -> bool {
        false
    }

    fn train(&mut self, tetris: &Tetris, best_action_score: f64) {
        let analysis = analyse(tetris);
        let input = [
            analysis.low_edges as f64 / 200.0,
            analysis.gaps as f64 / 200.0,
            analysis.central_columns_max_height as f64 / 200.0,
            analysis.total_neighbour_diff as f64 / 200.0,
        ];
        let training_item = TrainingItem{
            input,
            output: best_action_score
        };
        if self.training_buffer.len() >= self.training_buffer_capacity {
            self.training_buffer[self.training_input_count % self.training_buffer_capacity] = training_item
        } else {
            self.training_buffer.push(training_item);
        }


        self.training_input_count = self.training_input_count + 1;

    }
}

impl Strategy for QLearning {
    fn choose_actions(&mut self, tetris: &Tetris) -> Vec<Action> {
        let best_actions = self.best_actions(tetris);
        if best_actions.result == GameOver {
            self.train(tetris, GAME_OVER_PENALTY);
        } else {
            self.train(tetris, best_actions.score * DISCOUNT_RATE);
        }
        if self.should_experiment() {
            self.random_strategy.choose_actions(tetris)
        } else {
            best_actions.actions
        }
    }

    fn score(&self, outcome: &Tetris) -> f64 {
        1.0
    }
}