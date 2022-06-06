use std::fmt;

#[derive(Clone, Copy)]
pub struct Chessboard {
    pub board: [[char; 8]; 8],
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.board {
            for y in x {
                write!(f, "{}", y)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}

impl Chessboard {
    pub fn new() -> Chessboard {
        Chessboard {
            board: [
                ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ],
        }
    }
}
