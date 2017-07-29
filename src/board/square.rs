use interface::Board;
use models::*;

extern crate rand;
use rand::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SquareBoard {
    pub cells: Vec<Cell>,
    pub size: i32,
}

impl SquareBoard {
    pub fn new(size: i32) -> SquareBoard {
        SquareBoard {
            size: size,
            cells: Vec::new(),
        }
    }

    pub fn with_cells(cells: Vec<Cell>) -> SquareBoard {
        SquareBoard {
            size: 0,
            cells,
        }
    }

    pub fn all_alive(size: i32) -> SquareBoard {
        let mut vec = Vec::new();

        for x in 0..size {
            for y in 0..size {
                let location = Coordinates { x, y };
                let state = CellState::Alive;
                let cell = Cell {
                    cell_state: state,
                    location: location,
                };
                vec.push(cell);
            }
        }

        SquareBoard {
            size: size,
            cells: vec,
        }
    }

    pub fn random(size: i32) -> SquareBoard {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::new();

        for x in 0..size {
            for y in 0..size {
                let location = Coordinates { x, y };
                let state = if rng.gen() {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
                let cell = Cell {
                    cell_state: state,
                    location: location,
                };
                vec.push(cell);
            }
        }

        SquareBoard {
            size: size,
            cells: vec,
        }
    }
}

impl Board for SquareBoard {
    fn at(&self, coordinates: Coordinates) -> Option<&Cell> {
        self.cells.iter().find(|x| x.location == coordinates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_square_board_all_alive() {
        let board = SquareBoard::all_alive(5);
        let x = 0;
        let y = 0;

        let x2 = 4;
        let y2 = 4;
        let cell = board.at(Coordinates { x, y });
        let cell2 = board.at(Coordinates { x: x2, y: y2 });
        assert_eq!(
            cell,
            Some(&Cell {
                cell_state: CellState::Alive,
                location: Coordinates { x, y },
            })
        );
        assert_eq!(
            cell2,
            Some(&Cell {
                cell_state: CellState::Alive,
                location: Coordinates { x: x2, y: y2 },
            })
        );
    }

    #[test]
    fn a_square_board_outside_edges_returns_none() {
        let board = SquareBoard::all_alive(2);
        let outside_of_board = Coordinates { x: 3, y: 3 };
        let none_cell = board.at(outside_of_board);
        assert_eq!(none_cell, None);
    }
}
