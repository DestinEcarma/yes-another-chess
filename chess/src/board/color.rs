pub type Color = usize;

pub trait Colors {
	const WHITE: Color = 0;
	const BLACK: Color = 1;
	const BOTH: Color = 2;
}

pub trait ColorConsts {
	const COLOR_SIZE: usize = 2;
	const COLOR_RANGE: std::ops::Range<Color> = 0..Self::COLOR_SIZE;
}

impl Colors for Color {}
impl ColorConsts for usize {}

pub trait GetColor<T> {
	fn get_color(value: T) -> Self;
}

impl GetColor<char> for Color {
	fn get_color(value: char) -> Self {
		match value {
			'w' | 'W' => Self::WHITE,
			'b' | 'B' => Self::BLACK,
			_ => panic!("Invalid color: {}", value),
		}
	}
}

impl GetColor<bool> for Color {
	fn get_color(value: bool) -> Self {
		match value {
			true => Self::WHITE,
			false => Self::BLACK,
		}
	}
}

pub trait ColorString {
	fn color_string(&self) -> String;
}

impl ColorString for Color {
	fn color_string(&self) -> String {
		String::from(match *self {
			Self::WHITE => "w",
			Self::BLACK => "b",
			Self::BOTH => "Both",
			_ => panic!("Invalid color: {}", self),
		})
	}
}
