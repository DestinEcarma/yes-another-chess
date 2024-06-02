use super::{
	file_rank::{FileString, GetFile, GetRank, RankString},
	File, Rank,
};

pub type Square = usize;

pub trait Squares {
	const A1: Square = 0;
	const B1: Square = 1;
	const C1: Square = 2;
	const D1: Square = 3;
	const E1: Square = 4;
	const F1: Square = 5;
	const G1: Square = 6;
	const H1: Square = 7;
	const A2: Square = 8;
	const B2: Square = 9;
	const C2: Square = 10;
	const D2: Square = 11;
	const E2: Square = 12;
	const F2: Square = 13;
	const G2: Square = 14;
	const H2: Square = 15;
	const A3: Square = 16;
	const B3: Square = 17;
	const C3: Square = 18;
	const D3: Square = 19;
	const E3: Square = 20;
	const F3: Square = 21;
	const G3: Square = 22;
	const H3: Square = 23;
	const A4: Square = 24;
	const B4: Square = 25;
	const C4: Square = 26;
	const D4: Square = 27;
	const E4: Square = 28;
	const F4: Square = 29;
	const G4: Square = 30;
	const H4: Square = 31;
	const A5: Square = 32;
	const B5: Square = 33;
	const C5: Square = 34;
	const D5: Square = 35;
	const E5: Square = 36;
	const F5: Square = 37;
	const G5: Square = 38;
	const H5: Square = 39;
	const A6: Square = 40;
	const B6: Square = 41;
	const C6: Square = 42;
	const D6: Square = 43;
	const E6: Square = 44;
	const F6: Square = 45;
	const G6: Square = 46;
	const H6: Square = 47;
	const A7: Square = 48;
	const B7: Square = 49;
	const C7: Square = 50;
	const D7: Square = 51;
	const E7: Square = 52;
	const F7: Square = 53;
	const G7: Square = 54;
	const H7: Square = 55;
	const A8: Square = 56;
	const B8: Square = 57;
	const C8: Square = 58;
	const D8: Square = 59;
	const E8: Square = 60;
	const F8: Square = 61;
	const G8: Square = 62;
	const H8: Square = 63;
}

pub trait SquareConsts {
	const SQUARE_SIZE: usize = 64;
	const SQUARE_RANGE: std::ops::Range<Square> = 0..Self::SQUARE_SIZE;
}

impl Squares for Square {}
impl SquareConsts for Square {}

pub trait GetSquare<T> {
	fn get_square(value: T) -> Self;
}

impl GetSquare<(File, Rank)> for Square {
	#[inline(always)]
	fn get_square((file, rank): (File, Rank)) -> Self {
		(rank * 8 + file) as Self
	}
}

impl GetSquare<&str> for Square {
	fn get_square(value: &str) -> Self {
		let chars = value.chars().collect::<Vec<char>>();

		match (chars.first(), chars.get(1)) {
			(Some(file), Some(rank)) => {
				let file = File::get_file(*file);
				let rank = Rank::get_rank(*rank);

				(rank * 8 + file) as Self
			}
			_ => panic!("Invalid square: {}", value),
		}
	}
}

pub trait SquareLocation {
	fn location(&self) -> (File, Rank);
}

impl SquareLocation for Square {
	#[inline(always)]
	fn location(&self) -> (File, Rank) {
		((*self % 8) as File, (*self / 8) as Rank)
	}
}

pub trait SquareString {
	fn square_string(&self) -> String;
}

impl SquareString for Square {
	#[inline(always)]
	fn square_string(&self) -> String {
		let file = (*self % 8) as File;
		let rank = (*self / 8) as Rank;

		format!("{}{}", file.file_string(), rank.rank_string())
	}
}
