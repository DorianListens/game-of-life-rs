use board::*;
use boards::square::SquareBoard;

struct SquareGenerator {}

impl Generator<SquareBoard> for SquareGenerator {
    fn generate(&self, board: &SquareBoard) -> SquareBoard {
        SquareBoard::new(0)
    }
}

fn find_neighbours(point: Coordinates) -> Vec<Coordinates> {
    vec![
        Coordinates {
            x: point.x - 1,
            y: point.y - 1,
        },
        Coordinates {
            x: point.x - 1,
            y: point.y,
        },
        Coordinates {
            x: point.x - 1,
            y: point.y + 1,
        },
        Coordinates {
            x: point.x,
            y: point.y - 1,
        },
        Coordinates {
            x: point.x,
            y: point.y + 1,
        },
        Coordinates {
            x: point.x + 1,
            y: point.y + 1,
        },
        Coordinates {
            x: point.x + 1,
            y: point.y,
        },
        Coordinates {
            x: point.x + 1,
            y: point.y - 1,
        },
    ]
}

fn process_cell(cell: Cell, neighbours: Vec<CellState>) -> Cell {
    match cell.cell_state {
        CellState::Dead => process_dead_cell(cell, neighbours),
        CellState::Alive => process_living_cell(cell, neighbours),
    }
}

fn process_dead_cell(cell: Cell, neighbours: Vec<CellState>) -> Cell {
    let live_count = neighbours
        .iter()
        .filter(|&x| x == &CellState::Alive)
        .count();

    let new_state = if live_count == 3 {
        CellState::Alive
    } else {
        CellState::Dead
    };

    Cell {
        cell_state: new_state,
        location: cell.location,
    }
}

fn process_living_cell(cell: Cell, neighbours: Vec<CellState>) -> Cell {
    Cell {
        cell_state: CellState::Dead,
        location: Coordinates { x: 0, y: 0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_neighbours_from_the_origin() {
        let coords = Coordinates { x: 0, y: 0 };
        let expected = vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: -1, y: 0 },
            Coordinates { x: -1, y: 1 },
            Coordinates { x: 0, y: -1 },
            Coordinates { x: 0, y: 1 },
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 1, y: 0 },
            Coordinates { x: 1, y: -1 },
        ];

        let result = find_neighbours(coords);

        assert_eq!(expected, result);
    }

    #[test]
    fn a_dead_cell_with_no_neighbours_stays_dead() {
        let cell = Cell {
            cell_state: CellState::Dead,
            location: Coordinates { x: 0, y: 0 },
        };
        let neighbours = vec![];

        let processed_cell = process_cell(cell, neighbours);

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

        let processed_cell = process_cell(cell, neighbours);

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

        let processed_cell = process_cell(cell, neighbours);

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

        let processed_cell = process_cell(cell, neighbours);

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

        let processed_cell = process_cell(cell, neighbours);

        assert_eq!(processed_cell.cell_state, CellState::Dead);
    }
}
