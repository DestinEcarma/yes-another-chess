pub type Color = usize;

pub struct ColorUtils;

impl ColorUtils {
	pub const SIZE: usize = 2;
	pub const RANGE: std::ops::Range<Color> = 0..Self::SIZE;
}

impl ColorUtils {
	pub const WHITE: Color = 0;
	pub const BLACK: Color = 1;
	pub const BOTH: Color = 2;
}

impl ColorUtils {
	pub fn parse(value: char) -> Color {
		match value {
			'w' | 'W' => Self::WHITE,
			'b' | 'B' => Self::BLACK,
			_ => panic!("Invalid color: {}", value),
		}
	}

	pub fn from_bool(value: bool) -> Color {
		match value {
			true => Self::WHITE,
			false => Self::BLACK,
		}
	}

	pub fn to_string(value: Color) -> String {
		String::from(match value {
			Self::WHITE => "w",
			Self::BLACK => "b",
			_ => panic!("Invalid color: {}", value),
		})
	}
}
