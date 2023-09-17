use super::{BitBoard, Index};

impl BitBoard {
    /// Empty BitBoard.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///     ").unwrap(),
    ///     BitBoard::EMPTY
    ///  );
    /// ```    
    pub const EMPTY: BitBoard = BitBoard::new(0);

    /// BitBoard with all bits (pieces) set.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///         x x x x x x x x
    ///     ").unwrap(),
    ///     BitBoard::UNIVERSE
    ///  );
    /// ```  
    pub const UNIVERSE: BitBoard = BitBoard::new(!0);

    /// BitBoard with only frame bits (pieces) set.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         x x x x x x x x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x - - - - - - x
    ///         x x x x x x x x
    ///     ").unwrap(),
    ///     BitBoard::FRAME
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const FRAME: BitBoard = BitBoard::new(BitBoard::RANK_1.state | BitBoard::RANK_8.state | BitBoard::FILE_A.state | BitBoard::FILE_H.state);

    /// Board's A File.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///         x - - - - - - -
    ///     ").unwrap(),
    ///     BitBoard::FILE_A
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const FILE_A: BitBoard = BitBoard::from_index_array(&[Index::A1, Index::A2, Index::A3, Index::A4, Index::A5, Index::A6, Index::A7, Index::A8]);

    /// Board's H File.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///         - - - - - - - x
    ///     ").unwrap(),
    ///     BitBoard::FILE_H
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const FILE_H: BitBoard = BitBoard::from_index_array(&[Index::H1, Index::H2, Index::H3, Index::H4, Index::H5, Index::H6, Index::H7, Index::H8]);

    /// Board's 1st Rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         x x x x x x x x
    ///     ").unwrap(),
    ///     BitBoard::RANK_1
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const RANK_1: BitBoard = BitBoard::from_index_array(&[Index::A1, Index::B1, Index::C1, Index::D1, Index::E1, Index::F1, Index::G1, Index::H1]);

    /// Board's 8th Rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         x x x x x x x x
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///         - - - - - - - -
    ///     ").unwrap(),
    ///     BitBoard::RANK_8
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const RANK_8: BitBoard = BitBoard::from_index_array(&[Index::A8, Index::B8, Index::C8, Index::D8, Index::E8, Index::F8, Index::G8, Index::H8]);

    /// Board's A1H8 diagonals.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         x - - - - - - x
    ///         - - - - - - x -
    ///         - - - - - x - -
    ///         - - - - x - - -
    ///         - - - x - - - -
    ///         - - x - - - - -
    ///         - x - - - - - -
    ///         x - - - - - - x
    ///     ").unwrap(),
    ///     BitBoard::A1H8[0] | BitBoard::A1H8[7] | BitBoard::A1H8[14]
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const A1H8: &'static[BitBoard] = &[
        BitBoard::from_index_array(&[Index::A8]),
        BitBoard::from_index_array(&[Index::A7, Index::B8]),
        BitBoard::from_index_array(&[Index::A6, Index::B7, Index::C8]),
        BitBoard::from_index_array(&[Index::A5, Index::B6, Index::C7, Index::D8]),
        BitBoard::from_index_array(&[Index::A4, Index::B5, Index::C6, Index::D7, Index::E8]),
        BitBoard::from_index_array(&[Index::A3, Index::B4, Index::C5, Index::D6, Index::E7, Index::F8]),
        BitBoard::from_index_array(&[Index::A2, Index::B3, Index::C4, Index::D5, Index::E6, Index::F7, Index::G8]),
        BitBoard::from_index_array(&[Index::A1, Index::B2, Index::C3, Index::D4, Index::E5, Index::F6, Index::G7, Index::H8]),
        BitBoard::from_index_array(&[Index::B1, Index::C2, Index::D3, Index::E4, Index::F5, Index::G6, Index::H7]),
        BitBoard::from_index_array(&[Index::C1, Index::D2, Index::E3, Index::F4, Index::G5, Index::H6]),
        BitBoard::from_index_array(&[Index::D1, Index::E2, Index::F3, Index::G4, Index::H5]),
        BitBoard::from_index_array(&[Index::E1, Index::F2, Index::G3, Index::H4]),
        BitBoard::from_index_array(&[Index::F1, Index::G2, Index::H3]),
        BitBoard::from_index_array(&[Index::G1, Index::H2]),
        BitBoard::from_index_array(&[Index::H1]),
    ];

    /// Board's A8H1 diagonals.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::BitBoard;
    ///
    /// assert_eq!(
    ///     BitBoard::from_string("
    ///         x - - - - - - x
    ///         - x - - - - - -
    ///         - - x - - - - -
    ///         - - - x - - - -
    ///         - - - - x - - -
    ///         - - - - - x - -
    ///         - - - - - - x -
    ///         x - - - - - - x
    ///     ").unwrap(),
    ///     BitBoard::A8H1[0] | BitBoard::A8H1[7] | BitBoard::A8H1[14]
    ///  );
    /// ```
    #[rustfmt::skip]
    pub const A8H1: &'static[BitBoard] = &[
        BitBoard::from_index_array(&[Index::A1]),
        BitBoard::from_index_array(&[Index::A2, Index::B1]),
        BitBoard::from_index_array(&[Index::A3, Index::B2, Index::C1]),
        BitBoard::from_index_array(&[Index::A4, Index::B3, Index::C2, Index::D1]),
        BitBoard::from_index_array(&[Index::A5, Index::B4, Index::C3, Index::D2, Index::E1]),
        BitBoard::from_index_array(&[Index::A6, Index::B5, Index::C4, Index::D3, Index::E2, Index::F1]),
        BitBoard::from_index_array(&[Index::A7, Index::B6, Index::C5, Index::D4, Index::E3, Index::F2, Index::G1]),
        BitBoard::from_index_array(&[Index::A8, Index::B7, Index::C6, Index::D5, Index::E4, Index::F3, Index::G2, Index::H1]),
        BitBoard::from_index_array(&[Index::B8, Index::C7, Index::D6, Index::E5, Index::F4, Index::G3, Index::H2]),
        BitBoard::from_index_array(&[Index::C8, Index::D7, Index::E6, Index::F5, Index::G4, Index::H3]),
        BitBoard::from_index_array(&[Index::D8, Index::E7, Index::F6, Index::G5, Index::H4]),
        BitBoard::from_index_array(&[Index::E8, Index::F7, Index::G6, Index::H5]),
        BitBoard::from_index_array(&[Index::F8, Index::G7, Index::H6]),
        BitBoard::from_index_array(&[Index::G8, Index::H7]),
        BitBoard::from_index_array(&[Index::H8]),
    ];
}
