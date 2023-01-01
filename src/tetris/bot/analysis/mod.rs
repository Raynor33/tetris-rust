use crate::tetris::Tetris;

pub struct Analysis {
    pub gaps: u8,
    pub max_height: u8,
    pub total_neighbour_diff: u8,
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
    let mut total_neighbour_diff = 0;
    for i in 1usize..10usize {
        let previous_column_height = column_heights.get(i - 1).unwrap();
        let column_height = column_heights.get(i).unwrap();
        let absolute_diff = column_height.abs_diff(*previous_column_height);
        total_neighbour_diff = total_neighbour_diff + absolute_diff;
    }

    Analysis {
        gaps,
        max_height,
        total_neighbour_diff
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
    fn should_indicate_zero_total_neighbour_diff_when_no_dead_blocks() {
        // given
        let mut tetris = tetris_with_only_j_shape();

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(0, analysis.total_neighbour_diff);
    }

    #[test]
    fn should_indicate_correct_total_neighbour_diff() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Drop);

        // when
        let analysis = analyse(&tetris);

        // then
        assert_eq!(4, analysis.total_neighbour_diff);
    }
}