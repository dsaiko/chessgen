use std::fmt;

/// An error returned when parsing a FEN string into ChessBoard.
#[derive(Debug)]
pub enum InvalidFENStringError {
    InvalidString(String),
}

impl std::error::Error for InvalidFENStringError {}

impl fmt::Display for InvalidFENStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidString(s) => write!(f, "Invalid FEN string: '{}'", s),
        }
    }
}

/// An error returned when parsing a string into ChessBoard.
#[derive(Debug)]
pub enum InvalidChessBoardStringError {
    InvalidString(String),
}

impl std::error::Error for InvalidChessBoardStringError {}

impl fmt::Display for InvalidChessBoardStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidString(s) => write!(f, "Invalid chessboard string:\n{}", s),
        }
    }
}

/// An error returned when parsing a string into Piece.
#[derive(Debug)]
pub enum InvalidPieceNotationError {
    InvalidPiece(String),
}

impl std::error::Error for InvalidPieceNotationError {}

impl fmt::Display for InvalidPieceNotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPiece(s) => write!(f, "Invalid chess piece notation: '{}'", s),
        }
    }
}

/// An error returned when parsing a string into Color.
#[derive(Debug)]
pub enum InvalidColorNotationError {
    InvalidColor(String),
}

impl std::error::Error for InvalidColorNotationError {}

impl fmt::Display for InvalidColorNotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidColor(s) => write!(f, "Invalid color notation: '{}'", s),
        }
    }
}

/// An error returned when parsing move notation string into Move.
#[derive(Debug)]
pub enum InvalidMoveNotationError {
    InvalidString(String),
}

impl std::error::Error for InvalidMoveNotationError {}

impl fmt::Display for InvalidMoveNotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidString(s) => write!(f, "Invalid Move notation: '{}'", s),
        }
    }
}
