use rayon::prelude::*;
use board::square::GridBoard;
use interface::{Board, Generator};
use engine::*;

pub struct SquareGenerator {}

impl Generator<GridBoard> for SquareGenerator {
    fn generate(&self, board: &GridBoard) -> GridBoard {
        let new_cells = board
            .cells
            .iter()
            .map(|x| {
                    (x, x.location.neighbours().iter()
                        .filter_map(|n| board.at(*n))
                        .map(|x| x.cell_state)
                        .collect::<Vec<_>>())
                }
            )
            .map(|(cell, neighbours)| process(cell, neighbours))
            .collect();

        GridBoard::with_cells(new_cells)
    }
}
