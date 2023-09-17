use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use super::{BitBoard, Index};

/// Operator allowing OR of two BitBoards.
///
/// # Examples
///
/// ```
/// use chessgen::BitBoard;
///
/// assert_eq!(
///     BitBoard::from_string("
///         x x x - - - - -
///         x x - - - - - -
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///     ").unwrap().mirrored_a8h1()
///     |    
///     BitBoard::from_string("
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///         - - - - - - x x
///     ").unwrap(),
///     BitBoard::from_string("
///         x x x - - - - -
///         x x - - - - - -
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///         - - - - - - x x
///     ").unwrap()
/// );    
/// ```    
impl BitOr for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.state | rhs.state)
    }
}

/// Operator allowing OR of two BitBoards.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b = BitBoard::EMPTY;
/// b |= Index::A1 | Index::H8;
///
/// assert_eq!(b.popcnt(), 2);
/// ```    
impl BitOrAssign for BitBoard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.state |= rhs.state;
    }
}

/// Operator allowing OR of BitBoard and Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let b = BitBoard::EMPTY | Index::A1;
///
/// assert_eq!(b.popcnt(), 1);
/// ```    
impl BitOr<Index> for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Index) -> Self::Output {
        Self::new(self.state | rhs.as_bitboard().state)
    }
}

/// Operator allowing OR of BitBoard and Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b = BitBoard::EMPTY;
/// b |= Index::A1;
///
/// assert_eq!(b.popcnt(), 1);
/// ```    
impl BitOrAssign<Index> for BitBoard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Index) {
        self.state |= rhs.as_bitboard().state;
    }
}

/// Operator allowing XOR of two BitBoards.
///
/// # Examples
///
/// ```
/// use chessgen::BitBoard;
///
/// assert_eq!(
///     BitBoard::from_string("
///         x x x - - - - -
///         x x - - - - - -
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///     ").unwrap().mirrored_a8h1()
///     ^    
///     BitBoard::from_string("
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///         - - - - - - x x
///     ").unwrap(),
///     BitBoard::from_string("
///         - x x - - - - -
///         x x - - - - - -
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///         - - - - - - x -
///     ").unwrap()
/// );    
/// ```    
impl BitXor for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.state ^ rhs.state)
    }
}

/// Operator allowing XOR of two BitBoards.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b =  Index::A1 | Index::A2 | Index::A3;
/// b ^= Index::A2.as_bitboard();
///
/// assert_eq!(b, Index::A1 | Index::A3);
/// ```    
impl BitXorAssign for BitBoard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.state ^= rhs.state;
    }
}

/// Operator allowing XOR of BitBoard and Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b = (Index::A1 | Index::A2 | Index::A3) ^ Index::A2;
///
/// assert_eq!(b, Index::A1 | Index::A3);
/// ```   
impl BitXor<Index> for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Index) -> Self::Output {
        Self::new(self.state ^ rhs.as_bitboard().state)
    }
}

/// Operator allowing XOR of BitBoard and Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b =  Index::A1 | Index::A2 | Index::A3;
/// b ^= Index::A2;
///
/// assert_eq!(b, Index::A1 | Index::A3);
/// ```  
impl BitXorAssign<Index> for BitBoard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Index) {
        self.state ^= rhs.as_bitboard().state;
    }
}

/// Operator allowing AND of two BitBoards.
///
/// # Examples
///
/// ```
/// use chessgen::BitBoard;
///
/// assert_eq!(
///     BitBoard::from_string("
///         x x x - - - - -
///         x x - - - - - -
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///     ").unwrap().mirrored_a8h1()
///     &    
///     BitBoard::from_string("
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///         - - - - - - x x
///     ").unwrap(),
///     BitBoard::from_string("
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///     ").unwrap()
/// );    
/// ```    
impl BitAnd for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.state & rhs.state)
    }
}

/// Operator allowing AND of two BitBoards.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b =  Index::A1 | Index::A2 | Index::A3;
/// b &= Index::A2.as_bitboard();
///
/// assert_eq!(b, Index::A2.as_bitboard());
/// ```
impl BitAndAssign for BitBoard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.state &= rhs.state;
    }
}

/// Operator allowing AND of BitBoard and Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let b =  (Index::A1 | Index::A2 | Index::A3) & Index::A2;
///
/// assert_eq!(b, Index::A2.as_bitboard());
/// ```
impl BitAnd<Index> for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Index) -> Self::Output {
        Self::new(self.state & rhs.as_bitboard().state)
    }
}

/// Operator allowing AND of two BitBoard and Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let mut b =  Index::A1 | Index::A2 | Index::A3;
/// b &= Index::A2;
///
/// assert_eq!(b, Index::A2.as_bitboard());
/// ```
impl BitAndAssign<Index> for BitBoard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Index) {
        self.state &= rhs.as_bitboard().state;
    }
}

/// Not operator for BitBoard.
///
/// # Examples
///
/// ```
/// use chessgen::BitBoard;
///
/// assert_eq!(
///     !BitBoard::from_string("
///         x x x - - - - -
///         x x - - - - - -
///         x - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - -
///         - - - - - - - x
///     ").unwrap().mirrored_a8h1(),
///     BitBoard::from_string("
///         - - - x x x x x
///         - - x x x x x x
///         - x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x -
///     ").unwrap()
/// );    
/// ```    
impl Not for BitBoard {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self::new(!self.state)
    }
}

/// Not operator for Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// assert_eq!(
///     !Index::A1,
///     BitBoard::from_string("
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         x x x x x x x x
///         - x x x x x x x
///     ").unwrap()
/// );    
/// ```    
impl Not for Index {
    type Output = BitBoard;

    #[inline(always)]
    fn not(self) -> Self::Output {
        BitBoard::new(!self.as_bitboard().state)
    }
}

/// Operator allowing OR of two Indices.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let b = Index::A1 | Index::A2;
/// ```
impl BitOr for Index {
    type Output = BitBoard;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard::new(self.as_bitboard().state | rhs.as_bitboard().state)
    }
}

/// Operator allowing OR of BitBoard and Indices.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let b = Index::A1 | BitBoard::EMPTY;
/// ```
impl BitOr<BitBoard> for Index {
    type Output = BitBoard;

    #[inline(always)]
    fn bitor(self, rhs: BitBoard) -> Self::Output {
        BitBoard::new(self.as_bitboard().state | rhs.state)
    }
}

/// Operator allowing cast from Index to BitBoard using BitBoard::form() or Index::into().
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let b: BitBoard = Index::A1.into();
/// ```
impl From<Index> for BitBoard {
    fn from(value: Index) -> Self {
        value.as_bitboard()
    }
}

/// Operator allowing substraction a number from Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let i = Index::B1 - 1;
/// assert_eq!(Index::A1, i);
/// ```
impl std::ops::Sub<usize> for Index {
    type Output = Index;

    fn sub(self, rhs: usize) -> Self::Output {
        Index::new(self.index - rhs)
    }
}

/// Operator allowing adding a number to Index.
///
/// # Examples
///
/// ```
/// use chessgen::{BitBoard, Index};
///
/// let i = Index::A1 + 1;
/// assert_eq!(Index::B1, i);
/// ```
impl std::ops::Add<usize> for Index {
    type Output = Index;

    fn add(self, rhs: usize) -> Self::Output {
        Index::new(self.index + rhs)
    }
}
