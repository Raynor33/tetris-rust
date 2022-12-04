use crate::tetris::shape::Shape;

pub trait ShapeFactory {
    fn next(&self) -> &Shape;
}