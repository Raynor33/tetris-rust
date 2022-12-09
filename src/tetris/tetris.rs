use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use crate::tetris::tetris::Action::Down;

pub enum Action {
    Left,
    Right,
    Rotate,
    Down,
    Drop,
}

struct Block {
    x: i8,
    y: i8,
}

impl Block {
    fn new(x: i8, y: i8) -> Block {
        Block { x, y }
    }
    fn x(&self) -> i8 {
        self.x
    }
    fn y(&self) -> i8 {
        self.y
    }
}

struct Shape {
    base_rotations: [[Block; 4]; 4],
}

impl Shape {
    fn new(base_rotations: [[Block; 4]; 4]) -> Shape {
        Shape {
            base_rotations
        }
    }

    pub fn o() -> Shape {
        Shape::new([
            [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(5, 1)],
        ])
    }

    pub fn s() -> Shape {
        Shape::new([
            [Block::new(4, 0), Block::new(5, 0), Block::new(3, 1), Block::new(4, 1)],
            [Block::new(4, 0), Block::new(4, 1), Block::new(5, 1), Block::new(5, 2)],
            [Block::new(4, 1), Block::new(5, 1), Block::new(3, 2), Block::new(4, 2)],
            [Block::new(3, 0), Block::new(3, 1), Block::new(4, 1), Block::new(4, 2)],
        ])
    }

    pub fn z() -> Shape {
        Shape::new([
            [Block::new(3, 0), Block::new(4, 0), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(5, 0), Block::new(5, 1), Block::new(4, 1), Block::new(4, 2)],
            [Block::new(3, 1), Block::new(4, 1), Block::new(4, 2), Block::new(5, 2)],
            [Block::new(4, 0), Block::new(4, 1), Block::new(3, 1), Block::new(3, 2)],
        ])
    }
    pub fn t() -> Shape {
        Shape::new([
            [Block::new(4, 0), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(4, 1), Block::new(4, 2), Block::new(5, 1)],
            [Block::new(4, 2), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(3, 1), Block::new(4, 1), Block::new(4, 2)],
        ])
    }
    pub fn l() -> Shape {
        Shape::new([
            [Block::new(5, 0), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(4, 1), Block::new(4, 2), Block::new(5, 2)],
            [Block::new(3, 2), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(3, 0), Block::new(4, 0), Block::new(4, 1), Block::new(4, 2)],
        ])
    }
    pub fn j() -> Shape {
        Shape::new([
            [Block::new(3, 0), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(4, 2)],
            [Block::new(5, 2), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
            [Block::new(4, 0), Block::new(3, 2), Block::new(4, 1), Block::new(4, 2)],
        ])
    }
    pub fn i() -> Shape {
        Shape::new([
            [Block::new(3, 1), Block::new(4, 1), Block::new(5, 1), Block::new(6, 1)],
            [Block::new(5, 0), Block::new(5, 1), Block::new(5, 2), Block::new(5, 3)],
            [Block::new(3, 2), Block::new(4, 2), Block::new(5, 2), Block::new(6, 2)],
            [Block::new(4, 0), Block::new(4, 1), Block::new(4, 2), Block::new(4, 3)],
        ])
    }

    fn has_block_at(&self, x: i8, y: i8, rotations: usize, x_diff: i8, y_diff: i8) -> bool {
        let base_rotation = rotations % self.base_rotations.len();
        match self.base_rotations.get(base_rotation) {
            Some(blocks) => {
                let mut matches = false;
                for block in blocks {
                    matches = block.x() == x - x_diff && block.y() == y - y_diff;
                    if matches {
                        break;
                    }
                }
                matches
            }
            None => panic!("code error")
        }
    }

    fn is_off_grid(&self, rotations: usize, x_diff: i8, y_diff: i8) -> bool {
        let base_rotation = rotations % self.base_rotations.len();
        match self.base_rotations.get(base_rotation) {
            Some(blocks) => {
                let mut off_grid: bool = false;
                for block in blocks {
                    let x = block.x() + x_diff;
                    let y = block.y() + y_diff;
                    off_grid = x > 9 || x < 0 || y > 19 || y < 0;
                    if off_grid {
                        break;
                    }
                }
                off_grid
            }
            None => panic!("code error")
        }
    }

    fn apply_to(&self, grid: &mut [[bool; 20]; 10], rotations: usize, x_diff: i8, y_diff: i8) {
        let base_rotation = rotations % self.base_rotations.len();
        match self.base_rotations.get(base_rotation) {
            Some(blocks) => {
                for block in blocks {
                    let x = block.x() + x_diff;
                    let y = block.y() + y_diff;
                    grid[usize::from(x.unsigned_abs())][usize::from(y.unsigned_abs())] = true;
                }
            }
            None => panic!("code error")
        }
    }

    fn intersects(&self, grid: &[[bool; 20]; 10], rotations: usize, x_diff: i8, y_diff: i8) -> bool {
        let base_rotation = rotations % self.base_rotations.len();
        match self.base_rotations.get(base_rotation) {
            Some(blocks) => {
                for block in blocks {
                    let x = block.x() + x_diff;
                    let y = block.y() + y_diff;
                    if grid[usize::from(x.unsigned_abs())][usize::from(y.unsigned_abs())] {
                        return true;
                    }
                }
                false
            }
            None => panic!("code error")
        }
    }
}

pub struct Tetris {
    shapes: Vec<Shape>,
    rng: ThreadRng,
    die: Uniform<usize>,
    current_shapes_index: usize,
    current_shape_rotations: usize,
    current_shape_x_diff: i8,
    current_shape_y_diff: i8,
    dead_blocks: [[bool; 20]; 10],
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris::new_with_custom_shapes(vec![
            Shape::o(),
            Shape::s(),
            Shape::z(),
            Shape::t(),
            Shape::l(),
            Shape::j(),
            Shape::i(),
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
            dead_blocks: [[false; 20]; 10],
        }
    }

