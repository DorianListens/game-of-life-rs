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
    born: String,
    died: String,
    none: String,
}

impl StringTransformer {
    fn new() -> StringTransformer {
        StringTransformer {
            alive: String::from("o"),
            dead: String::from(" "),
            born: format!("{}O{}", color::Fg(color::Green), color::Fg(color::Reset)),
            died: format!("{}x{}", color::Fg(color::Red), color::Fg(color::Reset)),
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
            CellState::Born => &self.born,
            CellState::Died => &self.died,
        }
    }
}


#[cfg(test)]
mod tests {
    use models::CellState::*;
    use super::*;

    #[test]
    fn born_cells_are_green() {
        let location = Coordinates { x: 0, y: 0 };
        let cell = Cell {
            cell_state: Born,
            location,
        };
        let cells = vec![Some(cell)];
        let transformer = StringTransformer::new();
        let output = transformer.row_to_string(&cells);
        assert_eq!(
            output,
            format!("{}O{}", color::Fg(color::Green), color::Fg(color::Reset))
        );
    }

    #[test]
    fn living_cells_are_white() {
        let location = Coordinates { x: 0, y: 0 };
        let cell = Cell {
            cell_state: Alive,
            location,
        };
        let cells = vec![Some(cell)];
        let transformer = StringTransformer::new();
        let output = transformer.row_to_string(&cells);
        assert_eq!(output, "o");
    }

    #[test]
    fn dying_cells_are_red() {
        let location = Coordinates { x: 0, y: 0 };
        let cell = Cell {
            cell_state: Died,
            location,
        };
        let cells = vec![Some(cell)];
        let transformer = StringTransformer::new();
        let output = transformer.row_to_string(&cells);
        assert_eq!(
            output,
            format!("{}x{}", color::Fg(color::Red), color::Fg(color::Reset))
        );
    }

    #[test]
    fn dead_cells_are_blank() {
        let location = Coordinates { x: 0, y: 0 };
        let cell = Cell {
            cell_state: Dead,
            location,
        };
        let cells = vec![Some(cell)];
        let transformer = StringTransformer::new();
        let output = transformer.row_to_string(&cells);
        assert_eq!(output, " ");
    }
}
