use rayon::prelude::*;
use board::grid::GridBoard;
use interface::{Board, Generator};
use engine::*;

pub struct SquareGenerator {}

impl Generator<GridBoard> for SquareGenerator {
    fn generate(&self, board: &GridBoard) -> GridBoard {
        let new_rows = board
            .rows
            .par_iter()
            .map(|row| {
                row.par_iter()
                    .map(|x| {
                        process(
                            &x,
                            x.location
                                .neighbours()
                                .iter()
                                .filter_map(|n| board.at(*n))
                                .map(|x| x.cell_state)
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect()
            })
            .collect();

        GridBoard::with_rows(new_rows)
    }
}
