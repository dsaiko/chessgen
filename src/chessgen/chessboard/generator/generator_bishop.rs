use crate::{BitBoard, ChessBoard, Color, Index, Move, Piece};

/// magic constants
/// See: [ChessProgramming Magic Bitboards](https://www.chessprogramming.org/Magic_Bitboards)
const MAGIC_A1H8: [BitBoard; 15] = [
    BitBoard::new(0x0),
    BitBoard::new(0x0),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x8080808080808000),
    BitBoard::new(0x4040404040400000),
    BitBoard::new(0x2020202020000000),
    BitBoard::new(0x1010101000000000),
    BitBoard::new(0x0808080000000000),
    BitBoard::new(0x0),
    BitBoard::new(0x0),
];

/// magic constants
/// See: [ChessProgramming Magic Bitboards](https://www.chessprogramming.org/Magic_Bitboards)
const MAGIC_A8H1: [BitBoard; 15] = [
    BitBoard::new(0x0),
    BitBoard::new(0x0),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0101010101010100),
    BitBoard::new(0x0080808080808080),
    BitBoard::new(0x0040404040404040),
    BitBoard::new(0x0020202020202020),
    BitBoard::new(0x0010101010101010),
    BitBoard::new(0x0008080808080808),
    BitBoard::new(0x0),
    BitBoard::new(0x0),
];

