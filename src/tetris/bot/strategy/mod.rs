use crate::tetris::Tetris;

pub mod random;

pub trait Strategy {
    fn score(&self, outcome: &Tetris) -> f32;
}