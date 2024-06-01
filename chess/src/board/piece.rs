use crate::board::Rank;

use super::{Bitboard, Color, Error};
use std::{fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
	King(Color),
	Queen(Color),
	Rook(Color),
	Bishop(Color),
	Knight(Color),
	Pawn(Color),
}

impl From<char> for Piece {
	fn from(value: char) -> Self {
		match value {
			'K' => Self::King(Color::White),
			'Q' => Self::Queen(Color::White),
			'R' => Self::Rook(Color::White),
			'B' => Self::Bishop(Color::White),
			'N' => Self::Knight(Color::White),
			'P' => Self::Pawn(Color::White),
			// Black pieces
			'k' => Self::King(Color::Black),
			'q' => Self::Queen(Color::Black),
			'r' => Self::Rook(Color::Black),
			'b' => Self::Bishop(Color::Black),
			'n' => Self::Knight(Color::Black),
			'p' => Self::Pawn(Color::Black),
			_ => panic!("{}", Error::InvalidPiece(value)),
		}
	}
}

impl From<u8> for Piece {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::King(Color::White),
			1 => Self::Queen(Color::White),
			2 => Self::Rook(Color::White),
			3 => Self::Bishop(Color::White),
			4 => Self::Knight(Color::White),
			5 => Self::Pawn(Color::White),
			// Black pieces
			6 => Self::King(Color::Black),
			7 => Self::Queen(Color::Black),
			8 => Self::Rook(Color::Black),
			9 => Self::Bishop(Color::Black),
			10 => Self::Knight(Color::Black),
			11 => Self::Pawn(Color::Black),
			_ => panic!("{}", Error::InvalidPiece(value)),
		}
	}
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let piece = match self {
			Self::King(Color::White) => "K",
			Self::Queen(Color::White) => "Q",
			Self::Rook(Color::White) => "R",
			Self::Bishop(Color::White) => "B",
			Self::Knight(Color::White) => "N",
			Self::Pawn(Color::White) => "P",
			// Black pieces
			Self::King(Color::Black) => "k",
			Self::Queen(Color::Black) => "q",
			Self::Rook(Color::Black) => "r",
			Self::Bishop(Color::Black) => "b",
			Self::Knight(Color::Black) => "n",
			Self::Pawn(Color::Black) => "p",
			// Both colors
			Self::King(Color::Both) => "King",
			Self::Queen(Color::Both) => "Queen",
			Self::Rook(Color::Both) => "Rook",
			Self::Bishop(Color::Both) => "Bishop",
			Self::Knight(Color::Both) => "Knight",
			Self::Pawn(Color::Both) => "Pawn",
		};

		write!(f, "{piece}")
	}
}

impl Piece {
	pub const SIZE: usize = 6;

	#[inline(always)]
	pub const fn raw_index(&self) -> usize {
		match self {
			Self::King(_) => 0,
			Self::Queen(_) => 1,
			Self::Rook(_) => 2,
			Self::Bishop(_) => 3,
			Self::Knight(_) => 4,
			Self::Pawn(_) => 5,
		}
	}

	#[inline(always)]
	pub fn index(&self) -> usize {
		let color = match self.color() {
			Color::White => 0,
			Color::Black => 6,
			Color::Both => panic!("{}", Error::InvalidPieceBoth(self)),
		};

		self.raw_index() + color
	}

	#[inline(always)]
	pub fn color(&self) -> Color {
		match self {
			Self::King(color)
			| Self::Queen(color)
			| Self::Rook(color)
			| Self::Bishop(color)
			| Self::Knight(color)
			| Self::Pawn(color) => *color,
		}
	}

	#[inline(always)]
	pub fn promotions(color: Color) -> [Piece; 4] {
		match color {
			Color::White => [
				Self::Queen(Color::White),
				Self::Rook(Color::White),
				Self::Bishop(Color::White),
				Self::Knight(Color::White),
			],
			Color::Black => [
				Self::Queen(Color::Black),
				Self::Rook(Color::Black),
				Self::Bishop(Color::Black),
				Self::Knight(Color::Black),
			],
			Color::Both => panic!("{}", Error::InvalidColor(color)),
		}
	}

	pub fn iter(self) -> PieceIterator {
		PieceIterator { inner: Some(self) }
	}
}

