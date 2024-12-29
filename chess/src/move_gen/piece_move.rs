// Marcel Vanthoor
// https://github.com/mvanthoor/rustic

use crate::board::{color::Colors, piece::PieceString, square::SquareUtils, Color, Piece, Square};
use std::fmt::{self, Debug};

#[derive(Copy, Clone, PartialEq)]
pub struct Move(pub usize);

impl std::ops::Shr<usize> for &Move {
	type Output = usize;

	#[inline(always)]
	fn shr(self, rhs: usize) -> Self::Output {
		self.0 >> rhs
	}
}

impl Debug for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let piece = self.piece();
		let from = self.from();
		let to = self.to();
		let captured = self.captured();
		let promoted = self.promoted();
		let en_passant = self.en_passant();
		let two_step = self.two_step();
		let castling = self.castling();

		write!(
			f,
			"{}{} {} {} {} {en_passant} {two_step} {castling}",
			SquareUtils::to_string(from),
			SquareUtils::to_string(to),
			piece.piece_string(Color::BOTH),
			captured.piece_string(Color::BOTH),
			promoted.piece_string(Color::BOTH),
		)
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let from = self.from();
		let to = self.to();

		write!(
			f,
			"{}{}",
			SquareUtils::to_string(from),
			SquareUtils::to_string(to)
		)
	}
}

impl Move {
	pub const PIECE: usize = 0;
	pub const FROM_SQUARE: usize = 3;
	pub const TO_SQUARE: usize = 9;
	pub const CAPTURE: usize = 15;
	pub const PROMOTION: usize = 18;
	pub const EN_PASSANT: usize = 21;
	pub const TWO_STEP: usize = 22;
	pub const CASTLING: usize = 23;

	#[inline(always)]
	pub fn new(data: usize) -> Self {
		Self(data)
	}

	#[inline(always)]
	pub fn piece(&self) -> Piece {
		(self >> Self::PIECE) & 0x7
	}

	#[inline(always)]
	pub fn from(&self) -> Square {
		(self >> Move::FROM_SQUARE) & 0x3F
	}

	#[inline(always)]
	pub fn to(&self) -> Square {
		(self >> Move::TO_SQUARE) & 0x3F
	}

	#[inline(always)]
	pub fn captured(&self) -> Piece {
		(self >> Move::CAPTURE) & 0x7
	}

	#[inline(always)]
	pub fn promoted(&self) -> Piece {
		(self >> Move::PROMOTION) & 0x7
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
