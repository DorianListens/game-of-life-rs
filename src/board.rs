use std::collections::HashMap;

pub trait Board: Clone {
    fn at(&self, coordiates: Coordinates) -> Option<&Cell>;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cell {
    pub cell_state: CellState,
    pub location: Coordinates,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EmptyBoard {}

impl EmptyBoard {
    pub fn new() -> EmptyBoard {
        EmptyBoard {}
    }
}

impl Board for EmptyBoard {
    fn at(&self, coordinates: Coordinates) -> Option<&Cell> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SquareBoard {
    cells: HashMap<Coordinates, Cell>,
    size: i32,
}

impl SquareBoard {
    pub fn new(size: i32) -> SquareBoard {
        SquareBoard {
            size: size,
            cells: HashMap::new(),
        }
    }

    pub fn all_alive(size: i32) -> SquareBoard {
        let mut map = HashMap::new();

        for x in 0..size {
            for y in 0..size {
                let location = Coordinates { x, y };
                let state = CellState::Alive;
                let cell = Cell {
                    cell_state: state,
                    location: location,
                };
                map.insert(location, cell);
            }
        }

        SquareBoard {
            size: size,
            cells: map,
        }
    }
}

impl Board for SquareBoard {
    fn at(&self, coordinates: Coordinates) -> Option<&Cell> {
        self.cells.get(&coordinates)
    }
}

#[cfg(test)]
mod tests {
    use board::*;

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