    pub fn block_at(&self, x: i8, y: i8) -> bool {
        let is_dead_block = self.dead_blocks[usize::from(x.unsigned_abs())][usize::from(y.unsigned_abs())];
        let is_current_shape = match self.shapes.get(self.current_shapes_index) {
            Some(shape) => {
                shape.has_block_at(x, y, self.current_shape_rotations, self.current_shape_x_diff, self.current_shape_y_diff)
            }
            None => panic!("code error")
        };
        is_dead_block || is_current_shape
    }

    fn validate_and_place(&mut self, rotations: usize, x_diff: i8, y_diff: i8) {
        match self.shapes.get(self.current_shapes_index) {
            Some(shape) => {
                let valid = !shape.is_off_grid(rotations, x_diff, y_diff) &&
                    !shape.intersects(&self.dead_blocks, rotations, x_diff, y_diff);
                if valid {
                    let shape_finished = shape.is_off_grid(rotations, x_diff, y_diff + 1) ||
                        shape.intersects(&self.dead_blocks, rotations, x_diff, y_diff + 1);
                    if shape_finished {
                        shape.apply_to(&mut self.dead_blocks, rotations, x_diff, y_diff);
                        self.complete_lines();
                        self.current_shapes_index = self.die.sample(&mut self.rng);
                        self.current_shape_rotations = 0;
                        self.current_shape_x_diff = 0;
                        self.current_shape_y_diff = 0;
                    } else {
                        self.current_shape_rotations = rotations;
                        self.current_shape_x_diff = x_diff;
                        self.current_shape_y_diff = y_diff;
                    }
                }
            }
            None => panic!("code error")
        }
    }

    fn complete_lines(&mut self) {
        let mut completed_lines = 0;
        for y in (0u8..20u8).rev() {
            let mut line_complete = true;
            for x in 0u8..10u8 {
                let current_block_present = self.dead_blocks[usize::from(x)][usize::from(y)];
                line_complete = line_complete && current_block_present;
                self.dead_blocks[usize::from(x)][usize::from(y + completed_lines)] = current_block_present;
                if completed_lines > 0 {
                    self.dead_blocks[usize::from(x)][usize::from(y)] = false
                }
            }
            if line_complete {
                completed_lines = completed_lines + 1;
            }
        }
    }

