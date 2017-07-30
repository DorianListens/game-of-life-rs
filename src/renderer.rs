use models::*;
use interface::{Board, Renderer};
use termion::clear;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::color;
use std::io::{Write, Stdout, stdout};
use std::cell::RefCell;

pub struct ScreenRenderer {
    stdout: Box<RefCell<Write>>,
    width: u16,
    height: u16,
}

impl<T: Board> Renderer<T> for ScreenRenderer {
    fn render(&self, board: &T) {
        let screen = self.rows(board).into_iter().map(row_to_string).collect::<Vec<_>>();
        let mut writer = self.stdout.borrow_mut();
        write!(&mut writer,
               "{}{}{}",
               clear::All,
               cursor::Goto(1, 1),
               cursor::Hide,
               ).unwrap();

        for row in screen {
            write!(&mut writer, "{}", row); 
        }

        writer.flush().unwrap();
    }
}

impl ScreenRenderer {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> ScreenRenderer {
        let term = stdout.into_raw_mode().unwrap();
        ScreenRenderer { stdout: Box::new(RefCell::new(term)), width, height }
    }

    fn rows<T: Board>(&self, board: &T) -> Vec<Vec<Option<Cell>>> {
        let mut rows = Vec::new();
        for x in 0..self.width {
            let mut row = Vec::new();
            for y in 0..self.height {
                row.push(board.at(Coordinates { x: x.into(), y: y.into() }));
            }
            rows.push(row);
        }
        rows
    }
}

fn row_to_string(cells: Vec<Option<Cell>>) -> String {
    let mut string = String::with_capacity(cells.len());
    for c in cells.into_iter().map(cell_to_str) {
        string.push_str(&c);
    }
    string
}

fn cell_to_str(cell: Option<Cell>) -> String {
    match cell {
        None => String::from("x"),
        Some(cell) => state_to_str(cell.cell_state),
    }
}

fn state_to_str(state: CellState) -> String {
    match state {
        CellState::Alive => format!("{}0{}", color::Fg(color::Green), color::Fg(color::Reset)),
        CellState::Dead => String::from(" "),
    }
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
