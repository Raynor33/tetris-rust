use crate::tetris::Tetris;

pub mod random;
pub mod weighted;

pub trait Strategy {
    fn score(&self, outcome: &Tetris) -> f64;
}