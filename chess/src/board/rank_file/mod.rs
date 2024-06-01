mod file;
mod rank;

use super::prelude::*;
use std::{fmt, ops};

pub use file::File;
pub use rank::Rank;

#[derive(Debug)]
pub struct RankFile(pub Rank, pub File);

impl From<RankFile> for usize {
	fn from(value: RankFile) -> Self {
		(value.0) * 8 + value.1
	}
}

impl From<(u8, u8)> for RankFile {
	fn from(value: (u8, u8)) -> Self {
		Self(Rank::from(value.0), File::from(value.1))
	}
}

impl From<(char, char)> for RankFile {
	fn from(value: (char, char)) -> Self {
		Self(Rank::from(value.1), File::from(value.0))
	}
}

impl fmt::Display for RankFile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.0, self.1)
	}
}
