use std::fmt;

use crate::Move;

/// An error returned when validating Move.
#[derive(Debug)]
pub enum IlegalMoveError {
    IlegalMove(Move),
}

impl std::error::Error for IlegalMoveError {}

impl fmt::Display for IlegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IlegalMove(s) => write!(f, "Invalid move: '{}'", s),
        }
    }
}
