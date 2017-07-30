
// Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

extern crate rand;
extern crate rayon;
extern crate termion;

pub mod models;
mod engine;
pub mod interface;
pub mod board;
pub mod generator;
pub mod renderer;
use interface::{Board, Generator, Renderer};
use std::{thread, time};

pub struct Game<'a, T: Board, U: 'a + Renderer<T>, V: 'a + Generator<T>> {
    board: T,
    renderer: &'a U,
    generator: &'a V,
}

impl<'a, T: Board, U: Renderer<T>, V: Generator<T>> Game<'a, T, U, V> {
    pub fn new(board: T, renderer: &'a U, generator: &'a V) -> Game<'a, T, U, V> {
        Game {
            board,
            renderer,
            generator,
        }
    }

    pub fn play(self, generations: u32) -> T {
        self.play_with_delay(generations, time::Duration::from_millis(0))
    }

    pub fn play_with_delay(self, generations: u32, delay: time::Duration) -> T {
        self.renderer.render(&self.board);

        let mut b = self.board;
        for _ in 0..generations {
            thread::sleep(delay);
            b = self.generator.generate(&b);
            self.renderer.render(&b);
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interface::*;

    #[test]
    fn zero_generations_returns_the_original_board() {
        let generator = FakeGenerator::new();
        let renderer = FakeRenderer::new();
        let seed_board = FakeBoard::new(0);
        let game = Game::new(seed_board.clone(), &renderer, &generator);
        let result_board = game.play(0);

        assert_eq!(seed_board, result_board);
    }

    #[test]
    fn the_game_renders_the_board() {
        let generator = FakeGenerator::new();
        let renderer = FakeRenderer::new();
        let board = FakeBoard::new(0);
        let game = Game::new(board.clone(), &renderer, &generator);
        let _ = game.play(0);

        assert_eq!(renderer.boards.borrow().first(), Some(&board));
    }

    #[test]
    fn the_game_renders_the_board_for_each_generation() {
        let generator = FakeGenerator::new();
        let renderer = FakeRenderer::new();
        let board = FakeBoard::new(0);
        let game = Game::new(board.clone(), &renderer, &generator);
        let _ = game.play(500);

        assert_eq!(renderer.boards.borrow().len(), 501);
    }

    #[test]
    fn the_game_passes_the_board_to_the_generator() {
        let generator = FakeGenerator::new();
        let renderer = FakeRenderer::new();
        let board = FakeBoard::new(0);
        let game = Game::new(board.clone(), &renderer, &generator);
        let _ = game.play(1);

        assert_eq!(generator.boards.borrow().first(), Some(&board));
    }

    #[test]
    fn the_generator_creates_new_boards() {
        let generator = FakeGenerator::new();
        let renderer = FakeRenderer::new();
        let board = FakeBoard::new(0);
        let game = Game::new(board.clone(), &renderer, &generator);
        let result = game.play(1);

        assert_eq!(result, FakeBoard::new(1337));
    }

    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    struct FakeBoard {
        seed: i32,
        rows: Vec<Vec<Cell>>,
    }

    impl FakeBoard {
        fn new(seed: i32) -> FakeBoard {
            FakeBoard { seed, rows: vec![] }
        }
    }

    impl Board for FakeBoard {
        fn at(&self, coordinates: Coordinates) -> Option<Cell> {
            None
        }

        fn rows(&self) -> &Vec<Vec<Cell>> {
            &self.rows
        }
    }

    struct FakeGenerator<T: Board> {
        boards: RefCell<Vec<T>>,
    }

    impl<T: Board> FakeGenerator<T> {
        fn new() -> FakeGenerator<T> {
            FakeGenerator {
                boards: RefCell::new(Vec::new()),
            }
        }
    }

    impl Generator<FakeBoard> for FakeGenerator<FakeBoard> {
        fn generate(&self, board: &FakeBoard) -> FakeBoard {
            self.boards.borrow_mut().push(board.clone());
            FakeBoard::new(1337)
        }
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
