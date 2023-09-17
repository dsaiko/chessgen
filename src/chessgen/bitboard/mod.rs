pub use self::bitboard::BitBoard;
pub use self::error::InvalidBitBoardStringError;
pub use self::error::InvalidIndexNotationError;
pub use self::index::Index;

#[allow(clippy::module_inception)]
mod bitboard;
mod bitboard_constants;
mod error;
mod index;
mod index_constants;
mod operators;
