use models::*;
use interface::{Board, Renderer};
use board::square::*;

pub struct ScreenRenderer {
    size: i32,
}

impl<T: Board> Renderer<T> for ScreenRenderer {
    fn render(&self, board: &T) {
        clear_screen();
        let screen: String = self.rows(board).into_iter().map(row_to_string).collect();
        print!("{}", screen);
    }
}

impl ScreenRenderer {
    pub fn new(size: i32) -> ScreenRenderer {
        ScreenRenderer { size }
    }

    fn rows<T: Board>(&self, board: &T) -> Vec<Vec<Option<Cell>>> {
        let mut rows = Vec::new();
        for x in 0..self.size {
            let mut row = Vec::new();
            for y in 0..self.size {
                row.push(board.at(Coordinates { x, y }));
            }
            rows.push(row);
        }
        rows
    }
}

fn row_to_string(cells: Vec<Option<Cell>>) -> String {
    let mut string = String::with_capacity(cells.len());
    for c in cells.into_iter().map(cell_to_char) {
        string.push(c);
    }
    string.push('\n');
    string
}

fn cell_to_char(cell: Option<Cell>) -> char {
    match cell {
        None => 'x',
        Some(cell) => char_for_state(cell.cell_state),
    }
}

fn char_for_state(state: CellState) -> char {
    match state {
        CellState::Alive => '0',
        CellState::Dead => ' ',
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

#[cfg(test)]
mod tests {
    use models::CellState::*;
    use models::*;
    use super::*;

    #[test]
    fn it_translates_cells_to_strings() {
        let location = Coordinates { x: 0, y: 0 };
        let cell1 = Cell {
            cell_state: Alive,
            location,
        };
        let cell2 = Cell {
            cell_state: Dead,
            location,
        };
        let cells = vec![Some(cell1), Some(cell2), None];
        let output = row_to_string(cells);
        assert_eq!(output, "0 x\n");
    }
}
