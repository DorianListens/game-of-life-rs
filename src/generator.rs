use board::square::GridBoard;
use interface::{Board, Generator};
use engine::*;

pub struct SquareGenerator {}

impl Generator<GridBoard> for SquareGenerator {
    fn generate(&self, board: &GridBoard) -> GridBoard {
        let neighbour_states = board.cells.iter().map(|x| x.location.neighbours()).map(
            |x| {
                x.into_iter()
                    .filter_map(|x| board.at(x))
                    .map(|x| x.cell_state)
                    .collect::<Vec<_>>()
            },
        );

        let new_cells = board
            .cells
            .iter()
            .zip(neighbour_states)
            .map(|(cell, neighbours)| process(cell, neighbours))
            .collect();

        GridBoard::with_cells(new_cells)
    }
}
