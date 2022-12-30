use ActionResult::Invalid;
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, ActionResult, Tetris};
use crate::tetris::Action::{Drop, Left, Right, Rotate};

pub struct Decisions {}

impl Decisions {
    pub fn new() -> Decisions {
        Decisions {}
    }
    pub fn choose_actions(&self, tetris: &Tetris, strategy: &dyn Strategy) -> Vec<&Action> {
        let mut best_actions = vec![&Drop];
        let mut best_actions_score = 0.0;
        for rotations in 0..4 {
            {
                let mut clone = tetris.clone();
                let mut actions = vec![&Rotate; rotations];
                actions.push(&Drop);
                for i in 0..actions.len() {
                    clone.input(actions.get(i).unwrap());
                }
                let score = strategy.score(&clone);
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
                        clone.input(actions.get(i).unwrap());
                    }
                    let result = clone.input(action);
                    actions.push(action);
                    if result == Invalid {
                        break;
                    } else {
                        shift = shift + 1;
                        clone.input(&Drop);
                        actions.push(&Drop);
                        let score = strategy.score(&clone);
                        if score > best_actions_score {
                            best_actions = actions;
                            best_actions_score = score;
                        }
                    }
                }
            }
        }
        best_actions
    }
}


#[cfg(test)]
mod tests {
    use crate::tetris::bot::decisions::Decisions;
    use crate::tetris::bot::strategy::Strategy;
    use crate::tetris::tests::tetris_with_only_j_shape;
    use crate::tetris::{Block, Tetris};
    use crate::tetris::Action::{Drop, Left, Right, Rotate};

    fn has_blocks_at(tetris: &Tetris, blocks: [Block; 4]) -> bool {
        tetris.block_at(blocks.get(0).unwrap().x, blocks.get(0).unwrap().y) &&
            tetris.block_at(blocks.get(1).unwrap().x, blocks.get(1).unwrap().y) &&
            tetris.block_at(blocks.get(2).unwrap().x, blocks.get(2).unwrap().y) &&
            tetris.block_at(blocks.get(3).unwrap().x, blocks.get(3).unwrap().y)
    }

    #[test]
    fn should_just_drop_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_partially_rotate_and_drop_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Rotate, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_fully_rotate_and_drop_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Rotate, &Rotate, &Rotate, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_move_part_the_way_left_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Left, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_move_all_the_way_left_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Left, &Left, &Left, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_move_part_the_way_right_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Right, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_move_all_the_way_right_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Right, &Right, &Right, &Right, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_rotate_and_move_left_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Rotate, &Left, &Left, &Left, &Left, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }

    #[test]
    fn should_rotate_and_move_right_when_that_is_best() {
        // given
        let tetris = tetris_with_only_j_shape();

        // and
        struct TestStrategy {}
        impl Strategy for TestStrategy {
            fn score(&self, outcome: &Tetris) -> f32 {
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

        let decisions = Decisions {};

        // when
        let strategy = TestStrategy {};
        let actions = decisions.choose_actions(&tetris, &strategy);

        // then
        let expected_actions = vec![&Rotate, &Right, &Right, &Right, &Right, &Drop];
        assert_eq!(expected_actions[..], actions[..]);
    }
}