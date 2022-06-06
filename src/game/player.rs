use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    White,
    Black,
    None,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}
