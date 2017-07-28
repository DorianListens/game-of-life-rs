use models::*;
use interface::{Board, Renderer};

pub struct ScreenRenderer {}
impl<T: Board> Renderer<T> for ScreenRenderer {
    fn render(&self, board: &T) {
    }
}
