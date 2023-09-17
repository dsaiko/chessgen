use std::fmt;

/// An error returned when parsing a string into BitBoard.
#[derive(Debug)]
pub enum InvalidBitBoardStringError {
    InvalidString(String),
}

impl std::error::Error for InvalidBitBoardStringError {}

impl fmt::Display for InvalidBitBoardStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidString(notation) => {
                write!(f, "Invalid bit board string:\n{}", notation)
            }
        }
    }
}

/// An error returned when parsing a string into bitboard::Index.
#[derive(Debug)]
pub enum InvalidIndexNotationError {
    InvalidNotation(String),
}

impl std::error::Error for InvalidIndexNotationError {}

impl fmt::Display for InvalidIndexNotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidNotation(notation) => {
                write!(f, "Unable to parse index notation: '{}'.", notation)
            }
        }
    }
}
