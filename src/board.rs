pub trait Board: Clone {
    fn at(&self, coordiates: Coordinates) -> Option<&Cell>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Cell {
    pub cell_state: CellState,
    pub location: Coordinates,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CellState {
    Alive,
    Dead,
}

pub trait Renderer<T: Board> {
    fn render(&self, board: &T);
}

pub trait Generator<T: Board> {
    fn generate(&self, board: &T) -> T;
}
