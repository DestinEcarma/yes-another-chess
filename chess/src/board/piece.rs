use super::{color::ColorUtils, Color};

pub type Piece = usize;

pub trait Pieces {
	const PAWN: Piece = 0;
	const KNIGHT: Piece = 1;
	const BISHOP: Piece = 2;
	const ROOK: Piece = 3;
	const QUEEN: Piece = 4;
	const KING: Piece = 5;
	const NONE: Piece = 6;
}

pub trait PieceConsts {
	const PIECE_SIZE: usize = 6;
	const PIECE_RANGE: std::ops::Range<usize> = 0..Self::PIECE_SIZE;
}

pub trait PiecePromotions {
	const PROMOTIONS: [Piece; 4] = [Piece::KNIGHT, Piece::BISHOP, Piece::ROOK, Piece::QUEEN];
}

impl Pieces for Piece {}
impl PieceConsts for usize {}
impl PiecePromotions for Piece {}

pub trait GetPiece<T> {
	fn get_piece(value: T) -> Self;
}

impl GetPiece<char> for Piece {
	fn get_piece(value: char) -> Self {
		match value {
			'P' | 'p' => Self::PAWN,
			'N' | 'n' => Self::KNIGHT,
			'B' | 'b' => Self::BISHOP,
			'R' | 'r' => Self::ROOK,
			'Q' | 'q' => Self::QUEEN,
			'K' | 'k' => Self::KING,
			_ => panic!("Invalid piece: {value}"),
		}
	}
}

pub trait PieceString {
	fn piece_string(&self, color: Color) -> String;
}

impl PieceString for Piece {
	fn piece_string(&self, color: Color) -> String {
		let ch = match *self {
			Piece::PAWN => 'P',
			Piece::KNIGHT => 'N',
			Piece::BISHOP => 'B',
			Piece::ROOK => 'R',
			Piece::QUEEN => 'Q',
			Piece::KING => 'K',
			Piece::NONE => return String::from("None"),
			_ => panic!("Invalid piece: {self}"),
		};

		String::from(match color {
			ColorUtils::WHITE => ch,
			ColorUtils::BLACK => ch.to_ascii_lowercase(),
			ColorUtils::BOTH => {
				let piece = match *self {
					Piece::PAWN => "Pawn",
					Piece::KNIGHT => "Knight",
					Piece::BISHOP => "Bishop",
					Piece::ROOK => "Rook",
					Piece::QUEEN => "Queen",
					Piece::KING => "King",
					_ => return String::from("None"),
				};

				return String::from(piece);
			}
			_ => panic!("Invalid color: {color}"),
		})
	}
}
