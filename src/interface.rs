pub use models::{Cell, Coordinates};

pub trait Board: Clone {
    fn at(&self, coordiates: Coordinates) -> Option<Cell>;
    fn rows(&self) -> &Vec<Vec<Cell>>;
}

pub trait Renderer<T: Board> {
    fn render(&self, board: &T);
}

pub trait Generator<T: Board> {
    fn generate(&self, board: &T) -> T;
}
