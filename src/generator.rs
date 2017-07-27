use board::square::SquareBoard;
use interface::{Board, Generator};
use engine::*;

struct SquareGenerator {}

impl Generator<SquareBoard> for SquareGenerator {
    fn generate(&self, board: &SquareBoard) -> SquareBoard {
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

        SquareBoard::with_cells(new_cells)
    }
}

