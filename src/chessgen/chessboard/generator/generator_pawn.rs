use crate::{BitBoard, ChessBoard, Color, Index, Move, Piece};

/// Moves generator for a Pawn.
pub(super) struct GeneratorPawn {
    pub attacks_cache: [[BitBoard; Index::ALL_FIELDS.len()]; Color::VALUES.len()],
}

impl GeneratorPawn {
    /// Construct a new generator.
    pub(super) const fn new() -> Self {
        let mut cache = [[BitBoard::EMPTY; Index::ALL_FIELDS.len()]; Color::VALUES.len()];

        let mut i = 0;
        while i < Index::ALL_FIELDS.len() {
            let b = Index::ALL_FIELDS[i].as_bitboard();

            cache[Color::White as usize][i] =
                BitBoard::new(b.shifted_northeast().state | b.shifted_northwest().state);
            cache[Color::Black as usize][i] =
                BitBoard::new(b.shifted_southeast().state | b.shifted_southwest().state);

            i += 1;
        }

        GeneratorPawn {
            attacks_cache: cache,
        }
    }

    /// Generate attacks.
    pub(super) fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let b = board.pieces[color][Piece::Pawn];

        match color {
            Color::White => b.shifted_northeast() | b.shifted_northwest(),
            Color::Black => b.shifted_southeast() | b.shifted_southwest(),
        }
    }

    /// Generate moves.
    pub(super) fn generate_moves(&self, board: &ChessBoard, f: &mut impl FnMut(Move)) {
        let empty_board = !board.all_pieces();
        let mut pieces = board.pieces[board.next_move][Piece::Pawn];

        while let (Some(from), tmp) = pieces.bitpop() {
            pieces = tmp;
            let b = from.as_bitboard();
            let attacks: BitBoard;
            let mut moves: BitBoard;

            match board.next_move {
                Color::White => {
                    moves = b.shifted_north() & empty_board;
                    // double move
                    if from < Index::A3 {
                        moves |= moves.shifted_north() & empty_board;
                    }
                    // attacks
                    attacks = b.shifted_northeast() | b.shifted_northwest();
                    moves |= attacks & board.opponent_pieces();
                }
                Color::Black => {
                    moves = b.shifted_south() & empty_board;
                    // double move
                    if from > Index::H6 {
                        moves |= moves.shifted_south() & empty_board;
                    }
                    // attacks
                    attacks = b.shifted_southeast() | b.shifted_southwest();
                    moves |= attacks & board.opponent_pieces();
                }
            };

            while let (Some(to), tmp) = moves.bitpop() {
                moves = tmp;
                if to > Index::H7 || to < Index::A2 {
                    // promotion
                    f(Move {
                        from,
                        to,
                        promotion: Some(Piece::Bishop),
                    });
                    f(Move {
                        from,
                        to,
                        promotion: Some(Piece::Knight),
                    });
                    f(Move {
                        from,
                        to,
                        promotion: Some(Piece::Queen),
                    });
                    f(Move {
                        from,
                        to,
                        promotion: Some(Piece::Rook),
                    });
                } else {
                    f(Move {
                        from,
                        to,
                        promotion: None,
                    });
                }
            }

            if let Some(en_passant_target) = board.en_passant_target {
                moves = attacks & en_passant_target;
                if let Some(to) = moves.bitscan() {
                    f(Move {
                        from,
                        to,
                        promotion: None,
                    });
                }
            }
        }
    }
}
