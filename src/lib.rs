

// Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

mod board;
mod life {
    use board::Board;

    pub struct Game<'a, T: Board, U: 'a + Renderer<T>> {
        board: T,
        renderer: &'a U,
    }

    impl<'a, T: Board, U: Renderer<T>> Game<'a, T, U> {
        pub fn new(board: T, renderer: &'a U) -> Game<'a, T, U> {
            Game { board, renderer }
        }

        pub fn play(self, generations: u32) -> T {
            self.renderer.render(&self.board);

            for i in 0..generations {
                self.renderer.render(&self.board);
            }

            self.board
        }
    }

    pub trait Renderer<T: Board> {
        fn render(&self, board: &T);
    }
}

#[cfg(test)]
mod tests {
    use life::*;
    use board::*;
    #[test]
    fn zero_generations_returns_the_original_board() {
        let renderer = FakeRenderer::new();
        let seed_board = EmptyBoard::new();
        let game = Game::new(seed_board.clone(), &renderer);
        let result_board = game.play(0);
        assert_eq!(seed_board, result_board);
    }

    #[test]
    fn the_game_renders_the_board() {
        let renderer = FakeRenderer::new();
        let board = EmptyBoard::new();
        let game = Game::new(board.clone(), &renderer);
        let _ = game.play(0);

        assert_eq!(renderer.boards.borrow().first(), Some(&board));
    }

    #[test]
    fn the_game_renders_the_board_for_each_generation() {
        let renderer = FakeRenderer::new();
        let board = EmptyBoard::new();
        let game = Game::new(board.clone(), &renderer);
        let _ = game.play(500);

        assert_eq!(renderer.boards.borrow().len(), 501);
    }

    use std::cell::RefCell;
    struct FakeRenderer<T: Board> {
        boards: RefCell<Vec<T>>,
    }

    impl<T: Board> FakeRenderer<T> {
        fn new() -> FakeRenderer<T> {
            FakeRenderer {
                boards: RefCell::new(Vec::new()),
            }
        }
    }

    impl<T: Board> Renderer<T> for FakeRenderer<T> {
        fn render(&self, board: &T) {
            self.boards.borrow_mut().push(board.clone());
        }
    }
}
