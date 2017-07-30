use rayon::prelude::*;
use board::grid::GridBoard;
use interface::{Board, Generator};
use engine::*;
use models::{Cell, CellState};

pub struct SquareGenerator {}

impl Generator<GridBoard> for SquareGenerator {
    fn generate(&self, board: &GridBoard) -> GridBoard {
        let new_rows = board
            .rows()
            .par_iter()
            .map(|row| next_row(board, row))
            .collect();

        GridBoard::with_rows(new_rows)
    }
}

fn next_row(board: &GridBoard, row: &Vec<Cell>) -> Vec<Cell> {
    row.par_iter()
        .map(|cell| process(&cell, neighbour_states(board, &cell)))
        .collect()
}

fn neighbour_states(board: &GridBoard, cell: &Cell) -> Vec<CellState> {
    cell.location
        .neighbours()
        .into_iter()
        .filter_map(|n| board.at(n))
        .map(|x| x.cell_state)
        .collect()
}
