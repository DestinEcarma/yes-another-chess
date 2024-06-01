use crate::board::{File, Rank, Square};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
	North,
	South,
	East,
	West,
	NorthEast,
	NorthWest,
	SouthEast,
	SouthWest,
}

impl PartialEq<Square> for Direction {
	fn eq(&self, other: &Square) -> bool {
		match self {
			Self::North => *other == Rank::Eighth,
			Self::South => *other == Rank::First,
			Self::East => *other == File::H,
			Self::West => *other == File::A,
			Self::NorthEast => *other == Rank::Eighth || *other == File::H,
			Self::NorthWest => *other == Rank::Eighth || *other == File::A,
			Self::SouthEast => *other == Rank::First || *other == File::H,
			Self::SouthWest => *other == Rank::First || *other == File::A,
		}
	}
}

impl ops::Add<Direction> for usize {
	type Output = Self;

	fn add(self, rhs: Direction) -> Self::Output {
		match rhs {
			Direction::North => self + 8,
			Direction::South => self - 8,
			Direction::East => self + 1,
			Direction::West => self - 1,
			Direction::NorthEast => self + 9,
			Direction::NorthWest => self + 7,
			Direction::SouthEast => self - 7,
			Direction::SouthWest => self - 9,
		}
	}
}

impl ops::Add<Direction> for Square {
	type Output = Self;

	fn add(self, rhs: Direction) -> Self::Output {
		match rhs {
			Direction::North => self + 8,
			Direction::South => self - 8,
			Direction::East => self + 1,
			Direction::West => self - 1,
			Direction::NorthEast => self + 9,
			Direction::NorthWest => self + 7,
			Direction::SouthEast => self - 7,
			Direction::SouthWest => self - 9,
		}
	}
}

impl ops::AddAssign<Direction> for Square {
	fn add_assign(&mut self, rhs: Direction) {
		*self = *self + rhs;
	}
}
