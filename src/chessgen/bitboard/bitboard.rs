use std::fmt;

use super::{Index, InvalidBitBoardStringError};

/// Bit representation of board of 64 pieces.
///
/// See: [ChessProgramming Bitboards](https://www.chessprogramming.org/Bitboards)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BitBoard {
    /// Unsigned 64 bit representation of state of 64 squares on the board.
    ///
    /// If bit is set - piece is present on the field of given position.
    pub state: u64,
}

impl BitBoard {
    /// Constructs a new BitBoard on top of 64 bit number.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::new(0b10111),
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x x x - x - - -
    ///     ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn new(state: u64) -> Self {
        BitBoard { state }
    }

    /// Constructs a new BitBoard from constant array of Indices.
    /// This constructor can be used to create constant bit boards.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// assert_eq!(
    ///     BitBoard::from_index_array(&[Index::A1, Index::A8]),
    ///     BitBoard::from_string("
    ///         x - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x - - - - - - -
    ///     ").unwrap());
    /// ```
    pub const fn from_index_array(indices: &[Index]) -> Self {
        let mut i = 0;
        let mut b = 0u64;

        while i < indices.len() {
            b |= indices[i].as_bitboard().state;
            i += 1;
        }

        BitBoard::new(b)
    }

    /// Constructs a new BitBoard from vector of Indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// assert_eq!(
    ///     BitBoard::from_indices(vec![Index::A1, Index::A8]),
    ///     BitBoard::from_string("
    ///         x - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x - - - - - - -
    ///     ").unwrap());
    /// ```
    #[must_use]
    pub fn from_indices(indices: Vec<Index>) -> Self {
        let mut b = BitBoard::EMPTY;

        for i in indices {
            b |= i.as_bitboard();
        }
        b
    }

    /// Converts this board to list of Indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// assert_eq!(
    ///     vec![Index::A1, Index::H1, Index::A8, Index::H8],
    ///     BitBoard::from_string("
    ///         x - - - - - - x
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x - - - - - - x
    ///     ").unwrap().to_indices());
    /// ```
    #[must_use]
    pub fn to_indices(self) -> Vec<Index> {
        let mut v: Vec<Index> = Vec::with_capacity(self.popcnt());

        let mut b = self;

        while let (Some(i), next) = b.bitpop() {
            b = next;
            v.push(i);
        }
        v
    }

    /// Returns of pieces (bits) on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(BitBoard::UNIVERSE.popcnt(), 64);
    /// assert_eq!(BitBoard::EMPTY.popcnt(), 0);
    ///
    /// assert_eq!(
    ///     8,
    ///     BitBoard::from_string("
    ///         x - - - - - - x
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - x x - - -
    ///         - - - x x - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x - - - - - - x
    ///     ").unwrap().popcnt());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn popcnt(self) -> usize {
        self.state.count_ones() as usize
    }

    /// Returns index of first non-empty bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// assert_eq!(BitBoard::EMPTY.bitscan(), None);
    ///
    /// assert_eq!(
    ///     Some(Index::D1),
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - x - - -
    ///         - - - x - - - -
    ///     ").unwrap().bitscan());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn bitscan(self) -> Option<Index> {
        if self.state == 0 {
            return None;
        }
        Some(Index::new(self.state.trailing_zeros() as usize))
    }

    /// Returns true if bit is set at position of Index.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// let b = BitBoard::from_string("
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - x - - -
    ///         - - - x - - - -
    ///  ").unwrap();
    ///
    /// assert!(b.has_bit(Index::D1));
    /// assert!(b.has_bit(Index::E2));
    /// assert!(!b.has_bit(Index::A1));
    /// ```
    pub const fn has_bit(self, i: Index) -> bool {
        (self.state & i.as_bitboard().state) != 0
    }

    /// Returns index of first non empty bit position and resets this bit in the returned board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, Index};
    ///
    /// let (mut i, mut b) = BitBoard::EMPTY.bitpop();
    /// assert_eq!(i, None);
    /// assert_eq!(b, BitBoard::EMPTY);
    ///
    /// b = BitBoard::from_string("
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///     ").unwrap();
    ///
    /// (i, b) = b.bitpop();
    ///
    /// assert_eq!(
    ///     b,
    ///     BitBoard::from_string("
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         - x x x x x x x
    ///     ").unwrap());
    ///
    /// (i, b) = b.bitpop();
    /// assert_eq!(
    ///     b,
    ///     BitBoard::from_string("
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         - - x x x x x x
    ///     ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn bitpop(self) -> (Option<Index>, Self) {
        if self.state == 0 {
            return (None, BitBoard::EMPTY);
        }

        let b = self.state;

        (self.bitscan(), BitBoard::new(b & (b - 1)))
    }

    /// Shifts all existing Board pieces north by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_north(),
    ///     BitBoard::from_string("
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x x x x x x x x
    ///         - - - - - - - -
    ///     ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_north(self) -> Self {
        BitBoard::new(self.state << 8)
    }

    /// Shifts all existing Board pieces south by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_south(),
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         x x x x x x x x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///     ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_south(self) -> Self {
        BitBoard::new(self.state >> 8)
    }

    /// Shifts all existing Board pieces east by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_east(),
    ///     BitBoard::from_string("
    ///         - x x x x x x x
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x x x x x x x
    ///     ").unwrap());        
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_east(self) -> Self {
        BitBoard::new(self.state << 1 & !BitBoard::FILE_A.state)
    }

    /// Shifts all existing Board pieces north-east by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_northeast(),
    ///     BitBoard::from_string("
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x x x x x x x
    ///         - - - - - - - -
    ///     ").unwrap());    
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_northeast(self) -> Self {
        BitBoard::new(self.state << 9 & !BitBoard::FILE_A.state)
    }

    /// Shifts all existing Board pieces south-east by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_southeast(),
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         - x x x x x x x
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///         - x - - - - - -
    ///     ").unwrap());    
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_southeast(self) -> Self {
        BitBoard::new(self.state >> 7 & !BitBoard::FILE_A.state)
    }

    /// Shifts all existing Board pieces west by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_west(),
    ///     BitBoard::from_string("
    ///         x x x x x x x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         x x x x x x x -
    ///     ").unwrap());    
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_west(self) -> Self {
        BitBoard::new(self.state >> 1 & !BitBoard::FILE_H.state)
    }

    /// Shifts all existing Board pieces south-west by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_southwest(),
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         x x x x x x x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///     ").unwrap());    
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_southwest(self) -> Self {
        BitBoard::new(self.state >> 9 & !BitBoard::FILE_H.state)
    }

    /// Shifts all existing Board pieces south-west by one.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted_northwest(),
    ///     BitBoard::from_string("
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         - - - - - - x -
    ///         x x x x x x x -
    ///         - - - - - - - -
    ///     ").unwrap());    
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn shifted_northwest(self) -> Self {
        BitBoard::new(self.state << 7 & !BitBoard::FILE_H.state)
    }

    /// Shifts all existing Board pieces by multiple fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted(2, 2),
    ///     BitBoard::from_string("
    ///         - - x - - - - -   
    ///         - - x - - - - -   
    ///         - - x - - - - -   
    ///         - - x - - - - -   
    ///         - - x - - - - -   
    ///         - - x x x x x x   
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///     ").unwrap());    
    ///
    /// assert_eq!(
    ///     BitBoard::FRAME.shifted(-2, -2),
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x x x x x x - -
    ///         - - - - - x - -
    ///         - - - - - x - -
    ///         - - - - - x - -
    ///         - - - - - x - -
    ///         - - - - - x - -
    ///     ").unwrap());    
    /// ```
    #[must_use]
    pub const fn shifted(self, dx: isize, dy: isize) -> Self {
        let mut b = self.state;

        // dy = up/down
        if dy > 0 {
            b <<= dy * 8;
        }
        if dy < 0 {
            b >>= (-dy) * 8;
        }

        // dx = left / right
        if dx > 0 {
            let mut i = 0usize;
            while i < dx as usize {
                b = (b << 1) & !BitBoard::FILE_A.state;
                i += 1;
            }
        }
        if dx < 0 {
            let mut i = 0usize;
            while i < (-dx) as usize {
                b = (b >> 1) & !BitBoard::FILE_H.state;
                i += 1;
            }
        }

        BitBoard::new(b)
    }

    /// Returns this board mirrored vertically.
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
    ///     ").unwrap().mirrored_vertically(),    
    ///     BitBoard::from_string("
    ///         - - - - - - - x
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x - - - - - - -
    ///         x x - - - - - -
    ///         x x x - - - - -
    ///     ").unwrap());    
    /// ```    
    #[must_use]
    pub const fn mirrored_vertically(self) -> Self {
        let mut result = 0u64;
        let b = self.state;

        result |= (b >> 56) & BitBoard::RANK_1.state;
        result |= ((b >> 48) & BitBoard::RANK_1.state) << 8;
        result |= ((b >> 40) & BitBoard::RANK_1.state) << 16;
        result |= ((b >> 32) & BitBoard::RANK_1.state) << 24;
        result |= ((b >> 24) & BitBoard::RANK_1.state) << 32;
        result |= ((b >> 16) & BitBoard::RANK_1.state) << 40;
        result |= ((b >> 8) & BitBoard::RANK_1.state) << 48;
        result |= (b & BitBoard::RANK_1.state) << 56;

        BitBoard::new(result)
    }

    /// Returns this board mirrored horizontally.
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
    ///     ").unwrap().mirrored_horizontally(),    
    ///     BitBoard::from_string("
    ///         - - - - - x x x
    ///         - - - - - - x x
    ///         - - - - - - - x
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x - - - - - - -
    ///     ").unwrap());    
    /// ```    
    #[must_use]
    pub const fn mirrored_horizontally(self) -> Self {
        const K1: u64 = 0x5555555555555555u64;
        const K2: u64 = 0x3333333333333333u64;
        const K4: u64 = 0x0f0f0f0f0f0f0f0fu64;

        let mut b = self.state;

        b = ((b >> 1) & K1) | ((b & K1) << 1);
        b = ((b >> 2) & K2) | ((b & K2) << 2);
        b = ((b >> 4) & K4) | ((b & K4) << 4);

        BitBoard::new(b)
    }

    /// Returns bitboard flipped around A1H8 diagonal.
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
    ///         - - - - x x x x
    ///     ").unwrap().mirrored_a1h8(),    
    ///     BitBoard::from_string("
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - x
    ///         - - - - - - x x
    ///         - - - - - x x x
    ///     ").unwrap());    
    /// ```    
    #[must_use]
    pub const fn mirrored_a1h8(self) -> Self {
        const K1: u64 = 0x5500550055005500u64;
        const K2: u64 = 0x3333000033330000u64;
        const K4: u64 = 0x0f0f0f0f00000000u64;

        let mut b = self.state;

        let mut t = K4 & (b ^ (b << 28));

        b ^= t ^ (t >> 28);
        t = K2 & (b ^ (b << 14));
        b ^= t ^ (t >> 14);
        t = K1 & (b ^ (b << 7));
        b ^= t ^ (t >> 7);

        BitBoard::new(b)
    }

    /// Returns bitboard flipped around A8H1 diagonal.
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
    ///         - - - - x x x x
    ///     ").unwrap().mirrored_a8h1(),    
    ///     BitBoard::from_string("
    ///         x x x - - - - -
    ///         x x - - - - - -
    ///         x - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///     ").unwrap());    
    /// ```    
    #[must_use]
    pub const fn mirrored_a8h1(self) -> Self {
        const K1: u64 = 0xaa00aa00aa00aa00u64;
        const K2: u64 = 0xcccc0000cccc0000u64;
        const K4: u64 = 0xf0f0f0f00f0f0f0fu64;

        let mut b = self.state;
        let mut t = b ^ (b << 36);

        b ^= K4 & (t ^ (b >> 36));
        t = K2 & (b ^ (b << 18));
        b ^= t ^ (t >> 18);
        t = K1 & (b ^ (b << 9));
        b ^= t ^ (t >> 9);

        BitBoard::new(b)
    }

    /// Creates new board form a string.
    /// String may or may not be decorated by coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         1 2 3 4 5 6 7 8
    ///       h x x x - - - - - h
    ///       g x x - - - - - - g
    ///       f x - - - - - - - f
    ///       e - - - - - - - - e
    ///       d - - - - - - - - d
    ///       c - - - - - - - - c
    ///       b - - - - - - - - b
    ///       a - - - - x x x x a
    ///         1 2 3 4 5 6 7 8
    ///     ").unwrap(),    
    ///     BitBoard::from_string("
    ///         x x x - - - - -
    ///         x x - - - - - -
    ///         x - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - x x x x
    ///     ").unwrap());    
    /// ```    
    pub fn from_string(str: &str) -> Result<Self, InvalidBitBoardStringError> {
        let mut b = BitBoard::EMPTY;

        let mut pieces = str.to_string().to_ascii_lowercase();
        pieces.retain(|c| !"abcdefgh00123456789 \n".contains(c));

        if pieces.len() != Index::ALL_FIELDS.len() {
            return Err(InvalidBitBoardStringError::InvalidString(str.to_string()));
        }

        for (i, c) in pieces.chars().enumerate() {
            match c {
                'x' => b |= Index::new(i).as_bitboard(),
                '-' => {}
                _ => return Err(InvalidBitBoardStringError::InvalidString(str.to_string())),
            }
        }

        Ok(b.mirrored_vertically())
    }
}

