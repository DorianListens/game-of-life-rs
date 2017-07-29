use models::*;
use interface::{Board, Renderer};
use board::square::*;

pub struct ScreenRenderer {
    size: i32,
}

impl<T: Board> Renderer<T> for ScreenRenderer {
    fn render(&self, board: &T) {
        clear_screen();
        for row in self.rows(board) {
            for c in row {
                print!("{}", c);
            }
            println!(" ");
        }
    }
}

impl ScreenRenderer {
    pub fn new(size: i32) -> ScreenRenderer {
        ScreenRenderer {
            size
        }
    }

    fn rows<T: Board>(&self, board: &T) -> Vec<Vec<char>> {
        let mut rows = Vec::new();
        for x in 0..self.size {
            let mut row = Vec::new();
            for y in 0..self.size {
                row.push(cell_to_string(board.at(Coordinates { x, y })));
            }
            rows.push(row);
        }
        rows
    }
}

fn char_for_state(state: &CellState) -> char {
    match state {
        &CellState::Alive => '0',
        &CellState::Dead => ' ',
    }
}

fn cell_to_string(cell: Option<&Cell>) -> char {
    match cell {
        None => 'x',
        Some(cell) => char_for_state(&cell.cell_state),
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
