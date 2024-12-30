use super::{color::ColorUtils, Color};

pub type Piece = usize;

pub struct PieceUtils;

impl PieceUtils {
	pub const SIZE: usize = 6;
	pub const RANGE: std::ops::Range<usize> = 0..Self::SIZE;
}

impl PieceUtils {
	pub const PAWN: Piece = 0;
	pub const KNIGHT: Piece = 1;
	pub const BISHOP: Piece = 2;
	pub const ROOK: Piece = 3;
	pub const QUEEN: Piece = 4;
	pub const KING: Piece = 5;
	pub const NONE: Piece = 6;
}

impl PieceUtils {
	pub const PROMOTION_SIZE: usize = 4;
	pub const PROMOTIONS: [Piece; Self::PROMOTION_SIZE] = [
		PieceUtils::KNIGHT,
		PieceUtils::BISHOP,
		PieceUtils::ROOK,
		PieceUtils::QUEEN,
	];
}

impl PieceUtils {
	pub fn parse(value: char) -> Piece {
		match value {
			'P' | 'p' => Self::PAWN,
			'N' | 'n' => Self::KNIGHT,
			'B' | 'b' => Self::BISHOP,
			'R' | 'r' => Self::ROOK,
			'Q' | 'q' => Self::QUEEN,
			'K' | 'k' => Self::KING,
			_ => panic!("Invalid piece: {}", value),
		}
	}

	pub fn to_string(piece: Piece, color: Color) -> String {
		let ch = match piece {
			Self::PAWN => 'P',
			Self::KNIGHT => 'N',
			Self::BISHOP => 'B',
			Self::ROOK => 'R',
			Self::QUEEN => 'Q',
			Self::KING => 'K',
			Self::NONE => return String::from("None"),
			_ => panic!("Invalid piece: {piece}"),
		};

		String::from(match color {
			ColorUtils::WHITE => ch,
			ColorUtils::BLACK => ch.to_ascii_lowercase(),
			ColorUtils::BOTH => {
				let piece = match piece {
					Self::PAWN => "Pawn",
					Self::KNIGHT => "Knight",
					Self::BISHOP => "Bishop",
					Self::ROOK => "Rook",
					Self::QUEEN => "Queen",
					Self::KING => "King",
					_ => return String::from("None"),
				};

				return String::from(piece);
			}
			_ => panic!("Invalid color: {color}"),
		})
	}
}
