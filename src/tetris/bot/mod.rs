use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{ActionResult, Tetris};
use crate::tetris::Action::Drop;

pub mod analysis;

pub mod strategy;

pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }

    pub fn run(&self, strategy: &dyn Strategy) {
        let mut tetris = Tetris::new();
        let mut shape_count = 0;
        // TODO
        let all_action_sequences = vec![vec![Drop]];
        loop {
            shape_count = shape_count + 1;
            let chosen_actions_index = strategy.current_shape_actions(&tetris, &all_action_sequences);
            for action in all_action_sequences.get(chosen_actions_index).unwrap() {
                let result = tetris.input(action);
                if result == ActionResult::NextShape {
                    // this could happen if not all actions are required to reach the next shape
                    // eg if a down tick changes the sequence needed to complete the current shape
                    break;
                }
                if result == ActionResult::GameOver {
                    println!("Game completed after {} shapes handled", shape_count);
                    return;
                }
            }
        }
    }
}