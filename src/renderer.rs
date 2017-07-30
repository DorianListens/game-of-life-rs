use models::*;
use interface::{Board, Renderer};
use termion::cursor;
use termion::color;
use std::io::{stdout, Stdout, Write};
use std::cell::RefCell;

pub struct ScreenRenderer {
    stdout: Box<RefCell<Write>>,
    width: u16,
    height: u16,
    transformer: StringTransformer,
}

impl<T: Board> Renderer<T> for ScreenRenderer {
    fn render(&self, board: &T) {
        let screen = self.screen(board);

        let mut writer = self.stdout.borrow_mut();
        write!(&mut writer, "{}{}", cursor::Goto(0, 0), cursor::Hide,).expect("Couldn't write");

        writer.flush().unwrap();
        for row in screen {
            write!(&mut writer, "{}", row);
        }

        writer.flush().unwrap();
    }
}

impl ScreenRenderer {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> ScreenRenderer {
        ScreenRenderer {
            stdout: Box::new(RefCell::new(stdout)),
            width,
            height,
            transformer: StringTransformer::new(),
        }
    }

    fn screen<T: Board>(&self, board: &T) -> Vec<String> {
        board
            .rows()
            .into_iter()
            .map(|row| row.into_iter().map(|x| Some(*x)).collect())
            .map(|x| self.transformer.row_to_string(&x))
            .collect::<Vec<_>>()
    }
}

struct StringTransformer {
    alive: String,
    dead: String,
    none: String,
}

impl StringTransformer {
    fn new() -> StringTransformer {
        StringTransformer {
            alive: format!("{}0{}", color::Fg(color::Green), color::Fg(color::Reset)),
            dead: String::from(" "),
            none: String::from("x"),
        }
    }

    fn row_to_string(&self, cells: &Vec<Option<Cell>>) -> String {
        cells.iter().map(|x| self.cell_to_str(x)).collect()
    }

    fn cell_to_str(&self, cell: &Option<Cell>) -> &str {
        match *cell {
            None => &self.none,
            Some(cell) => self.state_to_str(&cell.cell_state),
        }
    }

    fn state_to_str(&self, state: &CellState) -> &str {
        match *state {
            CellState::Alive => &self.alive,
            CellState::Dead => &self.dead,
        }
    }
}


#[cfg(test)]
mod tests {
    use models::CellState::*;
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
        let transformer = StringTransformer::new();
        let output = transformer.row_to_string(&cells);
        assert_eq!(output, "0 x\n");
    }
}
