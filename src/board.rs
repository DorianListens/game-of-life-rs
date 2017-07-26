use std::collections::HashMap;
use life::{Cell, CellState, Coordinates};

pub trait Board: Clone {
    fn at(&self, coordiates: Coordinates) -> Option<&Cell>;
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
