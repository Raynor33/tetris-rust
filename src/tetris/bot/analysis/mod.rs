use crate::tetris::Tetris;

pub struct Analysis {
    pub gaps: u8,
    pub max_height: u8,
    pub total_neighbour_diff: u8,
}

pub fn analyse(tetris: &Tetris) -> Analysis {
    let mut gaps = 0;
    let mut max_height = 0;
    let mut total_neighbour_diff = 0;
    let mut previous_column_height = 0;
    let mut current_column_height;
    for x in 0u8..10u8 {
        current_column_height = 0;
        let mut column_has_higher_block = false;
        for y in 0u8..20u8 {
            let block_present = tetris.dead_blocks[usize::from(x)][usize::from(y)];
            if block_present {
                if max_height < current_column_height {
                    max_height = current_column_height;
                }
                if column_has_higher_block == false {
                    current_column_height = 20 - y;
                }
                column_has_higher_block = true;
            } else if column_has_higher_block {
                gaps = gaps + 1;
            }
            if y == 19 {
                if x > 0 {
                    total_neighbour_diff = total_neighbour_diff + current_column_height.abs_diff(previous_column_height);
                }
                previous_column_height = current_column_height;
            }
        }
    }

    Analysis {
        gaps,
        max_height,
        total_neighbour_diff,
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