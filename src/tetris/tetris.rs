use rand::distributions::{Distribution, Uniform};
use crate::tetris::action::Action;
use crate::tetris::shape::Shape;
use crate::tetris::shape_factory::ShapeFactory;
use rand::Rng;
use rand::rngs::ThreadRng;


struct Tetris {
    shapes: Vec<Shape>,
    rng: ThreadRng,
    die: Uniform<usize>,
    current_shapes_index: usize,
}

impl Tetris {
    fn new() -> Tetris {
        Tetris::new_with_custom_shapes(vec![
            Shape::j(),
            // Shape::l(),
        ])
    }

    fn new_with_custom_shapes(shapes: Vec<Shape>) -> Tetris {
        let shapes_count = shapes.len();
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..shapes_count);
        let i = die.sample(&mut rng);
        Tetris {
            shapes,
            rng,
            die,
            current_shapes_index: i,
        }
    }

    fn block_at(&self, x: i8, y: i8) -> bool {
        match self.shapes.get(self.current_shapes_index) {
            Some(shape) => {
                shape.has_block_at(x, y, 0, 0, 0)
            }
            None => panic!("code error")
        }
    }

    fn input(&self, action: Action) -> &Tetris {
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::tetris::shape::Shape;
    use super::*;

    struct FakeShapeFactory {
        shape: Shape,
    }

    impl FakeShapeFactory {
        fn new(shape: Shape) -> FakeShapeFactory {
            FakeShapeFactory {
                shape
            }
        }
    }

    impl ShapeFactory for FakeShapeFactory {
        fn next(&self) -> &Shape {
            &self.shape
        }
    }

    fn count_blocks(tetris: &Tetris) -> i32 {
        let mut count = 0;
        for x in 0..10 {
            for y in 0..20 {
                if tetris.block_at(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn blocks_as_string(tetris: &Tetris) -> String {
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

    #[test]
    fn should_start_with_a_shape() {
        // given
        let tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);

        // when / then
        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
    }
}