use std::sync::mpsc;
use std::{process, thread, time};
use tetris_rust::tetris::gui::Gui;
use tetris_rust::tetris::{ActionResult, Tetris};


fn main() {
    let (action_sender, action_receiver) = mpsc::channel();
    let (blocks_sender, blocks_receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut tetris = Tetris::new();
        loop {
            let result = tetris.input(action_receiver.recv().unwrap());

            let mut blocks = [[false; 20]; 10];
            for x in 0i8..10i8 {
                for y in 0i8..20i8 {
                    blocks[usize::from(x.unsigned_abs())][usize::from(y.unsigned_abs())] = tetris.block_at(x, y);
                }
            }
            blocks_sender.send(blocks).unwrap();
            if result == ActionResult::GameOver {
                thread::sleep(time::Duration::from_millis(500));
                process::exit(0);
            }
        }
    });
    Gui::launch(action_sender, blocks_receiver);
}