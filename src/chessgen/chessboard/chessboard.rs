use std::fmt;
use std::fmt::Write;

use crate::{BitBoard, Generator, IlegalMoveError, Index};

use super::{Color, InvalidChessBoardStringError, InvalidFENStringError, Move, Piece};

/// ChessBoard representation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ChessBoard {
    /// BitBoards array of pieces.
    pub pieces: [[BitBoard; Piece::VALUES.len()]; Color::VALUES.len()],
    /// Color to move.
    pub next_move: Color,
    /// Boolean array of castling options.
    /// Only [Piece::King] and [Piece::Queen] is used.
    pub castling_options: [[bool; 2]; Color::VALUES.len()],
    /// En-Passant target or none.
    pub en_passant_target: Option<Index>,
    /// Half move clock.
    pub half_move_clock: usize,
    /// Full mobe number.
    pub full_move_number: usize,
    /// Piece array [Index] for caching piece on a field.
    piece_cache: [Option<(Color, Piece)>; Index::ALL_FIELDS.len()],
}

/// Constructs a new empty ChessBoard.
impl Default for ChessBoard {
    fn default() -> Self {
        ChessBoard::EMPTY
    }
}

impl ChessBoard {
    // FEN definition of standard chessboard layout.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// assert_eq!(
    /// ChessBoard::from_fen(ChessBoard::STANDARD_BOARD_FEN).unwrap(),
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap());
    /// ```
    pub const STANDARD_BOARD_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// New empty chessboard.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// assert_eq!(
    /// ChessBoard::EMPTY,
    /// ChessBoard::from_string("
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    /// ").unwrap());
    /// ```
    pub const EMPTY: ChessBoard = ChessBoard::new();

    /// New chessboard with a standard layout.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// assert_eq!(
    /// ChessBoard::STANDARD,
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap());
    /// ```
    pub const STANDARD: ChessBoard = ChessBoard::new_standard_board();

    /// Constructs a new empty ChessBoard.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{Color, ChessBoard};
    ///
    /// let board = ChessBoard::EMPTY;
    ///
    /// assert_eq!(board.pieces(Color::White).popcnt(), 0);
    /// assert_eq!(board.pieces(Color::Black).popcnt(), 0);
    /// ```
    #[must_use]
    const fn new() -> Self {
        ChessBoard {
            pieces: [[BitBoard::EMPTY; Piece::VALUES.len()]; Color::VALUES.len()],
            next_move: Color::White,
            castling_options: [[false; 2]; Color::VALUES.len()],
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            piece_cache: [None; Index::ALL_FIELDS.len()],
        }
    }

    /// Creates a new chessboard initialized to the standard layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// board,
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap());
    /// ```
    #[must_use]
    const fn new_standard_board() -> Self {
        // note: following code could be written in better way using traits operators,
        // if the function was not const

        let mut pieces = [[BitBoard::EMPTY; Piece::VALUES.len()]; Color::VALUES.len()];

        pieces[Color::White as usize][Piece::King as usize] = Index::E1.as_bitboard();
        pieces[Color::White as usize][Piece::Queen as usize] = Index::D1.as_bitboard();
        pieces[Color::White as usize][Piece::Bishop as usize] =
            BitBoard::new(Index::C1.as_bitboard().state | Index::F1.as_bitboard().state);
        pieces[Color::White as usize][Piece::Knight as usize] =
            BitBoard::new(Index::B1.as_bitboard().state | Index::G1.as_bitboard().state);
        pieces[Color::White as usize][Piece::Rook as usize] =
            BitBoard::new(Index::A1.as_bitboard().state | Index::H1.as_bitboard().state);
        pieces[Color::White as usize][Piece::Pawn as usize] = BitBoard::new(
            Index::A2.as_bitboard().state
                | Index::B2.as_bitboard().state
                | Index::C2.as_bitboard().state
                | Index::D2.as_bitboard().state
                | Index::E2.as_bitboard().state
                | Index::F2.as_bitboard().state
                | Index::G2.as_bitboard().state
                | Index::H2.as_bitboard().state,
        );

        pieces[Color::Black as usize][Piece::King as usize] = Index::E8.as_bitboard();
        pieces[Color::Black as usize][Piece::Queen as usize] = Index::D8.as_bitboard();
        pieces[Color::Black as usize][Piece::Bishop as usize] =
            BitBoard::new(Index::C8.as_bitboard().state | Index::F8.as_bitboard().state);
        pieces[Color::Black as usize][Piece::Knight as usize] =
            BitBoard::new(Index::B8.as_bitboard().state | Index::G8.as_bitboard().state);
        pieces[Color::Black as usize][Piece::Rook as usize] =
            BitBoard::new(Index::A8.as_bitboard().state | Index::H8.as_bitboard().state);
        pieces[Color::Black as usize][Piece::Pawn as usize] = BitBoard::new(
            Index::A7.as_bitboard().state
                | Index::B7.as_bitboard().state
                | Index::C7.as_bitboard().state
                | Index::D7.as_bitboard().state
                | Index::E7.as_bitboard().state
                | Index::F7.as_bitboard().state
                | Index::G7.as_bitboard().state
                | Index::H7.as_bitboard().state,
        );

        let mut castling_options = [[false; 2]; Color::VALUES.len()];
        castling_options[Color::White as usize][Piece::Queen as usize] = true;
        castling_options[Color::White as usize][Piece::King as usize] = true;
        castling_options[Color::Black as usize][Piece::Queen as usize] = true;
        castling_options[Color::Black as usize][Piece::King as usize] = true;

        ChessBoard {
            pieces,
            next_move: Color::White,
            castling_options,
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            piece_cache: ChessBoard::new_piece_cache(&pieces),
        }
    }

    /// Returns bitboard of all pieces of a color.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard, Color};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// board.pieces(Color::White),
    /// BitBoard::from_string("
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     x x x x x x x x
    ///     x x x x x x x x
    /// ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn pieces(&self, color: Color) -> BitBoard {
        self.pieces[color]
            .iter()
            .fold(BitBoard::EMPTY, |res, val| res | *val)
    }

    /// Returns bitboard of all pieces.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// board.all_pieces(),
    /// BitBoard::from_string("
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     x x x x x x x x
    ///     x x x x x x x x
    /// ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn all_pieces(&self) -> BitBoard {
        self.pieces(Color::White) | self.pieces(Color::Black)
    }

    /// Returns bitboard of pieces which are to move.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// board.my_pieces(),
    /// BitBoard::from_string("
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     x x x x x x x x
    ///     x x x x x x x x
    /// ").unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn my_pieces(&self) -> BitBoard {
        self.pieces(self.next_move)
    }

    /// Returns bitboard of opponent pieces.
    ///     
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// board.opponent_pieces(),
    /// BitBoard::from_string("
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    /// ").unwrap());
    /// ```   
    #[inline(always)]
    #[must_use]
    pub fn opponent_pieces(&self) -> BitBoard {
        self.pieces(self.next_move.opponent())
    }

    /// Returns bitboard of filed which we may attack.
    /// This means all empty and opponent squares.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// board.board_to_attack(),
    /// BitBoard::from_string("
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     x x x x x x x x
    ///     - - - - - - - -
    ///     - - - - - - - -
    /// ").unwrap());
    /// ```   
    #[inline(always)]
    #[must_use]
    pub fn board_to_attack(&self) -> BitBoard {
        !self.my_pieces()
    }

    /// Returns index of my king - king of color which is at move.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Index};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// Index::E1,
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap().my_king().unwrap());
    ///
    /// assert!(ChessBoard::EMPTY.my_king().is_none());
    /// ```   
    #[inline(always)]
    #[must_use]
    pub fn my_king(&self) -> Option<Index> {
        self.pieces[self.next_move][Piece::King].bitscan()
    }

    /// Returns index of opponent's king - king of color which will be on move next turn.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Index};
    ///
    /// let board = ChessBoard::STANDARD;
    ///
    /// assert_eq!(
    /// Index::E8,
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap().opponent_king().unwrap());
    /// ```   
    #[inline(always)]
    #[must_use]
    pub fn opponent_king(&self) -> Option<Index> {
        self.pieces[self.next_move.opponent()][Piece::King].bitscan()
    }

    /// Returns piece (without color) on a ChessBoard field.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Color, Index, Piece};
    ///
    /// assert_eq!(
    /// (Color::White, Piece::King),
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap().piece_at(Index::E1).unwrap());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn piece_at(&self, i: Index) -> Option<(Color, Piece)> {
        self.piece_cache[i.index]
    }

    /// Construct piece cache which holds information about Piece at ChessBoard Index.
    #[must_use]
    pub(super) const fn new_piece_cache(
        pieces: &[[BitBoard; Piece::VALUES.len()]; Color::VALUES.len()],
    ) -> [Option<(Color, Piece)>; Index::ALL_FIELDS.len()] {
        let mut cache = [None; Index::ALL_FIELDS.len()];

        let mut ic = 0;
        while ic < Color::VALUES.len() {
            let mut ip = 0;
            while ip < Piece::VALUES.len() {
                let mut b = pieces[ic][ip];

                while let (Some(i), next) = b.bitpop() {
                    b = next;
                    cache[i.index] = Some((Color::VALUES[ic], Piece::VALUES[ip]));
                }
                ip += 1;
            }
            ic += 1;
        }

        cache
    }

    /// Generate all legal moves for a board
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Move};
    ///
    /// let mut board = ChessBoard::from_string("
    ///       a b c d e f g h
    ///     8 r n b q k b n r 8
    ///     7 p p p p p p p p 7
    ///     6 - - - - - - - - 6
    ///     5 - - - - - - - - 5
    ///     4 - - - - - - - - 4
    ///     3 - - - - - - - - 3
    ///     2 P P P P P P P P 2
    ///     1 R N B Q K B N R 1
    ///       a b c d e f g h
    /// ").unwrap();
    ///
    /// let moves = board.legal_moves();
    /// assert_eq!(moves.len(), 20);
    /// ```
    pub fn legal_moves(&self) -> Vec<Move> {
        Generator::G.legal_moves(self)
    }

    /// Generates all pseudo-legal moves.
    ///
    /// See: [ChessProgramming Pseudo Legal Move](https://www.chessprogramming.org/Pseudo-Legal_Move)
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard, Color, Index};
    ///
    /// let mut board = ChessBoard::from_string("
    ///      - - - - q - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - B - - -
    ///      - - - Q K - - -
    /// ").unwrap();
    ///
    /// let mut moves = Vec::new();
    ///
    /// board.moves(&mut |m| {
    ///    moves.push(m);
    /// });
    ///
    /// assert_eq!(moves.len(), 24);
    /// assert_eq!(board.legal_moves().len(), 16);
    /// ```
    pub fn moves(&self, f: &mut impl FnMut(Move)) {
        Generator::G.moves(self, f)
    }

    /// Generate attacks (moves and captures) for one side.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard, Color};
    ///
    /// let mut board = ChessBoard::from_string("
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - q k - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - Q K - - -
    /// ").unwrap();
    ///
    /// assert_eq!(
    ///     board.attacks(Color::White),
    ///     BitBoard::from_string("
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - x - - - x
    ///      x - - x - - x -
    ///      - x - x - x - -
    ///      - - x x x x - -
    ///      x x x x x x - -
    ///     ").unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     board.attacks(Color::Black),
    ///     BitBoard::from_string("
    ///      x - - x - - x -
    ///      - x - x - x - -
    ///      - - x x x x - -
    ///      x x x x x x - -
    ///      - - x x x x - -
    ///      - x - x - x - -
    ///      x - - x - - x -
    ///      - - - x - - - x
    ///     ").unwrap()
    /// );
    /// ```
    pub fn attacks(&self, color: Color) -> BitBoard {
        Generator::G.attacks(self, color)
    }

    /// Checks if opponent king is under check.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard, Color, Index};
    ///
    /// let mut board = ChessBoard::from_string("
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - k - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - Q K - - -
    /// ").unwrap();
    ///
    ///
    /// assert!(board.is_opponent_king_under_check());
    /// ```
    pub fn is_opponent_king_under_check(&self) -> bool {
        Generator::G.is_opponent_king_under_check(self)
    }

    /// Checks if BitMask is under attack by a side.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{BitBoard, ChessBoard, Color, Index};
    ///
    /// let mut board = ChessBoard::from_string("
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - q k - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - - - - - -
    ///      - - - Q K - - -
    /// ").unwrap();
    ///
    ///
    /// assert!(!board.is_bitmask_under_attack(Color::White, Index::A8 | Index::B8));
    /// assert!(board.is_bitmask_under_attack(Color::White, Index::A4 | Index::A5));
    /// ```
    pub fn is_bitmask_under_attack(&self, color: Color, b: BitBoard) -> bool {
        Generator::G.is_bitmask_under_attack(self, color, b)
    }

    /// Validate and apply move.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Generator, Move};
    ///
    /// let mut board = ChessBoard::from_string("
    ///       a b c d e f g h
    ///     8 r n b q k b n r 8
    ///     7 p p p p p p p p 7
    ///     6 - - - - - - - - 6
    ///     5 - - - - - - - - 5
    ///     4 - - - - - - - - 4
    ///     3 - - - - - - - - 3
    ///     2 P P P P P P P P 2
    ///     1 R N B Q K B N R 1
    ///       a b c d e f g h
    /// ").unwrap();
    ///
    /// board = board.validate_and_apply_move(&Move::from_string("b1c3").unwrap()).unwrap();
    ///
    /// assert_eq!(
    ///     board.pieces,
    ///     ChessBoard::from_string("
    ///       a b c d e f g h
    ///     8 r n b q k b n r 8
    ///     7 p p p p p p p p 7
    ///     6 - - - - - - - - 6
    ///     5 - - - - - - - - 5
    ///     4 - - - - - - - - 4
    ///     3 - - N - - - - - 3
    ///     2 P P P P P P P P 2
    ///     1 R - B Q K B N R 1
    ///       a b c d e f g h
    ///     ").unwrap().pieces
    /// );
    ///
    /// assert!(board.validate_and_apply_move(&Move::from_string("a1a8").unwrap()).is_err());
    /// ```
    pub fn validate_and_apply_move(&self, m: &Move) -> Result<ChessBoard, IlegalMoveError> {
        match self.legal_moves().iter().find(|mm| *mm == m) {
            Some(_) => Ok(self.apply_move(m)),
            None => Err(IlegalMoveError::IlegalMove(*m)),
        }
    }

    /// Apply move to copy of the ChessBoard and return it.
    /// Move validation is not performed, if the move is not valid,
    /// results of this operation may be unpredictable.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Move};
    ///
    /// let m = Move::from_string("e2e4").unwrap();
    /// let board = ChessBoard::STANDARD.apply_move(&m).pieces;
    ///
    /// assert_eq!(
    /// board,
    /// ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - P - - -
    ///     - - - - - - - -
    ///     P P P P - P P P
    ///     R N B Q K B N R
    /// ").unwrap().pieces);
    /// ```
    #[must_use]
    pub fn apply_move(&self, m: &Move) -> Self {
        let (_, piece) = self.piece_at(m.from).unwrap();
        let color = self.next_move;

        let mut pieces = self.pieces;
        let mut next_move = self.next_move;
        let mut castling_options = self.castling_options;
        let mut en_passant_target = self.en_passant_target;
        let mut full_move_number = self.full_move_number;
        let mut half_move_clock = self.half_move_clock + 1;
        let mut piece_cache = self.piece_cache;

        let is_capture = (self.opponent_pieces() & m.to) != BitBoard::EMPTY;
        let is_enpassant = piece == Piece::Pawn
            && match en_passant_target {
                Some(i) => m.to == i,
                None => false,
            };

        // reset enPassant
        en_passant_target = None;

        // make the move
        pieces[next_move][piece] ^= m.from | m.to;
        piece_cache[m.from] = None;
        piece_cache[m.to] = Some((color, piece));

        match piece {
            Piece::Rook => match next_move {
                Color::White => match m.from {
                    Index::A1 => castling_options[next_move][Piece::Queen] = false,
                    Index::H1 => castling_options[next_move][Piece::King] = false,
                    _ => {}
                },
                Color::Black => match m.from {
                    Index::A8 => castling_options[next_move][Piece::Queen] = false,
                    Index::H8 => castling_options[next_move][Piece::King] = false,
                    _ => {}
                },
            },
            Piece::King => {
                castling_options[next_move][Piece::Queen] = false;
                castling_options[next_move][Piece::King] = false;
                match next_move {
                    Color::White => {
                        if m.from == Index::E1 {
                            match m.to {
                                Index::C1 => {
                                    pieces[next_move][Piece::Rook] ^= Index::A1 | Index::D1;
                                    piece_cache[Index::A1] = None;
                                    piece_cache[Index::D1] = Some((color, Piece::Rook));
                                }
                                Index::G1 => {
                                    pieces[next_move][Piece::Rook] ^= Index::H1 | Index::F1;
                                    piece_cache[Index::H1] = None;
                                    piece_cache[Index::F1] = Some((color, Piece::Rook));
                                }
                                _ => {}
                            }
                        }
                    }
                    Color::Black => {
                        if m.from == Index::E8 {
                            match m.to {
                                Index::C8 => {
                                    pieces[next_move][Piece::Rook] ^= Index::A8 | Index::D8;
                                    piece_cache[Index::A8] = None;
                                    piece_cache[Index::D8] = Some((color, Piece::Rook));
                                }
                                Index::G8 => {
                                    pieces[next_move][Piece::Rook] ^= Index::H8 | Index::F8;
                                    piece_cache[Index::H8] = None;
                                    piece_cache[Index::F8] = Some((color, Piece::Rook));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Piece::Pawn => {
                half_move_clock = 0;
                if m.to.distance_to(m.from) > 10 {
                    let i = if next_move == Color::White {
                        m.from.shifted_north()
                    } else {
                        m.from.shifted_south()
                    };
                    en_passant_target = Some(i.unwrap());
                } else if let Some(promotion) = m.promotion {
                    pieces[next_move][Piece::Pawn] ^= m.to;

                    pieces[next_move][promotion] ^= m.to;
                    piece_cache[m.to] = Some((color, promotion));
                }
            }
            _ => {}
        }

        if is_enpassant {
            half_move_clock = 0;

            match next_move {
                Color::White => {
                    let i = m.to.shifted_south().unwrap();
                    pieces[Color::Black][Piece::Pawn] ^= i;
                    piece_cache[i] = None;
                }
                Color::Black => {
                    let i = m.to.shifted_north().unwrap();
                    pieces[Color::White][Piece::Pawn] ^= i;
                    piece_cache[i] = None;
                }
            }
        }

        if is_capture {
            half_move_clock = 0;
            let opponent_color = self.next_move.opponent();

            for p in Piece::VALUES {
                if pieces[opponent_color][p].has_bit(m.to) {
                    pieces[opponent_color][p] ^= m.to;
                    break;
                }
            }

            match next_move {
                Color::White => match m.to {
                    Index::A8 => castling_options[Color::Black][Piece::Queen] = false,
                    Index::H8 => castling_options[Color::Black][Piece::King] = false,
                    _ => {}
                },
                Color::Black => match m.to {
                    Index::A1 => castling_options[Color::White][Piece::Queen] = false,
                    Index::H1 => castling_options[Color::White][Piece::King] = false,
                    _ => {}
                },
            }
        }

        next_move = self.next_move.opponent();

        if next_move == Color::Black {
            full_move_number += 1;
        }

        ChessBoard {
            pieces,
            next_move,
            castling_options,
            en_passant_target,
            half_move_clock,
            full_move_number,
            piece_cache,
        }
    }

    /// Creates new board form a string.
    /// String may or may not be decorated with coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// assert_eq!(
    ///  ChessBoard::from_string("
    ///     r n b q k b n r
    ///     p p p p p p p p
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     - - - - - - - -
    ///     P P P P P P P P
    ///     R N B Q K B N R
    /// ").unwrap(),
    /// ChessBoard::from_string("
    ///       a b c d e f g h
    ///     8 r n b q k b n r 8
    ///     7 p p p p p p p p 7
    ///     6 - - - - - - - - 6
    ///     5 - - - - - - - - 5
    ///     4 - - - - - - - - 4
    ///     3 - - - - - - - - 3
    ///     2 P P P P P P P P 2
    ///     1 R N B Q K B N R 1
    ///       a b c d e f g h
    /// ").unwrap());
    /// ```
    pub fn from_string(str: &str) -> Result<ChessBoard, InvalidChessBoardStringError> {
        let mut pieces = str.replace("a b c d e f g h", "");
        pieces.retain(|c| !"0123456789 \n".contains(c));
        pieces = pieces.replace('-', "1");

        if pieces.len() != Index::ALL_FIELDS.len() {
            return Err(InvalidChessBoardStringError::InvalidString(str.to_string()));
        }

        pieces += " w KQkq - 0 1";

        match ChessBoard::from_fen(&pieces) {
            Ok(board) => Ok(board),
            Err(_) => Err(InvalidChessBoardStringError::InvalidString(str.to_string())),
        }
    }

    /// Returns FEN representation of this board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// assert_eq!(
    ///  ChessBoard::STANDARD.to_fen(),
    ///  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// );
    /// ```
    #[must_use]
    pub fn to_fen(&self) -> String {
        let mut pieces = Vec::new();

        for c in Color::VALUES {
            for p in Piece::VALUES {
                pieces.push((self.pieces[c][p].mirrored_vertically(), p.to_char(c)))
            }
        }

        let mut fen = String::new();
        let mut spaces: usize = 0;

        let output_spaces = |fen: &mut String, spaces: &mut usize| {
            if *spaces > 0 {
                write!(fen, "{}", spaces).unwrap();
            }
            *spaces = 0;
        };

        for i in 0..Index::ALL_FIELDS.len() {
            if i > 0 && (i % 8) == 0 {
                output_spaces(&mut fen, &mut spaces);
                write!(fen, "/").unwrap();
            }

            if let Some((_, c)) = pieces.iter().find(|p| p.0.has_bit(Index::new(i))) {
                output_spaces(&mut fen, &mut spaces);
                write!(fen, "{}", c).unwrap();
            } else {
                spaces += 1;
            }
        }
        output_spaces(&mut fen, &mut spaces);

        // next move color
        write!(fen, " {} ", self.next_move).unwrap();

        // Castling
        let mut some_castling = false;
        for c in Color::VALUES {
            if self.castling_options[c][Piece::King] {
                write!(fen, "{}", Piece::King.to_char(c)).unwrap();
                some_castling = true;
            }
            if self.castling_options[c][Piece::Queen] {
                write!(fen, "{}", Piece::Queen.to_char(c)).unwrap();
                some_castling = true;
            }
        }
        if !some_castling {
            write!(fen, "-").unwrap();
        }
        write!(fen, " ").unwrap();

        // enPassant
        if let Some(target) = self.en_passant_target {
            write!(fen, "{} ", target).unwrap();
        } else {
            write!(fen, "- ").unwrap();
        }

        // clock + move number
        write!(fen, "{} {}", self.half_move_clock, self.full_move_number).unwrap();

        fen
    }

    /// Returns ChessBoard from FEN definition.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::ChessBoard;
    ///
    /// assert_eq!(
    ///  ChessBoard::STANDARD,
    ///  ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    /// );
    ///
    /// assert_eq!(
    ///  ChessBoard::STANDARD,
    ///  ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap()
    /// );
    /// ```
    pub fn from_fen(fen: &str) -> Result<Self, InvalidFENStringError> {
        //"8/8/8/8/8/8/8/8"
        if fen.len() < 15 {
            return Err(InvalidFENStringError::InvalidString(fen.to_string()));
        }

        let mut pieces = [[BitBoard::EMPTY; Piece::VALUES.len()]; Color::VALUES.len()];
        let mut next_move = Color::White;
        let mut castling_options = [[true; 2]; Color::VALUES.len()];
        let mut en_passant_target = None;
        let mut full_move_number = 1;
        let mut half_move_clock = 0;

        let mut i: usize = 0;

        let chars = fen.as_bytes();

        fn shift_pieces(
            pieces: [[BitBoard; Piece::VALUES.len()]; Color::VALUES.len()],
            size: usize,
        ) -> [[BitBoard; Piece::VALUES.len()]; Color::VALUES.len()] {
            let mut shifted = pieces;

            for c in Color::VALUES {
                for p in Piece::VALUES {
                    shifted[c][p] = BitBoard::new(shifted[c][p].state << size);
                }
            }

            shifted
        }

        // pieces
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            if c == b' ' {
                break;
            }

            if c == b'/' {
                // nothing
                continue;
            }

            if c.is_ascii_digit() {
                // shift by number of empty fields
                pieces = shift_pieces(pieces, (c - b'0') as usize);
            } else {
                // shift all pieces by 1
                pieces = shift_pieces(pieces, 1);

                match Piece::from_char(c as char) {
                    Ok((color, piece)) => pieces[color][piece] |= Index::A1,
                    Err(_) => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
                }
            }
        }

        // need to mirror the boards
        for c in Color::VALUES {
            for p in Piece::VALUES {
                pieces[c][p] = pieces[c][p].mirrored_horizontally();
            }
        }

        // next move
        if i < chars.len() {
            match Color::from_char(chars[i] as char) {
                Ok(color) => next_move = color,
                Err(_) => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
            }
            i += 1;
        }

        // castling
        i += 1;
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            match c {
                b' ' => break,
                b'-' => {}
                _ => match Piece::from_char(c as char) {
                    Ok((color, piece)) => match piece {
                        Piece::King => castling_options[color][Piece::King] = true,
                        Piece::Queen => castling_options[color][Piece::Queen] = true,
                        _ => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
                    },
                    Err(_) => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
                },
            }
        }

        // enPassant
        let mut notation = String::new();
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            match c {
                b' ' => break,
                b'-' => {}
                _ => notation.push(c as char),
            }
        }
        if !notation.is_empty() {
            match Index::from_string(&notation) {
                Ok(i) => en_passant_target = Some(i),
                Err(_) => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
            }
        }

        // half move clock
        let mut n: usize = 0;
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            match c {
                b' ' => break,
                b'-' => {}
                _ if c.is_ascii_digit() => n = n * 10 + (c - b'0') as usize,
                _ => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
            }
        }
        if n > 0 {
            half_move_clock = n;
        }

        // full move number
        n = 0;
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            match c {
                b' ' => break,
                b'-' => {}
                _ if c.is_ascii_digit() => n = n * 10 + (c - b'0') as usize,
                _ => return Err(InvalidFENStringError::InvalidString(fen.to_string())),
            }
        }
        // full move nubmer starts at 1
        if n > 0 {
            full_move_number = n;
        }

        // fix castling - rooks
        if !pieces[Color::White][Piece::Rook].has_bit(Index::A1) {
            castling_options[Color::White][Piece::Queen] = false;
        }
        if !pieces[Color::White][Piece::Rook].has_bit(Index::H1) {
            castling_options[Color::White][Piece::King] = false;
        }
        if !pieces[Color::Black][Piece::Rook].has_bit(Index::A8) {
            castling_options[Color::Black][Piece::Queen] = false;
        }
        if !pieces[Color::Black][Piece::Rook].has_bit(Index::H8) {
            castling_options[Color::Black][Piece::King] = false;
        }

        // fix castling - kings
        if !pieces[Color::White][Piece::King].has_bit(Index::E1) {
            castling_options[Color::White][Piece::King] = false;
            castling_options[Color::White][Piece::Queen] = false;
        }
        if !pieces[Color::Black][Piece::King].has_bit(Index::E8) {
            castling_options[Color::Black][Piece::King] = false;
            castling_options[Color::Black][Piece::Queen] = false;
        }

        Ok(ChessBoard {
            pieces,
            next_move,
            castling_options,
            en_passant_target,
            half_move_clock,
            full_move_number,
            piece_cache: ChessBoard::new_piece_cache(&pieces),
        })
    }
}

/// Display and to_string() for a ChessBoard.
///
/// # Examples
///
/// ```
/// use chessgen::ChessBoard;
///
/// assert_eq!(
///     ChessBoard::STANDARD,
///     ChessBoard::from_string(ChessBoard::STANDARD.to_string().as_str()).unwrap()
/// );
/// ```
impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER: &str = "  a b c d e f g h\n";
        write!(f, "{}", HEADER)?;

        let mut pieces = Vec::new();

        for c in Color::VALUES {
            for p in Piece::VALUES {
                pieces.push((self.pieces[c][p].mirrored_vertically(), p.to_char(c)))
            }
        }

        for i in 0..Index::ALL_FIELDS.len() {
            if (i % 8) == 0 {
                if i > 0 {
                    writeln!(f, "{}", 9 - (i / 8))?;
                }

                write!(f, "{} ", 8 - (i / 8))?;
            }

            if let Some((_, c)) = pieces.iter().find(|p| p.0.has_bit(Index::new(i))) {
                write!(f, "{} ", c)?;
            } else {
                write!(f, "- ")?;
            }
        }

        write!(f, "1\n{}", HEADER)?;

        Ok(())
    }
}
