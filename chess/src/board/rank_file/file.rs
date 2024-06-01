use super::{Bitboard, Error, Square};
use std::{fmt, ops};

#[derive(Debug, Clone, Copy)]
pub enum File {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
}

impl Iterator for File {
	type Item = Self;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::A => *self = Self::B,
			Self::B => *self = Self::C,
			Self::C => *self = Self::D,
			Self::D => *self = Self::E,
			Self::E => *self = Self::F,
			Self::F => *self = Self::G,
			Self::G => *self = Self::H,
			Self::H => return None,
		}

		Some(*self)
	}
}

impl ops::Not for File {
	type Output = Bitboard;

	fn not(self) -> Self::Output {
		!Bitboard::from(self)
	}
}

impl ops::Shl<File> for u64 {
	type Output = u64;

	fn shl(self, rhs: File) -> Self::Output {
		self << rhs as u64
	}
}

impl ops::Add<File> for usize {
	type Output = usize;

	fn add(self, rhs: File) -> Self::Output {
		self + rhs as usize
	}
}

impl ops::Add<u8> for File {
	type Output = Self;

	fn add(self, rhs: u8) -> Self::Output {
		println!("{} {}", self as u8, rhs);
		Self::from(self as u8 + rhs)
	}
}

impl ops::AddAssign<u8> for File {
	fn add_assign(&mut self, rhs: u8) {
		*self = *self + rhs;
	}
}

impl From<u8> for File {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::A,
			1 => Self::B,
			2 => Self::C,
			3 => Self::D,
			4 => Self::E,
			5 => Self::F,
			6 => Self::G,
			7 => Self::H,
			_ => panic!("{}", Error::InvalidFile(value)),
		}
	}
}

impl From<char> for File {
	fn from(value: char) -> Self {
		match value {
			'A' | 'a' => Self::A,
			'B' | 'b' => Self::B,
			'C' | 'c' => Self::C,
			'D' | 'd' => Self::D,
			'E' | 'e' => Self::E,
			'F' | 'f' => Self::F,
			'G' | 'g' => Self::G,
			'H' | 'h' => Self::H,
			_ => panic!("{}", Error::InvalidFile(value)),
		}
	}
}

impl From<usize> for File {
	fn from(value: usize) -> Self {
		Self::from(value as u8)
	}
}

impl From<Square> for File {
	fn from(square: Square) -> Self {
		Self::from(square % 8)
	}
}

impl fmt::Display for File {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let file = match self {
			Self::A => "A",
			Self::B => "B",
			Self::C => "C",
			Self::D => "D",
			Self::E => "E",
			Self::F => "F",
			Self::G => "G",
			Self::H => "H",
		};

		write!(f, "{file}")
	}
}

impl File {
	pub fn iter(self) -> FileIterator {
		FileIterator { inner: Some(self) }
	}
}

pub struct FileIterator {
	inner: Option<File>,
}

impl Iterator for FileIterator {
	type Item = File;

	fn next(&mut self) -> Option<Self::Item> {
		let curr = self.inner?;
		let mut file = curr;

		self.inner = file.next();

		Some(curr)
	}
}
