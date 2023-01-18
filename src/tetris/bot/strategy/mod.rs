use crate::tetris::{Action, Tetris};
use crate::tetris::Action::{Drop, Left, Right, Rotate};
use crate::tetris::ActionResult::Invalid;

pub mod qlearning;
pub mod weighted;

pub trait Strategy {
    fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action>;

    fn score(&self, outcome: &Tetris) -> f64;

    fn best_actions(&self, tetris: &Tetris) -> (Vec<&Action>, f64) {
        let mut best_actions = vec![&Drop];
        let mut best_actions_score = 0.0;
        for rotations in 0..4 {
            {
                let mut clone = tetris.clone();
                let mut actions = vec![&Rotate; rotations];
                actions.push(&Drop);
                for i in 0..actions.len() {
                    clone.input(actions[i]);
                }
                let score = self.score(&clone);
                if score > best_actions_score {
                    best_actions = actions;
                    best_actions_score = score;
                }
            }
            for action in [&Right, &Left] {
                let mut shift = 0;
                loop {
                    let mut clone = tetris.clone();
                    let mut actions = vec![&Rotate; rotations];
                    actions.append(&mut vec![action; shift]);
                    for i in 0..actions.len() {
                        clone.input(actions[i]);
                    }
                    let result = clone.input(action);
                    actions.push(action);
                    if result == Invalid {
                        break;
                    } else {
                        shift = shift + 1;
                        clone.input(&Drop);
                        actions.push(&Drop);
                        let score = self.score(&clone);
                        if score > best_actions_score {
                            best_actions = actions;
                            best_actions_score = score;
                        }
                    }
                }
            }
        }
        (best_actions, best_actions_score)
    }
}


#[cfg(test)]
mod tests {
    use crate::tetris::tests::tetris_with_only_j_shape;
    use crate::tetris::{Action, Block, Tetris};
    use crate::tetris::Action::{Drop, Left, Right, Rotate};
    use crate::tetris::bot::strategy::Strategy;

    fn has_blocks_at(tetris: &Tetris, blocks: [Block; 4]) -> bool {
        tetris.block_at(blocks[0].x, blocks[0].y) &&
            tetris.block_at(blocks[1].x, blocks[1].y) &&
            tetris.block_at(blocks[2].x, blocks[2].y) &&
            tetris.block_at(blocks[3].x, blocks[3].y)
    }

    #[test]
    fn should_just_drop_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }

            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(3, 18),
                    Block::new(3, 19),
                    Block::new(4, 19),
                    Block::new(5, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }

        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_partially_rotate_and_drop_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(4, 17),
                    Block::new(5, 17),
                    Block::new(4, 18),
                    Block::new(4, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Rotate, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_fully_rotate_and_drop_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(4, 17),
                    Block::new(3, 19),
                    Block::new(4, 18),
                    Block::new(4, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Rotate, &Rotate, &Rotate, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_move_part_the_way_left_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(2, 18),
                    Block::new(2, 19),
                    Block::new(3, 19),
                    Block::new(4, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Left, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_move_all_the_way_left_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(0, 18),
                    Block::new(0, 19),
                    Block::new(1, 19),
                    Block::new(2, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Left, &Left, &Left, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_move_part_the_way_right_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(4, 18),
                    Block::new(4, 19),
                    Block::new(5, 19),
                    Block::new(6, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Right, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_move_all_the_way_right_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(7, 18),
                    Block::new(7, 19),
                    Block::new(8, 19),
                    Block::new(9, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Right, &Right, &Right, &Right, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_rotate_and_move_left_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(0, 17),
                    Block::new(1, 17),
                    Block::new(0, 18),
                    Block::new(0, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Rotate, &Left, &Left, &Left, &Left, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }

    #[test]
    fn should_rotate_and_move_right_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn choose_actions(&self, tetris: &Tetris) -> Vec<&Action> {
                todo!()
            }
            fn score(&self, outcome: &Tetris) -> f64 {
                if has_blocks_at(outcome, [
                    Block::new(8, 17),
                    Block::new(9, 17),
                    Block::new(8, 18),
                    Block::new(8, 19)
                ]) {
                    1.0
                } else {
                    0.0
                }
            }
        }


        // when
        let strategy = TestStrategy {};
        let actions = strategy.best_actions(&tetris);

        // then
        let expected_actions = vec![&Rotate, &Right, &Right, &Right, &Right, &Drop];
        assert_eq!(expected_actions[..], actions.0[..]);
    }
}