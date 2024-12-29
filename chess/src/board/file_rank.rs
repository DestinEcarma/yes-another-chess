pub type File = usize;
pub type Rank = usize;

pub struct RankUtils;

impl RankUtils {
	pub const SIZE: usize = 8;
	pub const RANGE: std::ops::Range<usize> = 0..Self::SIZE;
}

impl RankUtils {
	pub const R1: Rank = 0;
	pub const R2: Rank = 1;
	pub const R3: Rank = 2;
	pub const R4: Rank = 3;
	pub const R5: Rank = 4;
	pub const R6: Rank = 5;
	pub const R7: Rank = 6;
	pub const R8: Rank = 7;
}

impl RankUtils {
	pub fn from_char(value: char) -> Rank {
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

	pub fn to_char(value: Rank) -> char {
		match value {
			Self::R1 => '1',
			Self::R2 => '2',
			Self::R3 => '3',
			Self::R4 => '4',
			Self::R5 => '5',
			Self::R6 => '6',
			Self::R7 => '7',
			Self::R8 => '8',
			_ => panic!("Invalid rank: {}", value),
		}
	}
}

pub struct FileUtils;

impl FileUtils {
	pub const SIZE: usize = RankUtils::SIZE;
	pub const RANGE: std::ops::Range<usize> = RankUtils::RANGE;
}

impl FileUtils {
	pub const A: File = 0;
	pub const B: File = 1;
	pub const C: File = 2;
	pub const D: File = 3;
	pub const E: File = 4;
	pub const F: File = 5;
	pub const G: File = 6;
	pub const H: File = 7;
}

impl FileUtils {
	pub fn from_char(value: char) -> File {
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

	pub fn to_char(value: File) -> char {
		match value {
			Self::A => 'a',
			Self::B => 'b',
			Self::C => 'c',
			Self::D => 'd',
			Self::E => 'e',
			Self::F => 'f',
			Self::G => 'g',
			Self::H => 'h',
			_ => panic!("Invalid file: {}", value),
		}
	}
}
