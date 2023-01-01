use std::{thread, time};
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{ActionResult, Tetris};
use crate::tetris::bot::decisions::Decisions;

pub mod analysis;
pub mod decisions;
pub mod strategy;

pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }

    pub fn run(&self, strategy: &dyn Strategy, action_pause: u64, debug: bool) {
        let mut tetris = Tetris::new();
        let mut shape_count = 0;
        loop {
            shape_count = shape_count + 1;
            let decisions = Decisions::new();
            let actions = decisions.choose_actions(&tetris, strategy);
            for action in actions {
                if action_pause > 0 {
                    thread::sleep(time::Duration::from_millis(action_pause));
                }
                let result = tetris.input(action);
                if result == ActionResult::NextShape {
                    // this could happen if not all actions are required to reach the next shape
                    // eg if a down tick changes the sequence needed to complete the current shape
                    break;
                }
                if result == ActionResult::GameOver {
                    println!("{}", Self::blocks_as_string(&mut tetris));
                    println!("Game completed after {} shapes handled", shape_count);
                    return;
                }
                if debug {
                    println!("{}", Self::blocks_as_string(&mut tetris))
                }
            }
        }
    }

    fn blocks_as_string(tetris: &mut Tetris) -> String {
        let mut blocks_string = String::new();
        blocks_string.push_str(" ---------- \n");
        for y in 0..20 {
            blocks_string.push_str("|");
            for x in 0..10 {
                if tetris.block_at(x, y) {
                    blocks_string.push_str("*");
                } else {
                    blocks_string.push_str(" ");
                }
            }
            blocks_string.push_str("|\n");
        }
        blocks_string.push_str(" ---------- ");
        blocks_string
    }
}