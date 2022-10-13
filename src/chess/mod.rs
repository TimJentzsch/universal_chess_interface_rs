//! Several general chess constructs needed for UCI, such as moves.

mod file;
mod r#move;
mod promotion_piece;
mod rank;
mod square;

pub use file::*;
pub use promotion_piece::*;
pub use r#move::*;
pub use rank::*;
pub use square::*;

/// An error occured while parsing the given text.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;