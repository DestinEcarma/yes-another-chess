use std::fmt;

#[derive(Debug)]
pub enum Error<T: fmt::Display> {
	InvalidRank(T),
	InvalidFile(T),
	InvalidColor(T),
	InvalidPiece(T),
	InvalidSquare(T),
	InvalidPieceBoth(T),
	InvalidCastleRight(T),
	InvalidCastlingMove(T),
}

impl<T: fmt::Display> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = match self {
			Self::InvalidRank(value) => format!("Invalid rank: {value}"),
			Self::InvalidFile(value) => format!("Invalid file: {value}"),
			Self::InvalidColor(value) => format!("Invalid color: {value}"),
			Self::InvalidPiece(value) => format!("Invalid piece: {value}"),
			Self::InvalidSquare(value) => format!("Invalid square: {value}"),
			Self::InvalidCastleRight(value) => format!("Invalid castle right: {value}"),
			Self::InvalidCastlingMove(value) => format!("Invalid castling move: {value}"),
			Self::InvalidPieceBoth(value) => format!("Invalid piece cannot be indexed: {value}"),
		};

		write!(f, "{}", description)
	}
}
