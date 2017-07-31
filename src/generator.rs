use rayon::prelude::*;
use models::{Cell, CellState};
use interface::{Board, Generator};
use engine::process;

pub struct SimpleGenerator {}

impl<T: Board> Generator<T> for SimpleGenerator {
    fn generate(&self, board: &T) -> T {
        let new_rows = board
            .rows()
            .par_iter()
            .map(|row| next_row(board, row))
            .collect();

        T::from(new_rows)
    }
}

fn next_row<T: Board>(board: &T, row: &Vec<Cell>) -> Vec<Cell> {
    row.par_iter()
        .map(|cell| process(&cell, neighbour_states(board, &cell)))
        .collect()
}

fn neighbour_states<T: Board>(board: &T, cell: &Cell) -> Vec<CellState> {
    cell.location
        .neighbours()
        .into_iter()
        .filter_map(|n| board.at(n))
        .map(|x| x.cell_state)
        .collect()
}
