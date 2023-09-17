use crate::{ChessBoard, Color, Index, Piece};

/// Zobrist hash generator for ChessBoard.
/// This implementation uses deterministic random generator with a fixed seed,
/// so all instances of Zobrist should be initialized with same unique numbers.
///
/// See: [ChessProgramming Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)
pub struct Zobrist {
    /// Unique 64bit IDs for pieces.
    pieces: Vec<Vec<Vec<u64>>>,
    /// Unique 64bit IDs for castling.
    castling: Vec<Vec<u64>>,
    /// Unique 64bit IDs for en_passant Index.
    en_passant: Vec<u64>,
    /// Unique 64bit IDs for side on move.
    side: u64,
}

/// Construct a new instance of Zobrist hasher.
impl Default for Zobrist {
    fn default() -> Self {
        Self::new()
    }
}

impl Zobrist {
    /// Construct a new instance of Zobrist hasher.
    pub fn new() -> Zobrist {
        fastrand::seed(13);

        let mut pieces = vec![
            vec![vec![0u64; Index::ALL_FIELDS.len()]; Piece::VALUES.len()];
            Color::VALUES.len()
        ];
        let mut en_passant = vec![0u64; Index::ALL_FIELDS.len()];
        let mut castling = vec![vec![0u64; Piece::VALUES.len()]; Color::VALUES.len()];
        let side = fastrand::u64(..);

        (0..pieces.len()).for_each(|i1| {
            (0..pieces[i1].len()).for_each(|i2| {
                (0..pieces[i1][i2].len()).for_each(|i3| {
                    pieces[i1][i2][i3] = fastrand::u64(..);
                });
            });
        });

        (0..en_passant.len()).for_each(|i| {
            en_passant[i] = fastrand::u64(..);
        });

        (0..castling.len()).for_each(|i1| {
            (0..castling[i1].len()).for_each(|i2| {
                castling[i1][i2] = fastrand::u64(..);
            });
        });

        Zobrist {
            pieces,
            castling,
            en_passant,
            side,
        }
    }

    /// Creates Zobrist hash of a chess board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, Zobrist};
    ///
    /// let board = ChessBoard::from_string("
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
    ///  let hash = Zobrist::new().hash(&board);
    /// ```
    pub fn hash(&self, board: &ChessBoard) -> u64 {
        let mut hash = 0u64;

        if board.next_move != Color::White {
            hash ^= self.side;
        }

        for c in Color::VALUES {
            for p in [Piece::King, Piece::Queen] {
                if board.castling_options[c][p] {
                    hash ^= self.castling[c][p];
                }
            }
        }

        if let Some(en_passant_target) = board.en_passant_target {
            hash ^= self.en_passant[en_passant_target];
        }

        (0..Color::VALUES.len()).for_each(|c| {
            (0..Piece::VALUES.len()).for_each(|p| {
                let mut pieces = board.pieces[c][p];
                while let (Some(i), tmp) = pieces.bitpop() {
                    pieces = tmp;
                    hash ^= self.pieces[c][p][i];
                }
            });
        });

        hash
    }
}
