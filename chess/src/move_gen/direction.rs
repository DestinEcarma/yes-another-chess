use crate::board::{
	file_rank::{Files, Ranks},
	square::SquareUtils,
	File, Rank, Square,
};
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
		let (file, rank) = SquareUtils::location(*other);

		match self {
			Self::North => rank == Rank::R8,
			Self::South => rank == Rank::R1,
			Self::East => file == File::H,
			Self::West => file == File::A,
			Self::NorthEast => rank == Rank::R8 || file == File::H,
			Self::NorthWest => rank == Rank::R8 || file == File::A,
			Self::SouthEast => rank == Rank::R1 || file == File::H,
			Self::SouthWest => rank == Rank::R1 || file == File::A,
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
