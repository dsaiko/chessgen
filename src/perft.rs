use std::{env, time::Instant};

use chessgen::{ChessBoard, PerfT};

const CACHE_SIZE: usize = 64 * 1024 * 1024;

/// Run PerfT at specific board and depth.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut depth = 7usize;
    let mut fen = ChessBoard::STANDARD_BOARD_FEN.to_string();

    if args.len() == 1 {
        println!(
            "usage: [NO-ARGUMENTS] - running standard layout perft for the default depth of {}",
            depth
        );
        println!("usage: [DEPTH]        - running standard layout perft for the given depth");
        println!("usage: [DEPTH] [FEN]  - running custom board layout perft for the given depth");
        println!();
    }

    if args.len() > 1 {
        depth = args[1].parse()?;
    }

    if args.len() > 2 {
        fen = args[2].to_string();
    }

    let board = ChessBoard::from_fen(&fen)?;

    let start = Instant::now();
    let count = PerfT::new(CACHE_SIZE).perft_n(&board, depth);
    let duration = start.elapsed();

    println!("perfT finished:");
    println!("   FEN:   {}", fen);
    println!("   depth: {}", depth);
    println!("   count: {}", humanize_number(count));
    println!("   time:  {:?}", duration);

    Ok(())
}

/// Helper to output number with thousands separator.
fn humanize_number(n: u64) -> String {
    let mut s = String::new();
    let i_str = n.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