/// Display and to_string() for a BitBoard.
///
/// # Examples
///
/// ```
/// use chessgen::BitBoard;
/// let b = BitBoard::from_string("
///         1 2 3 4 5 6 7 8
///       h x x x - - - - - h
///       g x x - - - - - - g
///       f x - - - - - - - f
///       e - - - - - - - - e
///       d - - - - - - - - d
///       c - - - - - - - - c
///       b - - - - - - - - b
///       a - - - - x x x x a
///         1 2 3 4 5 6 7 8
///  ").unwrap();
///
/// assert_eq!(
///     b,
///     BitBoard::from_string(b.to_string().as_str()).unwrap()
/// );    
/// ```
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER: &str = "  a b c d e f g h\n";
        write!(f, "{}", HEADER)?;

        let b = self.mirrored_vertically();

        for i in 0..Index::ALL_FIELDS.len() {
            if i % 8 == 0 {
                if i > 0 {
                    // print right column digit
                    writeln!(f, "{}", 9 - (i / 8))?;
                }

                // print left column digit
                write!(f, "{} ", 8 - (i / 8))?;
            }

            if b.has_bit(Index::new(i)) {
                write!(f, "x ")?;
            } else {
                write!(f, "- ")?;
            }
        }

        write!(f, "1\n{}", HEADER)?;

        Ok(())
    }
}
