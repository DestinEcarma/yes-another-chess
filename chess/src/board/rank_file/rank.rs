use super::{file::File, Bitboard, Error, Square};
use std::{fmt, ops};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
	First,
	Second,
	Third,
	Fourth,
	Fifth,
	Sixth,
	Seventh,
	Eighth,
}

impl Iterator for Rank {
	type Item = Self;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::First => *self = Self::Second,
			Self::Second => *self = Self::Third,
			Self::Third => *self = Self::Fourth,
			Self::Fourth => *self = Self::Fifth,
			Self::Fifth => *self = Self::Sixth,
			Self::Sixth => *self = Self::Seventh,
			Self::Seventh => *self = Self::Eighth,
			Self::Eighth => return None,
		}

		Some(*self)
	}
}

impl DoubleEndedIterator for Rank {
	fn next_back(&mut self) -> Option<Self::Item> {
		match self {
			Self::First => return None,
			Self::Second => *self = Self::First,
			Self::Third => *self = Self::Second,
			Self::Fourth => *self = Self::Third,
			Self::Fifth => *self = Self::Fourth,
			Self::Sixth => *self = Self::Fifth,
			Self::Seventh => *self = Self::Sixth,
			Self::Eighth => *self = Self::Seventh,
		}

		Some(*self)
	}
}

impl ops::Not for Rank {
	type Output = Bitboard;

	fn not(self) -> Self::Output {
		!Bitboard::from(self)
	}
}

impl ops::BitOr<File> for Rank {
	type Output = Bitboard;

	fn bitor(self, rhs: File) -> Self::Output {
		Bitboard::from(self) | Bitboard::from(rhs)
	}
}

impl ops::Shl<Rank> for u64 {
	type Output = u64;

	fn shl(self, rhs: Rank) -> Self::Output {
		self << (rhs as u8 * 8)
	}
}

impl ops::Mul<usize> for Rank {
	type Output = usize;

	fn mul(self, rhs: usize) -> Self::Output {
		self as usize * rhs
	}
}

impl ops::Index<Rank> for Vec<String> {
	type Output = String;

	fn index(&self, index: Rank) -> &Self::Output {
		&self[index as usize]
	}
}

impl From<u8> for Rank {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::First,
			1 => Self::Second,
			2 => Self::Third,
			3 => Self::Fourth,
			4 => Self::Fifth,
			5 => Self::Sixth,
			6 => Self::Seventh,
			7 => Self::Eighth,
			_ => panic!("{}", Error::InvalidRank(value)),
		}
	}
}

impl From<char> for Rank {
	fn from(value: char) -> Self {
		match value {
			'1' => Self::First,
			'2' => Self::Second,
			'3' => Self::Third,
			'4' => Self::Fourth,
			'5' => Self::Fifth,
			'6' => Self::Sixth,
			'7' => Self::Seventh,
			'8' => Self::Eighth,
			_ => panic!("{}", Error::InvalidRank(value)),
		}
	}
}

impl From<usize> for Rank {
	fn from(value: usize) -> Self {
		Self::from(value as u8)
	}
}

impl From<Square> for Rank {
	fn from(square: Square) -> Self {
		Self::from(square / 8)
	}
}

impl fmt::Display for Rank {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let rank = match self {
			Self::First => "1",
			Self::Second => "2",
			Self::Third => "3",
			Self::Fourth => "4",
			Self::Fifth => "5",
			Self::Sixth => "6",
			Self::Seventh => "7",
			Self::Eighth => "8",
		};

		write!(f, "{rank}")
	}
}

impl Rank {
	pub fn iter(self) -> RankIterator {
		RankIterator { inner: Some(self) }
	}
}

pub struct RankIterator {
	inner: Option<Rank>,
}

impl Iterator for RankIterator {
	type Item = Rank;

	fn next(&mut self) -> Option<Self::Item> {
		let curr = self.inner?;
		let mut rank = curr;

		self.inner = rank.next();

		Some(curr)
	}
}

impl DoubleEndedIterator for RankIterator {
	fn next_back(&mut self) -> Option<Self::Item> {
		let curr = self.inner?;
		let mut rank = curr;

		self.inner = rank.next_back();

		Some(curr)
	}
}
