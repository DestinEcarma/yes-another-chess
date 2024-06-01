use super::{Bitboard, Error, File, Rank, RankFile};
use std::{fmt, ops};

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Square {
	A1, B1, C1, D1, E1, F1, G1, H1, // Rank 8
	A2, B2, C2, D2, E2, F2, G2, H2, // Rank 7
	A3, B3, C3, D3, E3, F3, G3, H3, // Rank 6
	A4, B4, C4, D4, E4, F4, G4, H4, // Rank 5
	A5, B5, C5, D5, E5, F5, G5, H5, // Rank 4
	A6, B6, C6, D6, E6, F6, G6, H6, // Rank 3
	A7, B7, C7, D7, E7, F7, G7, H7, // Rank 2
	A8, B8, C8, D8, E8, F8, G8, H8, // Rank 1
}

impl PartialEq<Rank> for Square {
	fn eq(&self, other: &Rank) -> bool {
		(*self / 8) == *other as usize
	}
}

impl PartialEq<File> for Square {
	fn eq(&self, other: &File) -> bool {
		(*self % 8) == *other as usize
	}
}

impl ops::BitOr<Square> for Square {
	type Output = Bitboard;

	fn bitor(self, rhs: Square) -> Self::Output {
		Bitboard::from(self) | Bitboard::from(rhs)
	}
}

impl ops::BitXor<usize> for Square {
	type Output = Self;

	fn bitxor(self, rhs: usize) -> Self::Output {
		Self::from(self as usize ^ rhs)
	}
}

impl ops::Shl<usize> for Square {
	type Output = usize;

	fn shl(self, rhs: usize) -> Self::Output {
		(self as usize) << rhs
	}
}

impl ops::Shl<Square> for u64 {
	type Output = u64;

	fn shl(self, rhs: Square) -> Self::Output {
		self << rhs as usize
	}
}

impl ops::Not for Square {
	type Output = Bitboard;

	fn not(self) -> Self::Output {
		!Bitboard::from(self)
	}
}

impl ops::Add<usize> for Square {
	type Output = Square;

	fn add(self, rhs: usize) -> Self::Output {
		Square::from(self as usize + rhs)
	}
}

impl ops::Sub<usize> for Square {
	type Output = Square;

	fn sub(self, rhs: usize) -> Self::Output {
		Square::from(self as usize - rhs)
	}
}

impl ops::Sub<Square> for Square {
	type Output = i8;

	fn sub(self, rhs: Square) -> Self::Output {
		self as i8 - rhs as i8
	}
}

impl ops::Div<usize> for Square {
	type Output = usize;

	fn div(self, rhs: usize) -> Self::Output {
		self as usize / rhs
	}
}

impl ops::Rem<usize> for Square {
	type Output = usize;

	fn rem(self, rhs: usize) -> Self::Output {
		self as usize % rhs
	}
}

#[rustfmt::skip]
impl From<usize> for Square {
	fn from(value: usize) -> Self {
		match value {
			// Rank 8
			0 => Self::A1,  1 => Self::B1,  2 => Self::C1,  3 => Self::D1,
			4 => Self::E1,  5 => Self::F1,  6 => Self::G1,  7 => Self::H1,
			// Rank 7
			8 => Self::A2,  9 => Self::B2, 10 => Self::C2, 11 => Self::D2,
			12 => Self::E2, 13 => Self::F2, 14 => Self::G2, 15 => Self::H2,
			// Rank 6
			16 => Self::A3, 17 => Self::B3, 18 => Self::C3, 19 => Self::D3,
			20 => Self::E3, 21 => Self::F3, 22 => Self::G3, 23 => Self::H3,
			// Rank 5
			24 => Self::A4, 25 => Self::B4, 26 => Self::C4, 27 => Self::D4,
			28 => Self::E4, 29 => Self::F4, 30 => Self::G4, 31 => Self::H4,
			// Rank 4
			32 => Self::A5, 33 => Self::B5, 34 => Self::C5, 35 => Self::D5,
			36 => Self::E5, 37 => Self::F5, 38 => Self::G5, 39 => Self::H5,
			// Rank 3
			40 => Self::A6, 41 => Self::B6, 42 => Self::C6, 43 => Self::D6,
			44 => Self::E6, 45 => Self::F6, 46 => Self::G6, 47 => Self::H6,
			// Rank 2
			48 => Self::A7, 49 => Self::B7, 50 => Self::C7, 51 => Self::D7,
			52 => Self::E7, 53 => Self::F7, 54 => Self::G7, 55 => Self::H7,
			// Rank 1
			56 => Self::A8, 57 => Self::B8, 58 => Self::C8, 59 => Self::D8,
			60 => Self::E8, 61 => Self::F8, 62 => Self::G8, 63 => Self::H8,
			_ => panic!("{}", Error::InvalidSquare(value)),
		}
	}
}

