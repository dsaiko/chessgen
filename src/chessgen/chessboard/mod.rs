pub use self::chessboard::ChessBoard;
pub use self::color::Color;
pub use self::error::InvalidChessBoardStringError;
pub use self::error::InvalidColorNotationError;
pub use self::error::InvalidFENStringError;
pub use self::error::InvalidMoveNotationError;
pub use self::error::InvalidPieceNotationError;
pub use self::generator::Generator;
pub use self::generator::IllegalMoveError;
pub use self::generator::PerfT;
pub use self::generator::Zobrist;
pub use self::piece::Piece;
pub use self::r#move::Move;

mod board_indices;
#[allow(clippy::module_inception)]
mod chessboard;
mod color;
mod error;
mod generator;
mod r#move;
mod piece;
