use crate::{BitBoard, ChessBoard, Color, Index, Move, Piece};

/// magic constants
/// See: [ChessProgramming Magic Bitboards](https://www.chessprogramming.org/Magic_Bitboards)
const MAGIC_FILE: [BitBoard; 8] = [
    BitBoard::new(0x8040201008040200),
    BitBoard::new(0x4020100804020100),
    BitBoard::new(0x2010080402010080),
    BitBoard::new(0x1008040201008040),
    BitBoard::new(0x0804020100804020),
    BitBoard::new(0x0402010080402010),
    BitBoard::new(0x0201008040201008),
    BitBoard::new(0x0100804020100804),
];

/// Moves generator for a Rook (+Queen).
#[derive(Debug)]
pub(super) struct GeneratorRook {
    pub rank_shift: [usize; Index::ALL_FIELDS.len()],
    pub rank_mask: [BitBoard; Index::ALL_FIELDS.len()],
    pub rank_attacks: [[BitBoard; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()],

    pub file_magic: [BitBoard; Index::ALL_FIELDS.len()],
    pub file_mask: [BitBoard; Index::ALL_FIELDS.len()],
    pub file_attacks: [[BitBoard; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()],
}

impl GeneratorRook {
    /// Construct a new generator.
    pub(super) const fn new() -> Self {
        let file_a_mask = BitBoard::new(
            Index::A2.as_bitboard().state
                | Index::A3.as_bitboard().state
                | Index::A4.as_bitboard().state
                | Index::A5.as_bitboard().state
                | Index::A6.as_bitboard().state
                | Index::A7.as_bitboard().state,
        );

        let mut rank_shift = [0usize; Index::ALL_FIELDS.len()];
        let mut rank_mask = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut rank_attacks =
            [[BitBoard::EMPTY; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()];

        let mut file_magic = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut file_mask = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut file_attacks =
            [[BitBoard::EMPTY; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Index::ALL_FIELDS.len() {
            let rank = Index::new(i).rank();
            let file = Index::new(i).file();

            // get 6-bit mask for a rank
            rank_mask[i] = BitBoard::new(126 << (rank << 3));

            // compute needed rank shift
            rank_shift[i] = (rank << 3) + 1;

            // get 6-bit mask for a file
            file_mask[i] = BitBoard::new(file_a_mask.state << file);

            // index magic number directly fo field
            file_magic[i] = MAGIC_FILE[file];

            i += 1;
        }

        // precompute rank moves
        // for all pieces
        i = 0;
        while i < Index::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Index::ALL_FIELDS.len() {
                // reconstruct occupancy state
                let board = BitBoard::new(n as u64).shifted(1, Index::new(i).rank() as isize);

                // generate available moves
                let mut moves = BitBoard::EMPTY;

                // set piece in Ith position
                let mut piece = Index::new(i).as_bitboard();

                // move in one direction
                while piece.state != 0 {
                    piece = piece.shifted_west();
                    moves = BitBoard::new(moves.state | piece.state);

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.state & board.state) != 0 {
                        break;
                    }
                }

                // set piece back in Ith position
                piece = Index::new(i).as_bitboard();

                // move in other direction
                while piece.state != 0 {
                    piece = piece.shifted_east();
                    moves = BitBoard::new(moves.state | piece.state);

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.state & board.state) != 0 {
                        break;
                    }
                }

                // remember the moves
                rank_attacks[i][n] = moves;

                n += 1;
            }
            i += 1;
        }

        // precompute file moves
        // for all pieces
        i = 0;
        while i < Index::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Index::ALL_FIELDS.len() {
                // reconstruct occupancy state
                let board = BitBoard::new(n as u64)
                    .shifted(1, 0)
                    .mirrored_horizontally()
                    .mirrored_a1h8()
                    .shifted(Index::new(i).file() as isize, 0);

                // generate available moves
                let mut moves = BitBoard::EMPTY;

                // set piece in Ith position
                let mut piece = Index::new(i).as_bitboard();

                // move in one direction
                while piece.state != 0 {
                    piece = piece.shifted_north();
                    moves = BitBoard::new(moves.state | piece.state);

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.state & board.state) != 0 {
                        break;
                    }
                }

                // set piece back in Ith position
                piece = Index::new(i).as_bitboard();

                // move in other direction
                while piece.state != 0 {
                    piece = piece.shifted_south();
                    moves = BitBoard::new(moves.state | piece.state);

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.state & board.state) != 0 {
                        break;
                    }
                }

                // remember the moves
                file_attacks[i][n] = moves;
                n += 1;
            }
            i += 1;
        }

        GeneratorRook {
            rank_shift,
            rank_mask,
            rank_attacks,
            file_magic,
            file_mask,
            file_attacks,
        }
    }

    /// Generate attacks for one piece.
    fn attacks(&self, i: Index, all_pieces: BitBoard) -> BitBoard {
        // use magic multipliers to get occupancy state index
        let state_rank = (all_pieces & self.rank_mask[i]).state >> self.rank_shift[i.index];
        let state_file = ((all_pieces & self.file_mask[i]).state * self.file_magic[i].state) >> 57;

        // get possible attacks for field / occupancy state index
        self.rank_attacks[i.index][state_rank as usize]
            | self.file_attacks[i.index][state_file as usize]
    }

    /// Generate attacks.
    pub(super) fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let mut b = board.pieces[color][Piece::Rook] | board.pieces[color][Piece::Queen];
        let mut attacks = BitBoard::EMPTY;

        let all_pieces = board.all_pieces();

        // for all rooks
        while let (Some(i), tmp) = b.bitpop() {
            b = tmp;
            attacks |= self.attacks(i, all_pieces);
        }

        attacks
    }

    /// Generate moves.
    pub(super) fn generate_moves(&self, board: &ChessBoard, f: &mut impl FnMut(Move)) {
        let all_pieces = board.all_pieces();
        let board_available = board.board_to_attack();

        for p in [Piece::Rook, Piece::Queen] {
            let mut pieces = board.pieces[board.next_move][p];

            while let (Some(i), tmp) = pieces.bitpop() {
                pieces = tmp;
                let mut moves = self.attacks(i, all_pieces) & board_available;

                while let (Some(t), tmp) = moves.bitpop() {
                    moves = tmp;
                    f(Move {
                        from: i,
                        to: t,
                        promotion: None,
                    });
                }
            }
        }
    }
}
