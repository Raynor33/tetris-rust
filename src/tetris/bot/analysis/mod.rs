use crate::tetris::Tetris;

pub struct DeadBlocksAnalysis {
    pub gaps: u8,
    pub max_height: u8,
}

pub fn analyse_dead_blocks(tetris: &Tetris) -> DeadBlocksAnalysis {
    let mut gaps = 0;
    let mut max_height = 0;
    for x in 0u8..10u8 {
        let mut column_has_higher_block = false;
        for y in 0u8..20u8 {
            let block_present = tetris.dead_blocks[usize::from(x)][usize::from(y)];
            if block_present {
                let height = 20 - y;
                if max_height < height {
                    max_height = height;
                }
                column_has_higher_block = true;
            }
            else if column_has_higher_block {
                gaps = gaps + 1;
            }
        }
    }
    DeadBlocksAnalysis {
        gaps,
        max_height,
    }
}

#[cfg(test)]
mod tests {
    use crate::tetris::Action::{Drop, Rotate};
    use crate::tetris::tests::tetris_with_only_j_shape;
    use super::*;

    #[test]
    fn should_indicate_no_gaps_when_no_dead_blocks() {
        // given
        let tetris = tetris_with_only_j_shape();

        // when
        let analysis = analyse_dead_blocks(&tetris);

        // then
        assert_eq!(0, analysis.gaps);
    }

    #[test]
    fn should_indicate_no_gaps_when_there_are_none() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Drop);

        // when
        let analysis = analyse_dead_blocks(&tetris);

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
        let analysis = analyse_dead_blocks(&tetris);

        // then
        assert_eq!(2, analysis.gaps);
    }

    #[test]
    fn should_indicate_zero_max_height_when_no_dead_blocks() {
        // given
        let tetris = tetris_with_only_j_shape();

        // when
        let analysis = analyse_dead_blocks(&tetris);

        // then
        assert_eq!(0, analysis.max_height);
    }

    #[test]
    fn should_indicate_correct_max_height() {
        // given
        let mut tetris = tetris_with_only_j_shape();
        tetris.input(&Drop);

        // when
        let analysis = analyse_dead_blocks(&tetris);

        // then
        assert_eq!(2, analysis.max_height);
    }
}