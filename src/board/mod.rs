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

