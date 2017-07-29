use interface::Board;
use models::*;

extern crate rand;
use rand::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GridBoard {
    pub cells: Vec<Cell>,
    pub width: i32,
    pub height: i32,
}

impl GridBoard {
    pub fn square(size: i32) -> GridBoard {
        GridBoard {
            width: size,
            height: size,
            cells: Vec::new(),
        }
    }

    pub fn with_cells(cells: Vec<Cell>) -> GridBoard {
        GridBoard { width: 0, height: 0, cells }
    }

    pub fn all_alive(size: i32) -> GridBoard {
        GridBoard::fill_with(size, size, Box::new(|_| CellState::Alive ))
    }

    pub fn random_square(size: i32) -> GridBoard {
        GridBoard::random(size, size)
    }

    pub fn random(width: i32, height: i32) -> GridBoard {
        let mut rng = rand::thread_rng();
        GridBoard::fill_with(width, height, Box::new(move |_| {
            if rng.gen() {
                CellState::Alive
            } else {
                CellState::Dead
            }
        }))
    }

    fn fill_with(width: i32, height: i32, mut state_for_coords: Box<FnMut(Coordinates) -> CellState>) -> GridBoard {
        let mut cells = Vec::new();

        for x in 0..width {
            for y in 0..height {
                let location = Coordinates { x, y };
                let state = state_for_coords(location);
                let cell = Cell {
                    cell_state: state,
                    location: location,
                };
                cells.push(cell);
            }
        }

        GridBoard {
            width,
            height,
            cells,
        }
    }
}

impl Board for GridBoard {
    fn at(&self, coordinates: Coordinates) -> Option<Cell> {
        self.cells
            .iter()
            .find(|x| x.location == coordinates)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_square_board_all_alive() {
        let board = GridBoard::all_alive(5);
        let x = 0;
        let y = 0;

        let x2 = 4;
        let y2 = 4;
        let cell = board.at(Coordinates { x, y });
        let cell2 = board.at(Coordinates { x: x2, y: y2 });
        assert_eq!(
            cell,
            Some(Cell {
                cell_state: CellState::Alive,
                location: Coordinates { x, y },
            })
        );
        assert_eq!(
            cell2,
            Some(Cell {
                cell_state: CellState::Alive,
                location: Coordinates { x: x2, y: y2 },
            })
        );
    }

    #[test]
    fn a_square_board_outside_edges_returns_none() {
        let board = GridBoard::all_alive(2);
        let outside_of_board = Coordinates { x: 3, y: 3 };
        let none_cell = board.at(outside_of_board);
        assert_eq!(none_cell, None);
    }
}