pub struct PieceIterator {
	inner: Option<Piece>,
}

impl Iterator for PieceIterator {
	type Item = Piece;

	fn next(&mut self) -> Option<Self::Item> {
		let piece = self.inner?;

		self.inner = match piece {
			Piece::King(color) => Some(Piece::Queen(color)),
			Piece::Queen(color) => Some(Piece::Rook(color)),
			Piece::Rook(color) => Some(Piece::Bishop(color)),
			Piece::Bishop(color) => Some(Piece::Knight(color)),
			Piece::Knight(color) => Some(Piece::Pawn(color)),
			Piece::Pawn(color) => None,
		};

		Some(piece)
	}
}

impl DoubleEndedIterator for PieceIterator {
	fn next_back(&mut self) -> Option<Self::Item> {
		let piece = self.inner?;

		self.inner = match piece {
			Piece::Pawn(color) => Some(Piece::Knight(color)),
			Piece::Knight(color) => Some(Piece::Bishop(color)),
			Piece::Bishop(color) => Some(Piece::Rook(color)),
			Piece::Rook(color) => Some(Piece::Queen(color)),
			Piece::Queen(color) => Some(Piece::King(color)),
			Piece::King(_) => None,
		};

		Some(piece)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn from_char() {
		assert_eq!(Piece::from('K'), Piece::King(Color::White));
		assert_eq!(Piece::from('Q'), Piece::Queen(Color::White));
		assert_eq!(Piece::from('R'), Piece::Rook(Color::White));
		assert_eq!(Piece::from('B'), Piece::Bishop(Color::White));
		assert_eq!(Piece::from('N'), Piece::Knight(Color::White));
		assert_eq!(Piece::from('P'), Piece::Pawn(Color::White));
		// Black pieces
		assert_eq!(Piece::from('k'), Piece::King(Color::Black));
		assert_eq!(Piece::from('q'), Piece::Queen(Color::Black));
		assert_eq!(Piece::from('r'), Piece::Rook(Color::Black));
		assert_eq!(Piece::from('b'), Piece::Bishop(Color::Black));
		assert_eq!(Piece::from('n'), Piece::Knight(Color::Black));
		assert_eq!(Piece::from('p'), Piece::Pawn(Color::Black));
	}

	#[test]
	fn from_usize() {
		assert_eq!(Piece::from(0), Piece::King(Color::White));
		assert_eq!(Piece::from(1), Piece::Queen(Color::White));
		assert_eq!(Piece::from(2), Piece::Rook(Color::White));
		assert_eq!(Piece::from(3), Piece::Bishop(Color::White));
		assert_eq!(Piece::from(4), Piece::Knight(Color::White));
		assert_eq!(Piece::from(5), Piece::Pawn(Color::White));
		// Black pieces
		assert_eq!(Piece::from(6), Piece::King(Color::Black));
		assert_eq!(Piece::from(7), Piece::Queen(Color::Black));
		assert_eq!(Piece::from(8), Piece::Rook(Color::Black));
		assert_eq!(Piece::from(9), Piece::Bishop(Color::Black));
		assert_eq!(Piece::from(10), Piece::Knight(Color::Black));
		assert_eq!(Piece::from(11), Piece::Pawn(Color::Black));
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", Piece::King(Color::White)), "K");
		assert_eq!(format!("{}", Piece::Queen(Color::White)), "Q");
		assert_eq!(format!("{}", Piece::Rook(Color::White)), "R");
		assert_eq!(format!("{}", Piece::Bishop(Color::White)), "B");
		assert_eq!(format!("{}", Piece::Knight(Color::White)), "N");
		assert_eq!(format!("{}", Piece::Pawn(Color::White)), "P");
		// Black pieces
		assert_eq!(format!("{}", Piece::King(Color::Black)), "k");
		assert_eq!(format!("{}", Piece::Queen(Color::Black)), "q");
		assert_eq!(format!("{}", Piece::Rook(Color::Black)), "r");
		assert_eq!(format!("{}", Piece::Bishop(Color::Black)), "b");
		assert_eq!(format!("{}", Piece::Knight(Color::Black)), "n");
		assert_eq!(format!("{}", Piece::Pawn(Color::Black)), "p");
	}
}
