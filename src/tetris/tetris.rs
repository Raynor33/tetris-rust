use rand::distributions::{Distribution, Uniform};
use crate::tetris::action::Action;
use crate::tetris::shape::Shape;
use rand::Rng;
use rand::rngs::ThreadRng;


struct Tetris {
    shapes: Vec<Shape>,
    rng: ThreadRng,
    die: Uniform<usize>,
    current_shapes_index: usize,
    current_shape_rotations: usize,
    current_shape_x_diff: i8,
    current_shape_y_diff: i8,
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
            current_shape_rotations: 0,
            current_shape_x_diff: 0,
            current_shape_y_diff: 0,
        }
    }

    fn block_at(&self, x: i8, y: i8) -> bool {
        match self.shapes.get(self.current_shapes_index) {
            Some(shape) => {
                shape.has_block_at(x, y, self.current_shape_rotations, self.current_shape_x_diff, self.current_shape_y_diff)
            }
            None => panic!("code error")
        }
    }

    fn valid(&self, rotations: usize, x_diff: i8, y_diff: i8) -> bool {
        match self.shapes.get(self.current_shapes_index) {
            Some(shape) => {
                !shape.is_off_grid(rotations, x_diff, y_diff)
            }
            None => panic!("code error")
        }
    }

    fn input(&mut self, action: Action) -> &Tetris {
        match action {
            Action::Left => {
                if self.valid(self.current_shape_rotations, self.current_shape_x_diff - 1, self.current_shape_y_diff) {
                    self.current_shape_x_diff = self.current_shape_x_diff - 1
                }
            }
            Action::Right => {
                if self.valid(self.current_shape_rotations, self.current_shape_x_diff + 1, self.current_shape_y_diff) {
                    self.current_shape_x_diff = self.current_shape_x_diff + 1
                }
            }
            Action::Rotate => {
                if self.valid(self.current_shape_rotations + 1, self.current_shape_x_diff, self.current_shape_y_diff) {
                    self.current_shape_rotations = self.current_shape_rotations + 1
                }
            }
            Action::Down => {
                if self.valid(self.current_shape_rotations, self.current_shape_x_diff, self.current_shape_y_diff + 1) {
                    self.current_shape_y_diff = self.current_shape_y_diff + 1
                }
            }
            Action::Drop => {

            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::tetris::action::Action::{Down, Left, Right, Rotate};
    use crate::tetris::shape::Shape;
    use super::*;

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
        blocks_string.push_str(" 0123456789 \n");
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
        blocks_string.push_str(" 0123456789 ");
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

    #[test]
    fn should_move_shape_left() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);

        // when / then
        tetris.input(Left);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(2, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(2, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_move_shape_right() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);

        // when / then
        tetris.input(Right);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(6, 1), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_move_shape_down() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);

        // when / then
        tetris.input(Down);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 2), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 2), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 2), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_rotate_shape() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);

        // when / then
        tetris.input(Rotate);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 2), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_not_move_shape_left_off_the_grid() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        tetris.input(Left);
        tetris.input(Left);
        tetris.input(Left);

        // when / then
        tetris.input(Left);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(0, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(0, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(1, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(2, 1), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_not_move_shape_right_off_the_grid() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        tetris.input(Right);
        tetris.input(Right);
        tetris.input(Right);
        tetris.input(Right);

        // when / then
        tetris.input(Right);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(7, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(7, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(8, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(9, 1), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_not_rotate_shape_off_the_grid() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        tetris.input(Rotate);
        tetris.input(Left);
        tetris.input(Left);
        tetris.input(Left);
        tetris.input(Left);

        // when / then
        tetris.input(Rotate);

        assert_eq!(4, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(0, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(1, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(0, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(0, 2), "\n{}", blocks_as_string(&tetris));
    }

    // #[test]
    // fn should_add_a_new_shape_when_current_shape_reaches_the_bottom() {
    //     // given
    //     let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
    //     for i in 0..17 {
    //         tetris.input(Down);
    //     }
    //
    //     // when / then
    //     tetris.input(Down);
    //
    //     assert_eq!(8, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(3, 18), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(3, 19), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(4, 19), "\n{}", blocks_as_string(&tetris));
    //     assert!(tetris.block_at(5, 19), "\n{}", blocks_as_string(&tetris));
    // }

}