use crate::tetris::block::Block;

pub struct Shape {
    base_rotations: [[Block; 4]; 4]
}

impl Shape {
    fn new(base_rotations: [[Block; 4]; 4]) -> Shape {
        Shape {
            base_rotations
        }
    }

    pub fn j() -> Shape {
        Shape::new([
                [Block::new(3, 0), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
                [Block::new(4, 0), Block::new(5, 0), Block::new(4, 1), Block::new(4, 2)],
                [Block::new(5, 2), Block::new(3, 1), Block::new(4, 1), Block::new(5, 1)],
                [Block::new(4, 0), Block::new(3, 2), Block::new(4, 1), Block::new(4, 2)],
            ]
        )
    }

    pub fn has_block_at(&self, x: i8, y: i8, rotations: usize, x_diff: i8, y_diff: i8) -> bool {
        let base_rotation = rotations % self.base_rotations.len();
        match self.base_rotations.get(base_rotation) {
            Some(blocks) => {
                let mut matches = false;
                for block in blocks {
                    matches = block.x() == x - x_diff && block.y() == y - y_diff;
                    if matches {
                        break
                    }
                }
                matches
            }
            None => panic!("code error")
        }
    }
    
    pub fn is_off_grid(&self, rotations: usize, x_diff: i8, y_diff: i8) -> bool {
        let base_rotation = rotations % self.base_rotations.len();
        match self.base_rotations.get(base_rotation) {
            Some(blocks) => {
                let mut off_grid: bool = false;
                for block in blocks {
                    let x = block.x() + x_diff;
                    let y = block.y() + y_diff;
                    off_grid = x > 9 || x < 0 || y > 19 || y < 0;
                    if off_grid {
                        break
                    }
                }
                off_grid
            }
            None => panic!("code error")
        }
    }

}

