use std::fmt;

use crate::Move;

/// An error returned when validating Move.
#[derive(Debug)]
pub enum IllegalMoveError {
    IllegalMove(Move),
}

impl std::error::Error for IllegalMoveError {}

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IllegalMove(s) => write!(f, "Invalid move: '{}'", s),
        }
    }
}
