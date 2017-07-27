use board::square::SquareBoard;
use interface::{Board, Generator};
use models::*;

struct SquareGenerator {}

impl Generator<SquareBoard> for SquareGenerator {
    fn generate(&self, board: &SquareBoard) -> SquareBoard {
        let neighbour_states = board.cells.iter().map(|x| x.location.neighbours()).map(
            |x| {
                x.into_iter()
                    .map(|x| board.at(x))
                    .map(state)
                    .collect::<Vec<_>>()
            },
        );

        let new_cells = board
            .cells
            .iter()
            .zip(neighbour_states)
            .map(|(cell, neighbours)| process(cell, neighbours));

        SquareBoard::new(0)
    }
}

fn process_cell(cell: &Cell, neighbours: Vec<CellState>) -> Cell {
    process(cell, neighbours.into_iter().map(Option::Some).collect())
}

fn process(cell: &Cell, neighbours: Vec<Option<CellState>>) -> Cell {
    match cell.cell_state {
        CellState::Dead => process_dead_cell(cell, neighbours),
        CellState::Alive => process_living_cell(cell, neighbours),
    }
}

fn process_dead_cell(cell: &Cell, neighbours: Vec<Option<CellState>>) -> Cell {
    let new_state = if count_of_living_neighbours(neighbours) == 3 {
        CellState::Alive
    } else {
        CellState::Dead
    };

    Cell {
        cell_state: new_state,
        location: cell.location,
    }
}

fn count_of_living_neighbours(neighbours: Vec<Option<CellState>>) -> usize {
    neighbours
        .iter()
        .filter(|&x| x.is_some())
        .map(|x| x.unwrap())
        .filter(|&x| x == CellState::Alive)
        .count()
}

fn process_living_cell(cell: &Cell, neighbours: Vec<Option<CellState>>) -> Cell {
    let new_state = match count_of_living_neighbours(neighbours) {
        x if x < 2 || x > 3 => CellState::Dead,
        _ => CellState::Alive,
    };

    Cell {
        cell_state: new_state,
        location: cell.location,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_dead_cell_with_no_neighbours_stays_dead() {
        let cell = Cell {
            cell_state: CellState::Dead,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

    #[test]
    fn a_dead_cell_with_dead_neighbours_stays_dead() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: CellState::Dead,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Dead, Dead, Dead, Dead, Dead, Dead, Dead];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

    #[test]
    fn a_dead_cell_with_less_than_three_live_neighbours_stays_dead() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: CellState::Dead,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Alive, Alive, Dead, Dead, Dead, Dead, Dead];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

    #[test]
    fn a_dead_cell_with_three_live_neighbours_comes_to_life() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: CellState::Dead,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Alive, Alive, Alive, Dead, Dead, Dead, Dead];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Alive);
    }

    #[test]
    fn a_dead_cell_with_more_than_three_live_neighbours_stays_dead() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: CellState::Dead,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Alive, Alive, Alive, Alive, Alive, Dead, Dead];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

    #[test]
    fn a_live_cell_with_no_neighbours_dies() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: Alive,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

    #[test]
    fn a_live_cell_with_one_living_neighbour_dies() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: Alive,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Alive];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

    #[test]
    fn a_live_cell_with_two_or_three_living_neighbours_stays_alive() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: Alive,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Alive, Alive];
        let neighbours2 = vec![Alive, Alive, Alive];

        let processed_cell = process_cell(&cell, neighbours);
        let processed_cell2 = process_cell(&cell, neighbours2);

        assert_eq!(processed_cell.cell_state, Alive);
        assert_eq!(processed_cell2.cell_state, Alive);
    }

    #[test]
    fn a_live_cell_with_more_than_three_living_neighbours_dies() {
        use self::CellState::*;
        let cell = Cell {
            cell_state: Alive,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![Alive, Alive, Alive, Alive];

        let processed_cell = process_cell(&cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }

}
