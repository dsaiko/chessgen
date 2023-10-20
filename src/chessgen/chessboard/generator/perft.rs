use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread;

use crate::{BitBoard, ChessBoard, Piece, Zobrist};

/// PerfT checker.
///
/// See: [ChessProgramming PerfT](https://www.chessprogramming.org/Perft)
#[derive(Debug)]
pub struct PerfT {
    /// PerfT Cache.
    cache: PerfTCache,
    /// Zobrist hasher.
    zobrist: Zobrist,
}

impl PerfT {
    /// Create a new PerfT checker.
    pub fn new(cache_size: usize) -> Self {
        PerfT {
            cache: PerfTCache::new(cache_size),
            zobrist: Zobrist::new(),
        }
    }

    /// Returns count of possible moves up to certain dept.
    /// Runs the computation in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessgen::{ChessBoard, PerfT};
    ///
    /// let perft = PerfT::new(1024 * 1024);
    ///
    /// assert_eq!(
    ///     4_865_609,
    ///     perft.perft_n(&ChessBoard::from_fen(ChessBoard::STANDARD_BOARD_FEN).unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     15_833_292,
    ///     perft.perft_n(&ChessBoard::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     674_624,
    ///     perft.perft_n(&ChessBoard::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     164_075_551,
    ///     perft.perft_n(&ChessBoard::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     15_833_292,
    ///     perft.perft_n(&ChessBoard::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     193_690_690,
    ///     perft.perft_n(&ChessBoard::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     89_941_194,
    ///     perft.perft_n(&ChessBoard::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     0,
    ///     perft.perft_n(&ChessBoard::from_fen("2bqkbn1/3ppppp/8/8/8/8/r7/1r2K3 w - - 0 1").unwrap(), 5)
    /// );
    /// assert_eq!(
    ///     246_680,
    ///     perft.perft_n(&ChessBoard::from_fen("2bqkbn1/3ppppp/8/8/8/8/r7/1r2K3 b - - 0 1").unwrap(), 5)
    /// );
    /// ```
    pub fn perft_n(&self, board: &ChessBoard, depth: usize) -> u64 {
        let (tx, rx) = channel();

        thread::scope(|scope| {
            for m in board.legal_moves() {
                let tx = &tx;
                scope.spawn(move || {
                    let new_board = board.apply_move(&m);
                    tx.send(self.perft1(&new_board, depth - 1)).unwrap();
                });
            }
        });

        drop(tx);
        rx.iter().sum()
    }

    /// Returns count of possible moves up to certain dept.
    /// Runs the computation in one thread only.
    pub fn perft1(&self, board: &ChessBoard, depth: usize) -> u64 {
        if depth == 0 {
            return 1;
        }

        let hash = self.zobrist.hash(board);
        if let Some(count) = self.cache.get(hash, depth) {
            return count;
        }

        let mut count = 0u64;

        let attacks = board.attacks(board.next_move.opponent());
        let is_check = attacks & board.pieces[*board.next_move][*Piece::King] != BitBoard::EMPTY;

        board.moves(&mut |m| {
            let piece = board.piece_at(m.from);
            let is_king = piece == Some((board.next_move, Piece::King));
            let is_enpassant = piece == Some((board.next_move, Piece::Pawn))
                && match board.en_passant_target {
                    Some(i) => m.to == i,
                    None => false,
                };

            // need to validate legality of move only in following cases
            let need_to_validate = is_king || is_check || attacks.has_bit(m.from) || is_enpassant;

            match depth {
                1 => {
                    if !need_to_validate || !board.apply_move(&m).is_opponent_king_under_check() {
                        count += 1;
                    }
                }
                _ => {
                    let new_board = board.apply_move(&m);
                    if !need_to_validate || !new_board.is_opponent_king_under_check() {
                        count += self.perft1(&new_board, depth - 1);
                    }
                }
            }
        });

        self.cache.set(hash, depth, count);

        count
    }
}

/// Cache entry for PerfT.
#[derive(Debug)]
struct PerfTCacheEntry {
    /// ChessBoard hash.
    hash: u64,
    /// Depth of computation.
    depth: usize,
    /// Cached result.
    count: u64,
}

impl PerfTCacheEntry {
    /// Constructs a new PerfTCache Entry.
    fn new() -> Self {
        PerfTCacheEntry {
            hash: 0,
            depth: 0,
            count: 0,
        }
    }
}

/// PerfT Cache.
#[derive(Debug)]
struct PerfTCache {
    /// Cache size.
    size: usize,
    /// Synchronized cache.
    cache: Vec<Mutex<PerfTCacheEntry>>,
}

impl PerfTCache {
    /// Construct a PerfT cache with given size.
    pub fn new(cache_size: usize) -> Self {
        let mut cache = Vec::with_capacity(cache_size);
        for _ in 0..cache_size {
            cache.push(Mutex::new(PerfTCacheEntry::new()))
        }

        PerfTCache {
            size: cache_size,
            cache,
        }
    }

    /// Get cached result.
    #[inline(always)]
    fn get(&self, hash: u64, depth: usize) -> Option<u64> {
        let index = ((self.size - 1) as u64 & hash) as usize;

        let e = self.cache[index].lock().unwrap();
        if e.hash != hash || e.depth != depth {
            None
        } else {
            Some(e.count)
        }
    }

    /// Set cached result.
    #[inline(always)]
    fn set(&self, hash: u64, depth: usize, count: u64) {
        let index = ((self.size - 1) as u64 & hash) as usize;
        (*self.cache[index].lock().unwrap()) = PerfTCacheEntry { hash, depth, count };
    }
}
