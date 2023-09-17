use crate::{BitBoard, ChessBoard, Color, Index, Move, Piece};

/// Moves generator for a Knight.
#[derive(Debug)]
pub(super) struct GeneratorKnight {
    pub attacks_cache: [BitBoard; Index::ALL_FIELDS.len()],
}

impl GeneratorKnight {
    /// Construct a new generator.
    pub(super) const fn new() -> Self {
        let mut cache = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Index::ALL_FIELDS.len() {
            let b = Index::ALL_FIELDS[i].as_bitboard();

            cache[i] = BitBoard::new(
                b.shifted(2, 1).state
                    | b.shifted(2, -1).state
                    | b.shifted(1, 2).state
                    | b.shifted(-1, 2).state
                    | b.shifted(-2, 1).state
                    | b.shifted(-2, -1).state
                    | b.shifted(-1, -2).state
                    | b.shifted(1, -2).state,
            );

            i += 1;
        }

        GeneratorKnight {
            attacks_cache: cache,
        }
    }

    /// Generate attacks.
    pub(super) fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let mut b = board.pieces[color][Piece::Knight];

        let mut attacks = BitBoard::EMPTY;
        while let (Some(from), tmp) = b.bitpop() {
            b = tmp;
            attacks |= self.attacks_cache[from];
        }

        attacks
    }

    /// Generate moves.
    pub(super) fn generate_moves(&self, board: &ChessBoard, f: &mut impl FnMut(Move)) {
        let mut pieces = board.pieces[board.next_move][Piece::Knight];

        while let (Some(from), tmp) = pieces.bitpop() {
            pieces = tmp;
            let mut moves = self.attacks_cache[from] & board.board_to_attack();

            while let (Some(to), tmp) = moves.bitpop() {
                moves = tmp;
                f(Move {
                    from,
                    to,
                    promotion: None,
                })
            }
        }
    }
}
