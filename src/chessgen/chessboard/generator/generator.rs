use crate::{BitBoard, ChessBoard, Color, Move, Piece};

use super::generator_bishop::GeneratorBishop;
use super::generator_king::GeneratorKing;
use super::generator_knight::GeneratorKnight;
use super::generator_pawn::GeneratorPawn;
use super::generator_rook::GeneratorRook;

/// Chess moves generator.
/// Can generate attacks (BitBoard) and Moves for all Chess pieces.
/// Queen does not have it's own generator, rather is considered both Rook and Bishop.
///
/// # Examples
///
/// ## Generate all legal moves for a board
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
/// let moves = board.legal_moves();
/// assert_eq!(moves.len(), 20);
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
#[derive(Debug)]
pub struct Generator {
    /// King generator.
    generator_king: GeneratorKing,
    /// Pawn generator.
    generator_pawn: GeneratorPawn,
    /// Knight generator.
    generator_knight: GeneratorKnight,
    /// Rook (+Queen) generator.
    generator_rook: GeneratorRook,
    /// Bishop (+Queen) generator.
    generator_bishop: GeneratorBishop,
}

impl Generator {
    pub const G: Generator = Generator::new();

    /// Constructs a new default generator instance.
    const fn new() -> Self {
        Generator {
            generator_king: GeneratorKing::new(),
            generator_pawn: GeneratorPawn::new(),
            generator_knight: GeneratorKnight::new(),
            generator_rook: GeneratorRook::new(),
            generator_bishop: GeneratorBishop::new(),
        }
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
    pub(crate) fn attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        self.generator_king.generate_attacks(board, color)
            | self.generator_knight.generate_attacks(board, color)
            | self.generator_pawn.generate_attacks(board, color)
            | self.generator_rook.generate_attacks(board, color)
            | self.generator_bishop.generate_attacks(board, color)
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
    pub(crate) fn legal_moves(&self, board: &ChessBoard) -> Vec<Move> {
        let mut v = Vec::new();

        self.moves(board, &mut |m| {
            let new_board = board.apply_move(&m);
            if !self.is_opponent_king_under_check(&new_board) {
                v.push(m);
            }
        });

        v
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
    pub(crate) fn moves(&self, board: &ChessBoard, f: &mut impl FnMut(Move)) {
        self.generator_rook.generate_moves(board, f);
        self.generator_bishop.generate_moves(board, f);
        self.generator_pawn.generate_moves(board, f);
        self.generator_knight.generate_moves(board, f);
        self.generator_king.generate_moves(board, f);
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
    pub(crate) fn is_opponent_king_under_check(&self, board: &ChessBoard) -> bool {
        // check if opponent king is not under check by my pieces
        let Some(king) = board.opponent_king() else {
            return false;
        };

        let pieces = &board.pieces[board.next_move];
        let all_pieces = board.all_pieces();

        if pieces[Piece::Pawn] & self.generator_pawn.attacks_cache[board.next_move.opponent()][king]
            != BitBoard::EMPTY
        {
            return true;
        }

        if pieces[Piece::Knight] & self.generator_knight.attacks_cache[king] != BitBoard::EMPTY {
            return true;
        }

        if pieces[Piece::King] & self.generator_king.attacks_cache[king] != BitBoard::EMPTY {
            return true;
        }

        let rooks = pieces[Piece::Queen] | pieces[Piece::Rook];

        if self.generator_rook.rank_attacks[king.index][(all_pieces
            & self.generator_rook.rank_mask[king.index])
            .state as usize
            >> self.generator_rook.rank_shift[king.index]]
            & rooks
            != BitBoard::EMPTY
        {
            return true;
        }

        if self.generator_rook.file_attacks[king.index][((all_pieces
            & self.generator_rook.file_mask[king.index])
            .state
            * self.generator_rook.file_magic[king.index].state)
            as usize
            >> 57]
            & rooks
            != BitBoard::EMPTY
        {
            return true;
        }

        let bishops = pieces[Piece::Queen] | pieces[Piece::Bishop];
        if self.generator_bishop.a8h1_attacks[king.index][((all_pieces
            & self.generator_bishop.a8h1_mask[king.index])
            .state
            * self.generator_bishop.a8h1_magic[king.index].state)
            as usize
            >> 57]
            & bishops
            != BitBoard::EMPTY
        {
            return true;
        }

        if self.generator_bishop.a1h8_attacks[king.index][((all_pieces
            & self.generator_bishop.a1h8_mask[king.index])
            .state
            * self.generator_bishop.a1h8_magic[king.index].state)
            as usize
            >> 57]
            & bishops
            != BitBoard::EMPTY
        {
            return true;
        }

        false
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
    pub(crate) fn is_bitmask_under_attack(
        &self,
        board: &ChessBoard,
        color: Color,
        b: BitBoard,
    ) -> bool {
        self.generator_rook.generate_attacks(board, color) & b != BitBoard::EMPTY
            || self.generator_bishop.generate_attacks(board, color) & b != BitBoard::EMPTY
            || self.generator_knight.generate_attacks(board, color) & b != BitBoard::EMPTY
            || self.generator_pawn.generate_attacks(board, color) & b != BitBoard::EMPTY
            || self.generator_king.generate_attacks(board, color) & b != BitBoard::EMPTY
    }
}
