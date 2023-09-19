use std::fmt;

use super::{BitBoard, InvalidIndexNotationError};

/// BitBoard field Index.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Index {
    /// Field Index.
    pub index: usize,
}

impl Index {
    /// Constructs a new Index.
    /// Index range for BitBoard is 0..64, if out-of range index is created,
    /// BitBoard operations may fail with undefined behaviour.
    ///
    /// Use Index:: constants rather than creating own Index.
    #[inline(always)]
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Index { index }
    }

    /// Constructs index object from notation, or Error if notation is outside of
    /// chess notation range "A1".."H8".
    ///
    /// /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// assert_eq!(Index::A1, Index::from_string("a1").unwrap());
    ///
    /// assert!(Index::from_string("T5").is_err());
    /// assert!(Index::from_string("").is_err());
    ///
    /// let all_notations: [&str; Index::ALL_FIELDS.len()] = [
    ///   "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2",
    ///   "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4",
    ///   "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6",
    ///   "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8",
    ///   "e8", "f8", "g8", "h8",
    /// ];
    /// for i in Index::ALL_FIELDS {
    ///     assert_eq!(i.to_string(), all_notations[i.index]);
    ///
    ///     assert_eq!(*i, Index::from_string(all_notations[i.index]).unwrap());
    /// }
    ///
    /// ```
    pub fn from_string(s: &str) -> Result<Self, InvalidIndexNotationError> {
        // validate
        if s.len() != 2 {
            return Err(InvalidIndexNotationError::InvalidNotation(s.to_string()));
        }

        let lower = s.to_ascii_lowercase();

        let n = lower.as_bytes();
        let c1 = n[0];
        let c2 = n[1];

        // validate first char
        if !(b'a'..=b'h').contains(&c1) {
            return Err(InvalidIndexNotationError::InvalidNotation(s.to_string()));
        }

        // validate second char
        if !(b'1'..=b'8').contains(&c2) {
            return Err(InvalidIndexNotationError::InvalidNotation(s.to_string()));
        }

        let i = (c1 - b'a') + ((c2 - b'1') << 3);

        Ok(Index::new(i as usize))
    }

    /// Constructs index object from rank and file.
    ///
    /// /// # Examples
    ///
    /// ```
    /// use chessgen::{Index};
    ///
    /// assert_eq!(Index::A8, Index::from_rank_and_file(7, 0));
    /// ```
    pub const fn from_rank_and_file(rank: usize, file: usize) -> Index {
        Index::new(rank * 8 + file)
    }

    /// File of the chess position represented by this index.
    ///
    /// /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    ///
    /// let d: [usize; Index::ALL_FIELDS.len()] = [
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    ///     0, 1, 2, 3, 4, 5, 6, 7,
    /// ];
    ///
    /// for i in Index::ALL_FIELDS {
    ///     assert_eq!(i.file(), d[i.index]);
    /// }
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn file(self) -> usize {
        self.index % 8
    }

    /// Rank of the chess position represented by this index.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    ///
    /// let d: [usize; Index::ALL_FIELDS.len()] = [
    ///     0, 0, 0, 0, 0, 0, 0, 0,
    ///     1, 1, 1, 1, 1, 1, 1, 1,
    ///     2, 2, 2, 2, 2, 2, 2, 2,
    ///     3, 3, 3, 3, 3, 3, 3, 3,
    ///     4, 4, 4, 4, 4, 4, 4, 4,
    ///     5, 5, 5, 5, 5, 5, 5, 5,
    ///     6, 6, 6, 6, 6, 6, 6, 6,
    ///     7, 7, 7, 7, 7, 7, 7, 7,
    /// ];
    /// for i in Index::ALL_FIELDS {
    ///     let f = i.rank();
    ///     assert_eq!(f, d[i.index]);
    /// }
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn rank(self) -> usize {
        self.index / 8
    }

    /// Returns BitBoard with index position bit set as true.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// let mut b = BitBoard::EMPTY;
    ///
    /// for i in Index::ALL_FIELDS {
    ///     assert_eq!(i.as_bitboard().popcnt(), 1);
    ///     b |= *i;
    /// }
    ///
    /// assert_eq!(b, BitBoard::UNIVERSE);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn as_bitboard(self) -> BitBoard {
        BitBoard::new(1 << self.index)
    }

    /// Abs distance to other index.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::Index;
    ///
    /// assert_eq!(1, Index::A1.distance_to(Index::B1));
    /// assert_eq!(8, Index::A2.distance_to(Index::A1));
    ///
    /// ```    
    #[inline(always)]
    #[must_use]
    pub const fn distance_to(self, other: Index) -> usize {
        if self.index > other.index {
            self.index - other.index
        } else {
            other.index - self.index
        }
    }

    /// Shift Index north by one.
    /// If index would be out of board, None is returned.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::Index;
    ///
    /// assert_eq!(Index::A1, Index::A2.shifted_south().unwrap());
    /// assert!(Index::A1.shifted_south().is_none());
    /// assert!(Index::H1.shifted_south().is_none());
    /// ```    
    #[inline(always)]
    #[must_use]
    pub const fn shifted_south(self) -> Option<Index> {
        if self.index < 8 {
            None
        } else {
            Some(Index::new(self.index - 8))
        }
    }

    /// Shift Index south by one.
    /// If index would be out of board, None is returned.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::Index;
    ///
    /// assert_eq!(Index::A8, Index::A7.shifted_north().unwrap());
    /// assert!(Index::A8.shifted_north().is_none());
    /// assert!(Index::H8.shifted_north().is_none());
    /// ```    
    #[inline(always)]
    #[must_use]
    pub const fn shifted_north(self) -> Option<Index> {
        if self.index + 8 >= 64 {
            None
        } else {
            Some(Index::new(self.index + 8))
        }
    }
}

/// Display and to_string() for an Index.
///
/// # Examples
///
/// ```
/// use chessgen::Index;
///
/// assert_eq!(Index::A1.to_string(), "a1");
/// assert_eq!(Index::H8.to_string(), "h8");
/// ```
impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + (self.file() as u8)) as char,
            1 + self.rank()
        )?;
        Ok(())
    }
}
