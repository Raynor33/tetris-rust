use crate::tetris::Tetris;

pub struct Analysis {
    pub gaps: u8,
    pub max_height: u8,
    pub deep_hole_blocks: u8,
}

pub fn analyse(tetris: &Tetris) -> Analysis {
    let mut gaps = 0;
    let mut max_height = 0;
    let mut column_heights: [u8; 10] = [0; 10];
    for x in 0u8..10u8 {
        let mut column_has_higher_block = false;
        for y in 0u8..20u8 {
            let block_present = tetris.dead_blocks[usize::from(x)][usize::from(y)];
            if block_present {
                let height = 20 - y;
                if max_height < height {
                    max_height = height;
                }
                if column_has_higher_block == false {
                    column_heights[usize::from(x)] = 20 - y;
                    column_has_higher_block = true;
                }
            } else if column_has_higher_block {
                gaps = gaps + 1;
            }
        }
    }
    let mut deep_hole_blocks = 0;
    let max_shallow_hole_depth = 2;
    for i in 0usize..10usize {
        let column_height = column_heights.get(i).unwrap();
        if i == 0 {
            if column_height + max_shallow_hole_depth < *column_heights.get(i + 1usize).unwrap() {
                deep_hole_blocks = deep_hole_blocks + column_heights.get(i + 1usize).unwrap() - column_height - max_shallow_hole_depth;
            }
        } else if i == 9 {
            if column_height + max_shallow_hole_depth < *column_heights.get(i - 1usize).unwrap() {
                deep_hole_blocks = deep_hole_blocks + column_heights.get(i - 1usize).unwrap() - column_height - max_shallow_hole_depth;
            }
        } else {
            let previous_height = column_heights.get(i - 1).unwrap();
            let next_height = column_heights.get(i + 1).unwrap();
            if column_height + max_shallow_hole_depth < *previous_height && column_height + max_shallow_hole_depth < *next_height {
                let lowest = if previous_height < next_height {
                    previous_height
                } else {
                    next_height
                };
                deep_hole_blocks = deep_hole_blocks + lowest - column_height - max_shallow_hole_depth

            }
        }
    }

    Analysis {
        gaps,
        max_height,
        deep_hole_blocks,
    }
}

#[cfg(test)]
mod tests {
    use crate::tetris::Action::{Drop, Left, Right, Rotate};
    use crate::tetris::tests::tetris_with_only_j_shape;
    use super::*;

    #[test]
    fn should_indicate_no_gaps_when_no_dead_blocks() {
        // given
        let tetris = tetris_with_only_j_shape();

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(0, analysis.gaps);
    }

    #[test]
    fn should_indicate_no_gaps_when_there_are_none() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(0, analysis.gaps);
    }

    #[test]
    fn should_indicate_correct_number_of_gaps() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Rotate);
        tetris.input(&Rotate);
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(2, analysis.gaps);
    }

    #[test]
    fn should_indicate_zero_max_height_when_no_dead_blocks() {
        // given
        let tetris = tetris_with_only_j_shape();

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(0, analysis.max_height);
    }

    #[test]
    fn should_indicate_correct_max_height() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(2, analysis.max_height);
    }

    #[test]
    fn should_indicate_zero_deep_hole_blocks_when_no_dead_blocks() {
        // given
        let mut tetris = tetris_with_only_j_shape();

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(0, analysis.deep_hole_blocks);
    }

    #[test]
    fn should_indicate_zero_deep_hole_blocks_when_there_are_none() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(0, analysis.deep_hole_blocks);
    }

    #[test]
    fn should_indicate_correct_deep_hole_blocks_next_to_edge() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Rotate);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(1, analysis.deep_hole_blocks);
    }

    #[test]
    fn should_indicate_correct_deep_hole_blocks_between_shapes() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Rotate);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Drop);

        tetris.input(&Rotate);
        tetris.input(&Right);
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(1, analysis.deep_hole_blocks);
    }

    #[test]
    fn should_indicate_correct_deep_hole_blocks_with_multiple_deep_holes() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Rotate);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Drop);

        tetris.input(&Rotate);
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(2, analysis.deep_hole_blocks);
    }

    #[test]
    fn should_indicate_correct_deep_hole_blocks_with_very_deep_hole() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Rotate);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Drop);

        tetris.input(&Rotate);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Left);
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(4, analysis.deep_hole_blocks);
    }
}