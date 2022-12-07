use tetris_rust::tetris;
use tetris_rust::tetris::gui::Gui;
use tetris_rust::tetris::tetris::Tetris;


fn main() {
    let mut tetris: Tetris  = Tetris::new();
    let mut gui = Gui::new(tetris);
    gui.run();
}