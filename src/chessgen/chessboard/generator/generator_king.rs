use crate::{BitBoard, ChessBoard, Color, Index, Move, Piece};

use super::generator::Generator;

const WHITE_CASTLING_OO_EMPTY: BitBoard = BitBoard::from_index_array(&[Index::F1, Index::G1]);
const WHITE_CASTLING_OO_ATTACKS: BitBoard =
    BitBoard::from_index_array(&[Index::E1, Index::F1, Index::G1]);
const WHITE_CASTLING_OOO_EMPTY: BitBoard =
    BitBoard::from_index_array(&[Index::B1, Index::C1, Index::D1]);
const WHITE_CASTLING_OOO_ATTACKS: BitBoard =
    BitBoard::from_index_array(&[Index::C1, Index::D1, Index::E1]);
const BLACK_CASTLING_OO_EMPTY: BitBoard = BitBoard::from_index_array(&[Index::F8, Index::G8]);
const BLACK_CASTLING_OO_ATTACKS: BitBoard =
    BitBoard::from_index_array(&[Index::E8, Index::F8, Index::G8]);
const BLACK_CASTLING_OOO_EMPTY: BitBoard =
    BitBoard::from_index_array(&[Index::B8, Index::C8, Index::D8]);
const BLACK_CASTLING_OOO_ATTACKS: BitBoard =
    BitBoard::from_index_array(&[Index::C8, Index::D8, Index::E8]);

/// Moves generator for a King.
pub(super) struct GeneratorKing {
    pub attacks_cache: [BitBoard; Index::ALL_FIELDS.len()],
}

impl GeneratorKing {
    /// Construct a new generator.
    pub(super) const fn new() -> Self {
        let mut cache = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Index::ALL_FIELDS.len() {
            let b = Index::ALL_FIELDS[i].as_bitboard();

            cache[i] = BitBoard::new(
                b.shifted(1, -1).state
                    | b.shifted(1, 0).state
                    | b.shifted(1, 1).state
                    | b.shifted(0, -1).state
                    | b.shifted(0, 1).state
                    | b.shifted(-1, -1).state
                    | b.shifted(-1, 0).state
                    | b.shifted(-1, 1).state,
            );

            i += 1;
        }

        GeneratorKing {
            attacks_cache: cache,
        }
    }

    /// Generate attacks.
    pub(super) fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let b = board.pieces[color][Piece::King];

        if let Some(i) = b.bitscan() {
            self.attacks_cache[i]
        } else {
            BitBoard::EMPTY
        }
    }

    /// Generate moves.
    pub(super) fn generate_moves(&self, board: &ChessBoard, f: &mut impl FnMut(Move)) {
        let Some(from) = board.pieces[board.next_move][Piece::King].bitscan() else {
            return;
        };

        let mut moves = self.attacks_cache[from] & board.board_to_attack();

        while let (Some(to), tmp) = moves.bitpop() {
            moves = tmp;
            f(Move {
                from,
                to,
                promotion: None,
            })
        }

        let all_pieces = board.all_pieces();

        match board.next_move {
            Color::White => {
                if board.castling_options[board.next_move][Piece::King]
                    && (all_pieces & WHITE_CASTLING_OO_EMPTY) == BitBoard::EMPTY
                    && !Generator::G.is_bitmask_under_attack(
                        board,
                        Color::Black,
                        WHITE_CASTLING_OO_ATTACKS,
                    )
                {
                    f(Move {
                        from,
                        to: Index::G1,
                        promotion: None,
                    })
                }
                if board.castling_options[board.next_move][Piece::Queen]
                    && (all_pieces & WHITE_CASTLING_OOO_EMPTY) == BitBoard::EMPTY
                    && !Generator::G.is_bitmask_under_attack(
                        board,
                        Color::Black,
                        WHITE_CASTLING_OOO_ATTACKS,
                    )
                {
                    f(Move {
                        from,
                        to: Index::C1,
                        promotion: None,
                    })
                }
            }
            Color::Black => {
                if board.castling_options[board.next_move][Piece::King]
                    && (all_pieces & BLACK_CASTLING_OO_EMPTY) == BitBoard::EMPTY
                    && !Generator::G.is_bitmask_under_attack(
                        board,
                        Color::White,
                        BLACK_CASTLING_OO_ATTACKS,
                    )
                {
                    f(Move {
                        from,
                        to: Index::G8,
                        promotion: None,
                    })
                }
                if board.castling_options[board.next_move][Piece::Queen]
                    && (all_pieces & BLACK_CASTLING_OOO_EMPTY) == BitBoard::EMPTY
                    && !Generator::G.is_bitmask_under_attack(
                        board,
                        Color::White,
                        BLACK_CASTLING_OOO_ATTACKS,
                    )
                {
                    f(Move {
                        from,
                        to: Index::C8,
                        promotion: None,
                    })
                }
            }
        }
    }
}