    #[allow(unused_qualifications)]
    pub fn input(&mut self, action: Action) -> &Tetris {
        match action {
            Action::Left => {
                self.validate_and_place(self.current_shape_rotations, self.current_shape_x_diff - 1, self.current_shape_y_diff);
            }
            Action::Right => {
                self.validate_and_place(self.current_shape_rotations, self.current_shape_x_diff + 1, self.current_shape_y_diff);
            }
            Action::Rotate => {
                self.validate_and_place(self.current_shape_rotations + 1, self.current_shape_x_diff, self.current_shape_y_diff);
            }
            Action::Down => {
                self.validate_and_place(self.current_shape_rotations, self.current_shape_x_diff, self.current_shape_y_diff + 1);
            }
            Action::Drop => {
                loop {
                    self.input(Down);
                    if self.current_shape_y_diff == 0 {
                        break;
                    }
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::tetris::tetris::Action::{Down, Drop, Left, Right, Rotate};
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

    #[test]
    fn should_not_move_shape_left_into_dead_blocks() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        for _ in 0..9 {
            tetris.input(Left);
            tetris.input(Left);
            tetris.input(Left);
            tetris.input(Drop);
        }
        tetris.input(Down);
        tetris.input(Down);

        // when / then
        tetris.input(Left);

        assert_eq!(40, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 2), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 3), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 3), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 3), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_not_move_shape_right_into_dead_blocks() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        for _ in 0..9 {
            tetris.input(Right);
            tetris.input(Right);
            tetris.input(Right);
            tetris.input(Drop);
        }
        tetris.input(Down);
        tetris.input(Down);

        // when / then
        tetris.input(Right);

        assert_eq!(40, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 2), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 3), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 3), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 3), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_not_rotate_shape_into_dead_blocks() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        for _ in 0..9 {
            tetris.input(Right);
            tetris.input(Right);
            tetris.input(Right);
            tetris.input(Drop);
        }
        tetris.input(Rotate);
        tetris.input(Rotate);
        tetris.input(Rotate);
        tetris.input(Right);
        tetris.input(Down);
        tetris.input(Down);

        // when / then
        tetris.input(Rotate);

        assert_eq!(40, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        // [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(4, 2)],
        assert!(tetris.block_at(5, 2), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 3), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 4), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 4), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_add_a_new_shape_when_current_shape_reaches_the_bottom() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        for _ in 0..17 {
            tetris.input(Down);
        }

        // when / then
        tetris.input(Down);

        assert_eq!(8, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 18), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 19), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_add_a_new_shape_when_current_shape_reaches_dead_blocks() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        for _ in 0..18 {
            tetris.input(Down);
        }
        for _ in 0..15 {
            tetris.input(Down);
        }

        // when / then
        tetris.input(Down);

        assert_eq!(12, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 16), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 17), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 17), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 17), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 18), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 19), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_drop_to_the_bottom() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);

        // when / then
        tetris.input(Drop);

        assert_eq!(8, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 18), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 19), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_drop_to_dead_blocks() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        tetris.input(Drop);

        // when / then
        tetris.input(Drop);

        assert_eq!(12, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 16), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 17), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 17), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 17), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 18), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 19), "\n{}", blocks_as_string(&tetris));
    }

    #[test]
    fn should_complete_a_line() {
        // given
        let mut tetris = Tetris::new_with_custom_shapes(vec![Shape::j()]);
        tetris.input(Left);
        tetris.input(Left);
        tetris.input(Left);
        tetris.input(Drop);

        tetris.input(Drop);

        tetris.input(Right);
        tetris.input(Right);
        tetris.input(Right);
        tetris.input(Drop);

        tetris.input(Rotate);
        tetris.input(Rotate);
        tetris.input(Right);
        tetris.input(Right);
        tetris.input(Right);
        tetris.input(Right);

        // when / then
        tetris.input(Drop);

        assert_eq!(10, count_blocks(&tetris), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 0), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(4, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(5, 1), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(0, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(3, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(6, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(7, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(8, 19), "\n{}", blocks_as_string(&tetris));
        assert!(tetris.block_at(9, 19), "\n{}", blocks_as_string(&tetris));
    }
}