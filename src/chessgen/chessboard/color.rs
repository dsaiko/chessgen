use std::fmt;

use super::InvalidColorNotationError;

/// Chess piece color.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    /// White color.
    White = 0,
    /// Black color.
    Black,
}

impl Color {
    /// Array of possible color values.
    pub const VALUES: [Color; 2] = [Color::White, Color::Black];

    /// Creates color from char.
    /// 'w'/'W' for White, 'b'/'B' for Black.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::Color;
    ///
    /// assert_eq!(Color::from_char('w').unwrap(), Color::White);
    /// assert_eq!(Color::from_char('b').unwrap(), Color::Black);
    /// assert!(Color::from_char('?').is_err());
    /// ```
    pub fn from_char(c: char) -> Result<Self, InvalidColorNotationError> {
        match c {
            'w' | 'W' => Ok(Color::White),
            'b' | 'B' => Ok(Color::Black),
            _ => Err(InvalidColorNotationError::InvalidColor(c.to_string())),
        }
    }

    /// Returns color of opponent pieces.
    ///    
    /// # Examples
    ///
    /// ```
    /// use chessgen::Color;
    ///
    /// assert_eq!(Color::White.opponent(), Color::Black);
    /// ```
    pub const fn opponent(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Display and to_string() for Color.
///    
/// # Examples
///
/// ```
/// use chessgen::Color;
///
/// assert_eq!(Color::White.to_string(), "w");
/// assert_eq!(Color::Black.to_string(), "b");
/// ```
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}
