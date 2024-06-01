use std::fmt;

pub enum Error<T: fmt::Display> {
	InvalidColor(T),
	InvalidMagicPiece(T),
	InvalidMagicAttack(T),
}

impl<T: fmt::Display> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = match self {
			Self::InvalidColor(value) => format!("Invalid color: {}", value),
			Self::InvalidMagicPiece(value) => format!("Invalid magic piece: {}", value),
			Self::InvalidMagicAttack(value) => format!("Invalid magic attack: {}", value),
		};

		write!(f, "{}", description)
	}
}
