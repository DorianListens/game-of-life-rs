

// Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
//

#![feature(conservative_impl_trait)]

mod life {
use std::collections::HashMap;
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

    pub trait Board {
        fn at(&self, coordiates: Coordinates) -> Option<&Cell>;
    }

    pub struct Game<'a, T: Board, U: 'a + Renderer<T>> {
        board: T,
        renderer: &'a mut U,
    }

    impl<'a, T: Board, U: Renderer<T>> Game<'a, T, U> {
        pub fn new(board: T, renderer: &'a mut U) -> Game<'a, T, U> {
            Game { board, renderer }
        }

        pub fn play(self, generations: u32) -> T {
            self.renderer.render(&self.board);
            self.board
        }
    }

    pub trait Renderer<T: Board> {
        fn render(&mut self, board: &T);
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
            SquareBoard { size: size, cells: HashMap::new() }
        }

        pub fn all_alive(size: i32) -> SquareBoard {
            let mut map = HashMap::new();

            for x in 0..size {
                for y in 0..size {
                    let location = Coordinates { x, y } ;
                    let state = CellState::Alive;
                    let cell = Cell { cell_state: state, location: location };
                    map.insert(location, cell);
                }
            }

            SquareBoard { size: size, cells: map }
        }
    }

    impl Board for SquareBoard {
        fn at(&self, coordinates: Coordinates) -> Option<&Cell> {
            self.cells.get(&coordinates)
        }
    }
}



#[cfg(test)]
mod tests {
    use life::*;
    #[test]
    fn zero_generations_returns_the_original_board() {
        let mut renderer = FakeRenderer::new();
        let seed_board = EmptyBoard::new();
        let game = Game::new(seed_board.clone(), &mut renderer);
        let result_board = game.play(0);
        assert_eq!(seed_board, result_board);
    }

    #[test]
    fn the_game_renders_the_board() {
        let mut renderer = FakeRenderer::new();
        let board = EmptyBoard::new();
        let game = Game::new(board.clone(), &mut renderer);
        let _ = game.play(0);

        assert_eq!(renderer.boards.first(), Some(&board));
    }

    struct FakeRenderer<T: Board> {
        boards: Vec<T>,
    }

    impl<T: Board> FakeRenderer<T> {
        fn new() -> FakeRenderer<T> {
            FakeRenderer { boards: Vec::new() }
        }
    }

    impl<T: Board> Renderer <T>for FakeRenderer<T> {
        fn render(&mut self, board: &T) {
            self.boards.push(*board.clone());
        }
    }

    #[test]
    fn a_square_board_all_alive() {
        let board = SquareBoard::all_alive(5);
        let x = 0;
        let y = 0;
        
        let x2 = 4;
        let y2 = 4;
        let cell = board.at(Coordinates { x, y });
        let cell2 = board.at(Coordinates { x: x2, y: y2 });
        assert_eq!(cell, Some( &Cell { cell_state: CellState::Alive, location: Coordinates { x, y } } ));
        assert_eq!(cell2, Some( &Cell { cell_state: CellState::Alive, location: Coordinates { x: x2, y: y2 } } ));
    }

    #[test]
    fn a_square_board_outside_edges_returns_none() {
        let board = SquareBoard::all_alive(2);
        let outside_of_board = Coordinates { x: 3, y: 3 };
        let none_cell = board.at(outside_of_board);
        assert_eq!(none_cell, None);
    }
}
