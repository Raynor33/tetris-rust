use std::{thread, time};
use std::io::{stdout, Write};
use crossterm::{cursor, ExecutableCommand, QueueableCommand, terminal};
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{ActionResult, Tetris};

pub mod analysis;
pub mod strategy;

pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }

    pub fn run(&self, strategy: &dyn Strategy, action_pause: u64, debug: bool, games: u32) {
        for game in 0..games {
            self.do_game(strategy, action_pause, debug)
        }
    }

    pub fn do_game(&self, strategy: &dyn Strategy, action_pause: u64, debug: bool) {
        let mut tetris = Tetris::new();
        let mut shape_count = 0;
        loop {
            shape_count = shape_count + 1;
            let actions = strategy.choose_actions(&tetris);
            for action in actions {
                if action_pause > 0 {
                    thread::sleep(time::Duration::from_millis(action_pause));
                }
                let result = tetris.input(action);
                if debug {
                    Bot::draw(&tetris);
                }
                if result == ActionResult::NextShape {
                    // this could happen if not all actions are required to reach the next shape
                    // eg if a down tick changes the sequence needed to complete the current shape
                    break;
                }
                if result == ActionResult::GameOver {
                    println!("Game completed after {} shapes handled", shape_count);
                    stdout().execute(cursor::Show).unwrap();
                    return;
                }
            }
        }
    }

    fn draw(tetris: &Tetris) {
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
        blocks_string.push_str(" ---------- \n");

        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();
        stdout.execute(cursor::MoveToRow(0)).unwrap();
        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        stdout.write_all(blocks_string.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}