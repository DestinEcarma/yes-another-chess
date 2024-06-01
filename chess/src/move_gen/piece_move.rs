use crate::board::{Piece, Square};
use std::fmt::{self, Debug};

#[derive(Copy, Clone, PartialEq)]
pub struct Move(pub usize);

impl std::ops::Shr<usize> for &Move {
	type Output = usize;

	fn shr(self, rhs: usize) -> Self::Output {
		self.0 >> rhs
	}
}

impl Debug for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let piece = self.piece();
		let from = self.from();
		let to = self.to();
		let capture = self.captured();
		let promoted = self.promoted();
		let en_passant = self.en_passant();
		let two_step = self.two_step();
		let castling = self.castling();

		write!(
			f,
			"{from}{to} {piece} {capture:?} {promoted:?} {en_passant} {two_step} {castling}"
		)
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let from = self.from();
		let to = self.to();

		write!(f, "{from}{to}")
	}
}

impl Move {
	pub const PIECE: usize = 0;
	pub const FROM_SQUARE: usize = 4;
	pub const TO_SQUARE: usize = 10;
	pub const CAPTURE: usize = 16;
	pub const PROMOTION: usize = 20;
	pub const EN_PASSANT: usize = 24;
	pub const TWO_STEP: usize = 25;
	pub const CASTLING: usize = 26;

	#[inline(always)]
	pub fn new(data: usize) -> Self {
		Self(data)
	}

	#[inline(always)]
	pub fn piece(&self) -> Piece {
		Piece::from(((self >> Self::PIECE) & 0xf) as u8)
	}

	#[inline(always)]
	pub fn from(&self) -> Square {
		Square::from((self >> Move::FROM_SQUARE) & 0x3F)
	}

	#[inline(always)]
	pub fn to(&self) -> Square {
		Square::from((self >> Move::TO_SQUARE) & 0x3F)
	}

	#[inline(always)]
	pub fn captured(&self) -> Option<Piece> {
		let piece = ((self >> Move::CAPTURE) & 0xf) as u8;

		match piece {
			0 => None,
			_ => Some(Piece::from(piece)),
		}
	}

	#[inline(always)]
	pub fn promoted(&self) -> Option<Piece> {
		let piece = ((self >> Move::PROMOTION) & 0xf) as u8;

		match piece {
			0 => None,
			_ => Some(Piece::from(piece)),
		}
	}

	#[inline(always)]
	pub fn en_passant(&self) -> bool {
		(self >> Move::EN_PASSANT) & 0x1 == 1
	}

	#[inline(always)]
	pub fn two_step(&self) -> bool {
		(self >> Move::TWO_STEP) & 0x1 == 1
	}

	#[inline(always)]
	pub fn castling(&self) -> bool {
		(self >> Move::CASTLING) & 0x1 == 1
	}
}
