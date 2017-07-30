#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn is_neighbour(&self, other: &Coordinates) -> bool {
        match self {
            &Coordinates { x, y } if x == other.x - 1 || x == other.x || x == other.x + 1 => {
                y == other.y - 1 || y == other.y || y == other.y + 1
            }
            _ => false,
        }
    }

    pub fn neighbours(&self) -> Vec<Coordinates> {
        vec![
            Coordinates {
                x: self.x - 1,
                y: self.y - 1,
            },
            Coordinates {
                x: self.x - 1,
                y: self.y,
            },
            Coordinates {
                x: self.x - 1,
                y: self.y + 1,
            },
            Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            Coordinates {
                x: self.x + 1,
                y: self.y + 1,
            },
            Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            Coordinates {
                x: self.x + 1,
                y: self.y - 1,
            },
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Cell {
    pub cell_state: CellState,
    pub location: Coordinates,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CellState {
    Alive,
    Dead,
    Born,
    Died,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_neighbour_test() {
        let x = Coordinates { x: 0, y: 0 };
        assert!(x.is_neighbour(&Coordinates { x: 1, y: 0 }));
        assert!(!x.is_neighbour(&Coordinates { x: 2, y: 0 }));
    }

    #[test]
    fn find_neighbours_from_the_origin() {
        let coords = Coordinates { x: 0, y: 0 };
        let expected = vec![
            Coordinates { x: -1, y: -1 },
            Coordinates { x: -1, y: 0 },
            Coordinates { x: -1, y: 1 },
            Coordinates { x: 0, y: -1 },
            Coordinates { x: 0, y: 1 },
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 1, y: 0 },
            Coordinates { x: 1, y: -1 },
        ];

        let result = coords.neighbours();

        assert_eq!(expected, result);
    }
}
