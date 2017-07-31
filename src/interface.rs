pub use models::{Cell, Coordinates};

pub type Rows = Vec<Vec<Cell>>;

pub trait Board: Clone + From<Rows> + Sync {
    fn at(&self, coordiates: Coordinates) -> Option<Cell>;
    fn rows(&self) -> &Rows;
}

pub trait Renderer<T: Board> {
    fn render(&self, board: &T);
}

pub trait Generator<T: Board> {
    fn generate(&self, board: &T) -> T;
}