/// Moves generator for a Bishop (+Queen).
#[derive(Debug)]
pub(super) struct GeneratorBishop {
    pub(super) a1h8_mask: [BitBoard; Index::ALL_FIELDS.len()],
    pub(super) a1h8_magic: [BitBoard; Index::ALL_FIELDS.len()],
    pub(super) a1h8_attacks: [[BitBoard; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()],

    pub(super) a8h1_mask: [BitBoard; Index::ALL_FIELDS.len()],
    pub(super) a8h1_magic: [BitBoard; Index::ALL_FIELDS.len()],
    pub(super) a8h1_attacks: [[BitBoard; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()],
}

impl GeneratorBishop {
    /// Construct a new generator.
    pub(super) const fn new() -> Self {
        let mut a1h8_index = [0usize; Index::ALL_FIELDS.len()];
        let mut a1h8_mask = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut a1h8_magic = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut a1h8_attacks =
            [[BitBoard::EMPTY; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()];

        let mut a8h1_index = [0usize; Index::ALL_FIELDS.len()];
        let mut a8h1_mask = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut a8h1_magic = [BitBoard::EMPTY; Index::ALL_FIELDS.len()];
        let mut a8h1_attacks =
            [[BitBoard::EMPTY; Index::ALL_FIELDS.len()]; Index::ALL_FIELDS.len()];

        let mut i = 0;
        while i < Index::ALL_FIELDS.len() {
            let rank = Index::new(i).rank();
            let file = Index::new(i).file();

            // compute index of diagonal for the field
            a1h8_index[i] = file + 7 - rank % 8;
            a8h1_index[i] = file + rank % 8;

            // compute 6-bit diagonal for the field
            a1h8_mask[i] =
                BitBoard::new(BitBoard::A1H8[a1h8_index[i]].state & !BitBoard::FRAME.state);
            a8h1_mask[i] =
                BitBoard::new(BitBoard::A8H1[a8h1_index[i]].state & !BitBoard::FRAME.state);

            // index magic multiplier for the field
            a1h8_magic[i] = MAGIC_A1H8[a1h8_index[i]];
            a8h1_magic[i] = MAGIC_A8H1[a8h1_index[i]];

            i += 1;
        }

        // precompute A1H8 moves
        // i is field index
        // n is 6 bit configuration
        // for all fields
        i = 0;
        while i < Index::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Index::ALL_FIELDS.len() {
                // get the diagonal
                let mut diagonal = BitBoard::A1H8[a1h8_index[i]];

                // reconstruct the state (number) into the diagonal
                // get the left/bottom bit - start of diagonal
                while diagonal.shifted_southwest().state != 0 {
                    diagonal = diagonal.shifted_southwest();
                }

                // traverse diagonal and set bits according to N
                let mut board = BitBoard::EMPTY;

                let mut m = n as u64;
                while diagonal.state != 0 {
                    // shift down by one
                    diagonal = diagonal.shifted_northeast();
                    if (m & 1) != 0 {
                        board = BitBoard::new(board.state | diagonal.state);
                    }
                    m >>= 1;
                }

                // make it 6-bit only
                board = BitBoard::new(board.state & !BitBoard::FRAME.state);

                // pre-compute moves
                let mut moves = BitBoard::EMPTY;

                // set piece in Ith position
                let mut piece = Index::new(i).as_bitboard();

                // move in one direction
                while piece.state != 0 {
                    piece = piece.shifted_northeast();
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
                    piece = piece.shifted_southwest();
                    moves = BitBoard::new(moves.state | piece.state);

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.state & board.state) != 0 {
                        break;
                    }
                }

                // remember the moves
                a1h8_attacks[i][n] = moves;

                n += 1;
            }

            i += 1;
        }

        // precompute A8H1 moves
        // i is field index
        // n is 6 bit configuration
        // for all fields
        i = 0;
        while i < Index::ALL_FIELDS.len() {
            let mut n = 0;
            // for all occupancy states
            while n < Index::ALL_FIELDS.len() {
                // get the diagonal
                let mut diagonal = BitBoard::A8H1[a8h1_index[i]];

                // get the left/top bit - start of the diagonal
                while diagonal.shifted_northwest().state != 0 {
                    diagonal = diagonal.shifted_northwest();
                }

                // traverse diagonal and set bits according to N
                let mut board = BitBoard::EMPTY;

                let mut m = n as u64;
                while diagonal.state != 0 {
                    // shift down by one
                    diagonal = diagonal.shifted_southeast();
                    if (m & 1) != 0 {
                        board = BitBoard::new(board.state | diagonal.state);
                    }
                    m >>= 1;
                }

                // make it 6-bit only
                board = BitBoard::new(board.state & !BitBoard::FRAME.state);

                // pre-compute moves
                let mut moves = BitBoard::EMPTY;

                // set piece in Ith position
                let mut piece = Index::new(i).as_bitboard();

                // move in one direction
                while piece.state != 0 {
                    piece = piece.shifted_northwest();
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
                    piece = piece.shifted_southeast();
                    moves = BitBoard::new(moves.state | piece.state);

                    // end when there is another piece on the board (either color, own color will have to be stripped out)
                    if (piece.state & board.state) != 0 {
                        break;
                    }
                }

                // remember the moves
                a8h1_attacks[i][n] = moves;

                n += 1;
            }

            i += 1;
        }

        GeneratorBishop {
            a1h8_mask,
            a1h8_magic,
            a1h8_attacks,
            a8h1_mask,
            a8h1_magic,
            a8h1_attacks,
        }
    }

    /// Generate attacks for one piece.
    fn attacks(&self, i: Index, all_pieces: BitBoard) -> BitBoard {
        // use magic multipliers to get occupancy state index

        let index_a1h8 =
            ((all_pieces & self.a1h8_mask[*i]).state * self.a1h8_magic[*i].state) >> 57;
        let index_a8h1 =
            ((all_pieces & self.a8h1_mask[*i]).state * self.a8h1_magic[*i].state) >> 57;

        self.a1h8_attacks[i.index][index_a1h8 as usize]
            | self.a8h1_attacks[i.index][index_a8h1 as usize]
    }

    /// Generate attacks.
    pub(super) fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let mut b = board.pieces[*color][*Piece::Bishop] | board.pieces[*color][*Piece::Queen];
        let mut attacks = BitBoard::EMPTY;

        let all_pieces = board.all_pieces();

        // for all bishops
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

        for p in [*Piece::Bishop, *Piece::Queen] {
            let mut pieces = board.pieces[*board.next_move][p];

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
