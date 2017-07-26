use board::*;
use boards::square::SquareBoard;

struct SquareGenerator {}

impl Generator<SquareBoard> for SquareGenerator {
    fn generate(&self, board: &SquareBoard) -> SquareBoard {
        SquareBoard::new(0)
    }
}

fn find_neighbours(point: Coordinates) -> Vec<Coordinates> {
    vec![
        Coordinates {
            x: point.x - 1,
            y: point.y - 1,
        },
        Coordinates {
            x: point.x - 1,
            y: point.y,
        },
        Coordinates {
            x: point.x - 1,
            y: point.y + 1,
        },
        Coordinates {
            x: point.x,
            y: point.y - 1,
        },
        Coordinates {
            x: point.x,
            y: point.y + 1,
        },
        Coordinates {
            x: point.x + 1,
            y: point.y + 1,
        },
        Coordinates {
            x: point.x + 1,
            y: point.y,
        },
        Coordinates {
            x: point.x + 1,
            y: point.y - 1,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result = find_neighbours(coords);

        assert_eq!(expected, result);
    }
}
