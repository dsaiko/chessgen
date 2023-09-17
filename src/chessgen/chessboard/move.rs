use std::fmt::Display;

use crate::bitboard::Index;

use super::{InvalidMoveNotationError, Piece};

/// Chess Move definition.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    /// Index from which the piece moved.
    pub from: Index,
    /// Index to which the piece moved.
    pub to: Index,
    /// Promotion piece
    pub promotion: Option<Piece>,
}

impl Move {
    /// Constructs a new Move.
    #[inline(always)]
    #[must_use]
    pub const fn new(from: Index, to: Index, promotion: Option<Piece>) -> Self {
        Move {
            from,
            to,
            promotion,
        }
    }

    /// Operator allowing cast from Index to BitBoard using BitBoard::form() or Index::into().
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::Move;
    ///
    /// assert_eq!("a1c3", Move::from_string("a1c3").unwrap().to_string());
    /// assert_eq!("a7a8q", Move::from_string("a7a8q").unwrap().to_string());
    ///
    /// assert!(Move::from_string("").is_err());
    /// assert!(Move::from_string("123456").is_err());
    /// assert!(Move::from_string("1234").is_err());
    /// assert!(Move::from_string("a9a8q").is_err());
    /// assert!(Move::from_string("a7a8X").is_err());
    /// ```
    pub fn from_string(s: &str) -> Result<Self, InvalidMoveNotationError> {
        if !(4..=5).contains(&s.len()) {
            return Err(InvalidMoveNotationError::InvalidString(s.to_string()));
        }

        let Ok(from) = Index::from_string(&s[0..2]) else {
            return Err(InvalidMoveNotationError::InvalidString(s.to_string()));
        };
        let Ok(to) = Index::from_string(&s[2..4]) else {
            return Err(InvalidMoveNotationError::InvalidString(s.to_string()));
        };

        let mut promotion = None;
        if s.len() == 5 {
            let c = &s[4..5].chars().next().unwrap();
            let Ok((_, piece)) = Piece::from_char(*c) else {
                return Err(InvalidMoveNotationError::InvalidString(s.to_string()));
            };
            promotion = Some(piece);
        }

        Ok(Move::new(from, to, promotion))
    }
}

/// Display and to_string() for a Move.
///
/// # Examples
///
/// ```
/// use chessgen::{Index, Piece, Move};
///
/// assert_eq!(Move::new(Index::A1, Index::A3, None).to_string(), "a1a3");
/// assert_eq!(Move::new(Index::A7, Index::A8, Some(Piece::Queen)).to_string(), "a7a8q");
/// ```
impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)?;
        if let Some(promotion) = self.promotion {
            write!(f, "{}", promotion)?;
        }

        Ok(())
    }
}
