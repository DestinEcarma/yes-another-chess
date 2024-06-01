use super::{square, Bitboard, Color, Error, Piece, Square};
use std::{fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CastleRights(pub u8);

impl ops::Not for CastleRights {
	type Output = CastleRights;

	fn not(self) -> Self::Output {
		Self(!self.0)
	}
}

impl ops::BitAnd<Color> for CastleRights {
	type Output = u8;

	fn bitand(self, rhs: Color) -> Self::Output {
		match rhs {
			Color::White => self.0 & 0b0011,
			Color::Black => self.0 & 0b1100,
			_ => 0,
		}
	}
}

impl ops::BitAnd<CastleRight> for &CastleRights {
	type Output = bool;

	fn bitand(self, rhs: CastleRight) -> Self::Output {
		self.0 & rhs as u8 != 0
	}
}

impl ops::BitOrAssign<CastleRight> for CastleRights {
	fn bitor_assign(&mut self, rhs: CastleRight) {
		self.0 |= rhs as u8;
	}
}

impl ops::BitAndAssign<CastleRights> for CastleRights {
	fn bitand_assign(&mut self, rhs: CastleRights) {
		self.0 &= rhs.0;
	}
}

impl ops::BitAndAssign<Bitboard> for CastleRights {
	fn bitand_assign(&mut self, rhs: Bitboard) {
		self.0 &= rhs.0 as u8;
	}
}

impl fmt::Display for CastleRights {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut castle_rights = String::new();

		for castle_right in CastleRight::iter() {
			if self & castle_right {
				castle_rights += &format!("{castle_right}")
			}
		}

		write!(f, "{castle_rights}")
	}
}

impl CastleRights {
	#[inline(always)]
	pub fn square(square: Square) -> CastleRights {
		match square {
			Square::H1 => Self(0b0001),
			Square::A1 => Self(0b0010),
			Square::E1 => Self(0b0011),
			// Black castle rights
			Square::H8 => Self(0b0100),
			Square::A8 => Self(0b1000),
			Square::E8 => Self(0b1100),
			_ => Self(0),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CastleRight {
	WhiteKing = 0b0001,
	WhiteQueen = 0b0010,
	BlackKing = 0b0100,
	BlackQueen = 0b1000,
}

impl From<char> for CastleRight {
	fn from(value: char) -> Self {
		match value {
			'K' => Self::WhiteKing,
			'Q' => Self::WhiteQueen,
			'k' => Self::BlackKing,
			'q' => Self::BlackQueen,
			_ => panic!("{}", Error::InvalidCastleRight(value)),
		}
	}
}

impl From<u8> for CastleRight {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::WhiteKing,
			1 => Self::WhiteQueen,
			2 => Self::BlackKing,
			3 => Self::BlackQueen,
			_ => panic!("{}", Error::InvalidCastleRight(value)),
		}
	}
}

impl fmt::Display for CastleRight {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let castle_right = match self {
			Self::WhiteKing => "K",
			Self::WhiteQueen => "Q",
			Self::BlackKing => "k",
			Self::BlackQueen => "q",
		};

		write!(f, "{castle_right}")
	}
}

impl CastleRight {
	pub fn iter() -> CastleRightIter {
		CastleRightIter {
			inner: Some(Self::WhiteKing),
		}
	}

	#[inline(always)]
	pub fn blocker(&self) -> Bitboard {
		match self {
			Self::WhiteKing => Square::F1 | Square::G1,
			Self::WhiteQueen => Square::B1 | Square::C1 | Square::D1,
			Self::BlackKing => Square::F8 | Square::G8,
			Self::BlackQueen => Square::B8 | Square::C8 | Square::D8,
		}
	}

	#[inline(always)]
	pub fn neighbor(&self) -> Square {
		match self {
			Self::WhiteKing => Square::F1,
			Self::WhiteQueen => Square::D1,
			Self::BlackKing => Square::F8,
			Self::BlackQueen => Square::D8,
		}
	}

	#[inline(always)]
	pub fn square(&self) -> Square {
		match self {
			Self::WhiteKing => Square::G1,
			Self::WhiteQueen => Square::C1,
			Self::BlackKing => Square::G8,
			Self::BlackQueen => Square::C8,
		}
	}
}

pub struct CastleRightIter {
	inner: Option<CastleRight>,
}

impl Iterator for CastleRightIter {
	type Item = CastleRight;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.inner;

		self.inner = match current {
			Some(CastleRight::WhiteKing) => Some(CastleRight::WhiteQueen),
			Some(CastleRight::WhiteQueen) => Some(CastleRight::BlackKing),
			Some(CastleRight::BlackKing) => Some(CastleRight::BlackQueen),
			Some(CastleRight::BlackQueen) => None,
			None => None,
		};

		current
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn bit_or_assign() {
		let mut castle_rights = CastleRights::default();

		castle_rights |= CastleRight::WhiteKing;
		assert_eq!(castle_rights, CastleRights(0b0001));

		castle_rights |= CastleRight::WhiteQueen;
		assert_eq!(castle_rights, CastleRights(0b0011));

		castle_rights |= CastleRight::BlackKing;
		assert_eq!(castle_rights, CastleRights(0b0111));

		castle_rights |= CastleRight::BlackQueen;
		assert_eq!(castle_rights, CastleRights(0b1111));
	}

	#[test]
	fn from_char() {
		assert_eq!(CastleRight::from('K'), CastleRight::WhiteKing);
		assert_eq!(CastleRight::from('Q'), CastleRight::WhiteQueen);
		assert_eq!(CastleRight::from('k'), CastleRight::BlackKing);
		assert_eq!(CastleRight::from('q'), CastleRight::BlackQueen);
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", CastleRight::WhiteKing), "K");
		assert_eq!(format!("{}", CastleRight::WhiteQueen), "Q");
		assert_eq!(format!("{}", CastleRight::BlackKing), "k");
		assert_eq!(format!("{}", CastleRight::BlackQueen), "q");
	}
}
