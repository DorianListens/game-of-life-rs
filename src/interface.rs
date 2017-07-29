pub use models::{Cell, Coordinates};

pub trait Board: Clone {
    fn at(&self, coordiates: Coordinates) -> Option<Cell>;
}

pub trait Renderer<T: Board> {
    fn render(&self, board: &T);
}

pub trait Generator<T: Board> {
    fn generate(&self, board: &T) -> T;
}
