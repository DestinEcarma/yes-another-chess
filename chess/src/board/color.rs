use super::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	White,
	Black,
	Both,
}

impl std::ops::Not for Color {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Self::White => Self::Black,
			Self::Black => Self::White,
			Self::Both => Self::Both,
		}
	}
}

impl std::ops::Add<Color> for usize {
	type Output = usize;

	fn add(self, rhs: Color) -> Self::Output {
		self + rhs as usize
	}
}

impl PartialEq<usize> for Color {
	fn eq(&self, other: &usize) -> bool {
		match other {
			0..=5 => *self == Self::White,
			6..=11 => *self == Self::Black,
			_ => false,
		}
	}
}

impl From<char> for Color {
	fn from(value: char) -> Self {
		match value {
			'W' | 'w' => Self::White,
			'B' | 'b' => Self::Black,
			_ => panic!("{}", Error::InvalidColor(value)),
		}
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let color = match self {
			Self::White => "w",
			Self::Black => "b",
			Self::Both => "w/b",
		};

		write!(f, "{color}")
	}
}

impl Color {
	pub const SIZE: usize = 2;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn from_char() {
		assert_eq!(Color::from('W'), Color::White);
		assert_eq!(Color::from('w'), Color::White);
		assert_eq!(Color::from('B'), Color::Black);
		assert_eq!(Color::from('b'), Color::Black);
	}

	#[test]
	fn eq() {
		assert!(Color::White == 0);
		assert!(Color::White != 6);
		assert!(Color::Black == 6);
		assert!(Color::Black != 12);
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", Color::White), "w");
		assert_eq!(format!("{}", Color::Black), "b");
	}
}
