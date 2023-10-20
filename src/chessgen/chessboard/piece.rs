use std::fmt;
use std::ops::Deref;

use super::{Color, InvalidPieceNotationError};

/// Chess piece representation.
/// Piece without color information.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Piece {
    /// King.
    King = 0,
    /// Queen.
    Queen,
    /// Bishop.
    Bishop,
    /// Knight.
    Knight,
    /// Rook.
    Rook,
    /// Pawn.
    Pawn,
}

// Dereference Color into usize
impl Deref for Piece {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl Piece {
    /// Array of possible Piece values.
    pub const VALUES: [Self; 6] = [
        Piece::King,
        Piece::Queen,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook,
        Piece::Pawn,
    ];

    /// Creates piece from a char and return it's color.
    /// Piece is one char from 'kqbnrpk', or 'KQBNRPK'.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{Color, Piece};
    ///
    /// assert_eq!(Piece::from_char('P').unwrap(), (Color::White, Piece::Pawn));
    /// assert_eq!(Piece::from_char('k').unwrap(), (Color::Black, Piece::King));
    /// assert!(Piece::from_char('?').is_err());
    /// ```
    pub fn from_char(p: char) -> Result<(Color, Self), InvalidPieceNotationError> {
        let c = match p.is_lowercase() {
            true => Color::Black,
            false => Color::White,
        };

        match p.to_ascii_lowercase() {
            'k' => Ok((c, Piece::King)),
            'q' => Ok((c, Piece::Queen)),
            'b' => Ok((c, Piece::Bishop)),
            'n' => Ok((c, Piece::Knight)),
            'r' => Ok((c, Piece::Rook)),
            'p' => Ok((c, Piece::Pawn)),
            _ => Err(InvalidPieceNotationError::InvalidPiece(c.to_string())),
        }
    }

    /// Returns char representing piece in given color.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{Color, Piece};
    ///
    /// assert_eq!(Piece::Pawn.to_char(Color::Black), 'p');
    /// assert_eq!(Piece::Pawn.to_char(Color::White), 'P');
    /// ```
    pub const fn to_char(self, color: Color) -> char {
        let c = match self {
            Piece::King => 'k',
            Piece::Queen => 'q',
            Piece::Bishop => 'b',
            Piece::Knight => 'n',
            Piece::Rook => 'r',
            Piece::Pawn => 'p',
        };

        match color {
            Color::White => c.to_ascii_uppercase(),
            Color::Black => c,
        }
    }
}

/// Display and to_string() for a Piece.
/// Outputs lower-case characters.
///
/// # Examples
///
/// ```
/// use chessgen::Piece;
///
/// assert_eq!(Piece::Pawn.to_string(), "p");
/// assert_eq!(Piece::Knight.to_string(), "n");
/// ```
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char(Color::Black))
    }
}