#[rustfmt::skip]
impl From<&str> for Square {
	fn from(value: &str) -> Self {
		let mut chars = value.chars();

		match (chars.next(), chars.next()) {
			(Some(file), Some(rank)) => Self::from(RankFile::from((file, rank))),
			_ => panic!("{}", Error::InvalidSquare(value)),
		}
	}
}

#[rustfmt::skip]
impl From<RankFile> for Square {
	fn from(value: RankFile) -> Self {
		Self::from(usize::from(value))
	}
}

#[rustfmt::skip]
impl fmt::Display for Square {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			// Rank 8
			Self::A1 => "a1", Self::B1 => "b1", Self::C1 => "c1", Self::D1 => "d1",
			Self::E1 => "e1", Self::F1 => "f1", Self::G1 => "g1", Self::H1 => "h1",
			// Rank 7
			Self::A2 => "a2", Self::B2 => "b2", Self::C2 => "c2", Self::D2 => "d2",
			Self::E2 => "e2", Self::F2 => "f2", Self::G2 => "g2", Self::H2 => "h2",
			// Rank 6
			Self::A3 => "a3", Self::B3 => "b3", Self::C3 => "c3", Self::D3 => "d3",
			Self::E3 => "e3", Self::F3 => "f3", Self::G3 => "g3", Self::H3 => "h3",
			// Rank 5
			Self::A4 => "a4", Self::B4 => "b4", Self::C4 => "c4", Self::D4 => "d4",
			Self::E4 => "e4", Self::F4 => "f4", Self::G4 => "g4", Self::H4 => "h4",
			// Rank 4
			Self::A5 => "a5", Self::B5 => "b5", Self::C5 => "c5", Self::D5 => "d5",
			Self::E5 => "e5", Self::F5 => "f5", Self::G5 => "g5", Self::H5 => "h5",
			// Rank 3
			Self::A6 => "a6", Self::B6 => "b6", Self::C6 => "c6", Self::D6 => "d6",
			Self::E6 => "e6", Self::F6 => "f6", Self::G6 => "g6", Self::H6 => "h6",
			// Rank 2
			Self::A7 => "a7", Self::B7 => "b7", Self::C7 => "c7", Self::D7 => "d7",
			Self::E7 => "e7", Self::F7 => "f7", Self::G7 => "g7", Self::H7 => "h7",
			// Rank 1
			Self::A8 => "a8", Self::B8 => "b8", Self::C8 => "c8", Self::D8 => "d8",
			Self::E8 => "e8", Self::F8 => "f8", Self::G8 => "g8", Self::H8 => "h8",
		})
	}
}

impl Square {
	pub const SIZE: usize = 64;

	pub fn iter() -> SquareIter {
		SquareIter {
			inner: Some(Self::A1),
		}
	}
}

pub struct SquareIter {
	inner: Option<Square>,
}

impl Iterator for SquareIter {
	type Item = Square;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.inner?;

		self.inner = match current {
			Square::H8 => None,
			_ => Some(current + 1),
		};

		Some(current)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn from_usize() {
		assert_eq!(Square::from(0), Square::A1);
		assert_eq!(Square::from(63), Square::H8);
	}

	#[test]
	fn from_str() {
		assert_eq!(Square::from("A1"), Square::A1);
		assert_eq!(Square::from("H8"), Square::H8);
	}
}
