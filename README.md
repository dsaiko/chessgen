# chessgen

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](
https://github.com/dsaiko/chessgen)
[![Cargo](https://img.shields.io/crates/v/fastrand.svg)](
https://crates.io/crates/chessgen)
[![Documentation](https://docs.rs/fastrand/badge.svg)](
https://docs.rs/chessgen)


A simple and fast chess moves generator.

This is my next and probably last rewrite of a chess moves generator into a new programming language.

Since 2014, I have implemented the same engine in:
- C
- [C++](https://github.com/dsaiko/sachista-chess)
- Java
- Scala
- [Swift](https://github.com/dsaiko/sachista-chess-swift)
- [Go](https://github.com/dsaiko/sachista-chess-go)
- [Rust](https://github.com/dsaiko/chessgen)

Rust implementation is the newest and most advanced in terms of code quality and features.

## Performance comparison between different language versions.

[PerfT](https://www.chessprogramming.org/Perft) computes number of all possible moves for a given depth and is a common way how to test performance and result of a chess generator.


Rust implementation is very comparable in performance to C++ and it surprisingly behaves faster for bigger depth of moves. 

The reason for a longer startup of a perft binary is probably an initialisation of a cache array using a loop and Mutex:

```rust
  cache: Box<[Mutex<PerfTCacheEntry>]>
  ...
  let mut cache = Vec::with_capacity(cache_size);
  for _ in 0..cache_size {
      cache.push(Mutex::new(PerfTCacheEntry::new()))
  }
```

vs C++

```c++
    std::atomic<CacheEntry>   *cache;
  ...
    cache = new std::atomic<CacheEntry>[cacheSize];
```

Bellow are perft results for a standard chessboard layout on my AMD Ryzen 7 5800H, Ubuntu 23.04. 

<pre>
PERFT 7:     3,195,901,860 combinations
PERFT 8:    84,998,978,956 combinations
PERFT 9: 2,439,530,234,167 combinations

_______| PERFT 7 _| PERFT 8 __| PERFT 9 __|
C++:   |    1.05s |    28.99s |  2205.02s |
Rust:  |    1.59s |    26.28s |  1213.49s |
GO:    |    2.88s | 2m 16.47s |       --- |
</pre>

## Examples

### Initializing ChessBoard
```rust
use chessgen::ChessBoard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // standard chess board
    let mut board = ChessBoard::STANDARD;

    // creating board from FEN
    board = ChessBoard::from_fen(ChessBoard::STANDARD_BOARD_FEN)?;
    board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;

    // creating board from String representation
    board = ChessBoard::from_string(
        "
      a b c d e f g h
    8 r n b q k b n r 8
    7 p p p p p p p p 7
    6 - - - - - - - - 6
    5 - - - - - - - - 5
    4 - - - - - - - - 4
    3 - - - - - - - - 3
    2 P P P P P P P P 2
    1 R N B Q K B N R 1
      a b c d e f g h
    ",
    )?;

    // creating board from String representation without decoration
    board = ChessBoard::from_string(
        "
     r n b q k b n r 
     p p p p p p p p 
     - - - - - - - - 
     - - - - - - - - 
     - - - - - - - - 
     - - - - - - - - 
     P P P P P P P P 
     R N B Q K B N R 
    ",
    )?;

    println!("{}", board);

    Ok(())
}
```

Output: 
<pre>
  a b c d e f g h
8 r n b q k b n r 8
7 p p p p p p p p 7
6 - - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 P P P P P P P P 2
1 R N B Q K B N R 1
  a b c d e f g h
</pre>

### Generating attacks and moves
See: [ChessProgramming Pseudo Legal Move](https://www.chessprogramming.org/Pseudo-Legal_Move)

```rust
use chessgen::{ChessBoard, Color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = ChessBoard::from_string(
        "
          - - - - q - - k
          - - - - - - - -
          - - - - - - - -
          - - - - - - - -
          - - - - - - - -
          - - - - - - - -
          - - - - B - - -
          - - - Q K - - -
     ",
    )?;

    // attacks
    println!("Attacks:\n{}", board.attacks(Color::Black));

    // legal moves
    print!("Legal moves: ");
    for m in board.legal_moves() {
        print!("{} ", m)
    }
    println!("\n");

    // pseudo legal moves
    print!("Pseudo legal moves: ");
    board.moves(&mut |m| print!("{} ", m));
    println!("\n");

    Ok(())
}
```

Output: 
<pre>
Attacks:
  a b c d e f g h
8 x x x x - x x x 8
7 - - - x x x x x 7
6 - - x - x - x - 6
5 - x - - x - - x 5
4 x - - - x - - - 4
3 - - - - x - - - 3
2 - - - - x - - - 2
1 - - - - - - - - 1
  a b c d e f g h

Legal moves: d1a1 d1b1 d1c1 d1d2 d1d3 d1d4 d1d5 d1d6 d1d7 d1d8 
d1c2 d1b3 d1a4 e1f1 e1d2 e1f2 

Pseudo legal moves: d1a1 d1b1 d1c1 d1d2 d1d3 d1d4 d1d5 d1d6
d1d7 d1d8 e2f1 e2d3 e2f3 e2c4 e2g4 e2b5 e2h5 e2a6 d1c2 d1b3
d1a4 e1f1 e1d2 e1f2 
</pre>

### Applaying a move

```rust
use chessgen::{ChessBoard, Move};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = ChessBoard::from_string(
        "
          - - - - q - - k
          - - - - - - - -
          - - - - - - - -
          - - - - - - - -
          - - - - - - - -
          - - - - - - - -
          - - - - B - - -
          - - - Q K - - -
     ",
    )?;

    board = board.validate_and_apply_move(&Move::from_string("d1d8")?)?;

    println!("{}", board);

    Ok(())
}
```

Output: 
<pre>
  a b c d e f g h
8 - - - Q q - - k 8
7 - - - - - - - - 7
6 - - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 - - - - B - - - 2
1 - - - - K - - - 1
  a b c d e f g h
</pre>

### Running PerfT

```rust
use chessgen::{ChessBoard, PerfT};
use std::time::Instant;

const CACHE_SIZE: usize = 64 * 1024 * 1024;

/// Run PerfT at specific board and depth.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = ChessBoard::STANDARD;
    let depth = 5;

    let start = Instant::now();
    let count = PerfT::new(CACHE_SIZE).perft_n(&board, depth);
    let duration = start.elapsed();

    println!("perfT finished:");
    println!("   FEN:   {}", board.to_fen());
    println!("   depth: {}", depth);
    println!("   count: {}", count);
    println!("   time:  {:?}", duration);

    Ok(())
}
```

Output: 
<pre>
perfT finished:
   FEN:   rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
   depth: 5
   count: 4865609
   time:  1.363183325s
</pre>

### Displaying chess board

You may implement custom display of the chessboard using 
```rust
ChessBoard::piece_at()
```

```rust
use chessgen::{ChessBoard, Index};

/// Run PerfT at specific board and depth.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = ChessBoard::STANDARD;

    for i in Index::ALL_FIELDS {
        if i.index > 0 && i.rank() != Index::new(i.index - 1).rank() {
            println!();
        }

        let file = i.file();
        let rank = i.rank();

        // translate ranks for a display as rank 0 is the most bottom rank
        let translated_i = Index::from_rank_and_file(7 - rank, file);

        let output = match board.piece_at(translated_i) {
            Some((color, piece)) => piece.to_char(color),
            None => '-',
        };

        print!(" {} ", output);
    }
    println!();

    Ok(())
}
```

Output: 
<pre>
 r  n  b  q  k  b  n  r 
 p  p  p  p  p  p  p  p 
 -  -  -  -  -  -  -  - 
 -  -  -  -  -  -  -  - 
 -  -  -  -  -  -  -  - 
 -  -  -  -  -  -  -  - 
 P  P  P  P  P  P  P  P 
 R  N  B  Q  K  B  N  R 
</pre>