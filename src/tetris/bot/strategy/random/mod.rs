use rand::Rng;
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, Tetris};

pub struct Random {
}

impl Strategy for Random {
    fn choose_actions(&mut self, tetris: &Tetris) -> Vec<Action> {
        self.best_actions(tetris).actions
    }

    fn score(&self, _: &Tetris) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..1.0)
    }
}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}