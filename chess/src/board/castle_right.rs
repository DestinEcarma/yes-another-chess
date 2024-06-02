use super::{square::SquareConsts, square::Squares, Square};

pub type CastleRight = u8;

pub trait CastleRights {
	const NONE: CastleRight = 0;
	const WHITE_KING: CastleRight = 1;
	const WHITE_QUEEN: CastleRight = 2;
	const BLACK_KING: CastleRight = 4;
	const BLACK_QUEEN: CastleRight = 8;
	const ALL: CastleRight = 15;

	const WHITE: CastleRight = Self::WHITE_KING | Self::WHITE_QUEEN;
	const BLACK: CastleRight = Self::BLACK_KING | Self::BLACK_QUEEN;
}

pub trait CastleRightSquares {
	const SQUARES: [CastleRight; usize::SQUARE_SIZE] = {
		let mut squares = [CastleRight::NONE; usize::SQUARE_SIZE];

		squares[Square::A1] = CastleRight::WHITE_QUEEN;
		squares[Square::E1] = CastleRight::WHITE_KING | CastleRight::WHITE_QUEEN;
		squares[Square::H1] = CastleRight::WHITE_KING;

		squares[Square::A8] = CastleRight::BLACK_QUEEN;
		squares[Square::E8] = CastleRight::BLACK_KING | CastleRight::BLACK_QUEEN;
		squares[Square::H8] = CastleRight::BLACK_KING;

		squares
	};
}

impl CastleRights for CastleRight {}
impl CastleRightSquares for CastleRight {}

pub trait GetCastleRight<T> {
	fn get_castle_right(value: T) -> Self;
}

impl GetCastleRight<char> for CastleRight {
	fn get_castle_right(value: char) -> Self {
		match value {
			'K' => Self::WHITE_KING,
			'Q' => Self::WHITE_QUEEN,
			'k' => Self::BLACK_KING,
			'q' => Self::BLACK_QUEEN,
			_ => panic!("Invalid castle right: {}", value),
		}
	}
}

pub trait CastleRightString {
	fn castle_right_string(&self) -> String;
}

impl CastleRightString for CastleRight {
	fn castle_right_string(&self) -> String {
		let mut result = String::new();

		if self & CastleRight::WHITE_KING > 0 {
			result.push('K');
		}
		if self & CastleRight::WHITE_QUEEN > 0 {
			result.push('Q');
		}
		if self & CastleRight::BLACK_KING > 0 {
			result.push('k');
		}
		if self & CastleRight::BLACK_QUEEN > 0 {
			result.push('q');
		}

		result
	}
}
