use super::square::SquareUtils;

pub type CastleRight = u8;

pub struct CastleRightUtils;

impl CastleRightUtils {
	pub const SIZE: usize = 16;
}

impl CastleRightUtils {
	pub const NONE: CastleRight = 0;
	pub const WHITE_KING: CastleRight = 1;
	pub const WHITE_QUEEN: CastleRight = 2;
	pub const BLACK_KING: CastleRight = 4;
	pub const BLACK_QUEEN: CastleRight = 8;
	pub const ALL: CastleRight = 15;

	pub const WHITE: CastleRight = Self::WHITE_KING | Self::WHITE_QUEEN;
	pub const BLACK: CastleRight = Self::BLACK_KING | Self::BLACK_QUEEN;
}

impl CastleRightUtils {
	pub const SQUARES: [CastleRight; SquareUtils::SIZE] = {
		let mut squares = [Self::NONE; SquareUtils::SIZE];

		squares[SquareUtils::A1] = Self::WHITE_QUEEN;
		squares[SquareUtils::E1] = Self::WHITE_KING | Self::WHITE_QUEEN;
		squares[SquareUtils::H1] = Self::WHITE_KING;

		squares[SquareUtils::A8] = Self::BLACK_QUEEN;
		squares[SquareUtils::E8] = Self::BLACK_KING | Self::BLACK_QUEEN;
		squares[SquareUtils::H8] = Self::BLACK_KING;

		squares
	};
}

impl CastleRightUtils {
	pub fn parse(value: char) -> CastleRight {
		match value {
			'K' => Self::WHITE_KING,
			'Q' => Self::WHITE_QUEEN,
			'k' => Self::BLACK_KING,
			'q' => Self::BLACK_QUEEN,
			_ => panic!("Invalid castle right: {}", value),
		}
	}

	pub fn to_string(castle_right: CastleRight) -> String {
		let mut result = String::new();

		if castle_right & Self::WHITE_KING > 0 {
			result.push('K');
		}
		if castle_right & Self::WHITE_QUEEN > 0 {
			result.push('Q');
		}
		if castle_right & Self::BLACK_KING > 0 {
			result.push('k');
		}
		if castle_right & Self::BLACK_QUEEN > 0 {
			result.push('q');
		}

		result
	}
}
