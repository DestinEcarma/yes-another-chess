pub type File = usize;
pub type Rank = usize;

pub trait Files {
	const A: File = 0;
	const B: File = 1;
	const C: File = 2;
	const D: File = 3;
	const E: File = 4;
	const F: File = 5;
	const G: File = 6;
	const H: File = 7;
}

pub trait Ranks {
	const R1: Rank = 0;
	const R2: Rank = 1;
	const R3: Rank = 2;
	const R4: Rank = 3;
	const R5: Rank = 4;
	const R6: Rank = 5;
	const R7: Rank = 6;
	const R8: Rank = 7;
}

pub trait FileRankConsts {
	const FILE_RANK_SIZE: usize = 8;
	const FILE_RANK_RANGE: std::ops::Range<usize> = 0..Self::FILE_RANK_SIZE;
}

impl Files for File {}
impl Ranks for Rank {}
impl FileRankConsts for usize {}

pub trait GetFile<T> {
	fn get_file(value: T) -> Self;
}

impl GetFile<char> for File {
	fn get_file(value: char) -> Self {
		match value {
			'a' => Self::A,
			'b' => Self::B,
			'c' => Self::C,
			'd' => Self::D,
			'e' => Self::E,
			'f' => Self::F,
			'g' => Self::G,
			'h' => Self::H,
			_ => panic!("Invalid file: {}", value),
		}
	}
}

pub trait GetRank<T> {
	fn get_rank(value: T) -> Self;
}

impl GetRank<char> for Rank {
	fn get_rank(value: char) -> Self {
		match value {
			'1' => Self::R1,
			'2' => Self::R2,
			'3' => Self::R3,
			'4' => Self::R4,
			'5' => Self::R5,
			'6' => Self::R6,
			'7' => Self::R7,
			'8' => Self::R8,
			_ => panic!("Invalid rank: {}", value),
		}
	}
}

pub trait FileString {
	fn file_string(&self) -> String;
}

impl FileString for File {
	#[inline(always)]
	fn file_string(&self) -> String {
		String::from(match *self {
			Self::A => "a",
			Self::B => "b",
			Self::C => "c",
			Self::D => "d",
			Self::E => "e",
			Self::F => "f",
			Self::G => "g",
			Self::H => "h",
			_ => panic!("Invalid file: {self}"),
		})
	}
}

pub trait RankString {
	fn rank_string(&self) -> String;
}

impl RankString for Rank {
	#[inline(always)]
	fn rank_string(&self) -> String {
		String::from(match *self {
			Self::R1 => "1",
			Self::R2 => "2",
			Self::R3 => "3",
			Self::R4 => "4",
			Self::R5 => "5",
			Self::R6 => "6",
			Self::R7 => "7",
			Self::R8 => "8",
			_ => panic!("Invalid rank: {self}"),
		})
	}
}
