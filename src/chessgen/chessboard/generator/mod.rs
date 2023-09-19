pub use self::error::IllegalMoveError;
pub use self::generator::Generator;
pub use self::perft::PerfT;
pub use self::zobrist::Zobrist;

mod error;
#[allow(clippy::module_inception)]
mod generator;
mod generator_bishop;
mod generator_king;
mod generator_knight;
mod generator_pawn;
mod generator_rook;
mod perft;
mod zobrist;
