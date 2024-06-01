use super::{File, Rank, RankFile, Square};
use std::{fmt, num::Wrapping, ops};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bitboard(pub u64);

impl PartialEq<u64> for Bitboard {
	fn eq(&self, other: &u64) -> bool {
		self.0 == *other
	}
}

impl PartialOrd<u64> for Bitboard {
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(other)
	}
}

impl ops::Not for Bitboard {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self(!self.0)
	}
}

impl ops::BitOr<Bitboard> for Bitboard {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

impl ops::BitOr<Square> for Bitboard {
	type Output = Self;

	fn bitor(self, rhs: Square) -> Self::Output {
		Self(self.0 | (1 << rhs))
	}
}

impl ops::BitAnd<Bitboard> for Bitboard {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}

impl ops::BitAnd<Rank> for Bitboard {
	type Output = Self;

	fn bitand(self, rhs: Rank) -> Self::Output {
		self & Self::from(rhs)
	}
}

impl ops::BitAnd<File> for Bitboard {
	type Output = Self;

	fn bitand(self, rhs: File) -> Self::Output {
		self & Self::from(rhs)
	}
}

impl ops::BitAnd<u64> for &Bitboard {
	type Output = u64;

	fn bitand(self, rhs: u64) -> Self::Output {
		self.0 & rhs
	}
}

impl ops::BitAnd<Square> for Bitboard {
	type Output = Self;

	fn bitand(self, rhs: Square) -> Self::Output {
		Bitboard(self.0 & (1 << rhs))
	}
}

impl ops::BitAnd<Square> for &Bitboard {
	type Output = Bitboard;

	fn bitand(self, rhs: Square) -> Self::Output {
		Bitboard(self.0 & (1 << rhs))
	}
}

impl ops::BitOrAssign<Bitboard> for Bitboard {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

impl ops::BitOrAssign<Square> for Bitboard {
	fn bitor_assign(&mut self, rhs: Square) {
		self.0 |= 1 << rhs;
	}
}

impl ops::BitXorAssign<Square> for Bitboard {
	fn bitxor_assign(&mut self, rhs: Square) {
		self.0 ^= 1 << rhs;
	}
}

impl ops::BitAndAssign<Bitboard> for Bitboard {
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

impl ops::Shl<u8> for Bitboard {
	type Output = Self;

	fn shl(self, rhs: u8) -> Self::Output {
		Self(self.0 << rhs)
	}
}

impl ops::Shl<&Square> for Bitboard {
	type Output = Self;

	fn shl(self, rhs: &Square) -> Self::Output {
		Self(self.0 << *rhs)
	}
}

impl ops::Shl<Square> for Bitboard {
	type Output = Self;

	fn shl(self, rhs: Square) -> Self::Output {
		Self(self.0 << rhs)
	}
}

impl ops::Shl<Rank> for Bitboard {
	type Output = Self;

	fn shl(self, rhs: Rank) -> Self::Output {
		Self(self.0 << (rhs * 8))
	}
}

impl ops::Shl<File> for Bitboard {
	type Output = Self;

	fn shl(self, rhs: File) -> Self::Output {
		Self(self.0 << rhs as u64)
	}
}

impl ops::Shr<u8> for Bitboard {
	type Output = Self;

	fn shr(self, rhs: u8) -> Self::Output {
		Self(self.0 >> rhs)
	}
}

impl From<Rank> for Bitboard {
	fn from(rank: Rank) -> Self {
		Self(0xFF << rank)
	}
}

impl From<File> for Bitboard {
	fn from(file: File) -> Self {
		Self(0x0101010101010101 << file)
	}
}

impl From<Square> for Bitboard {
	fn from(square: Square) -> Self {
		Self(1 << square)
	}
}

impl fmt::Display for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut bitboard = String::new();

		for rank in Rank::Eighth.iter().rev() {
			for file in File::A.iter() {
				let square = Square::from(RankFile(rank, file));

				bitboard += match self.occupied(square) {
					true => "1 ",
					false => "0 ",
				};
			}

			bitboard += "\n";
		}

		write!(f, "{bitboard}")
	}
}

impl fmt::LowerHex for Bitboard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#018x}", self.0)
	}
}

impl Bitboard {
	#[inline(always)]
	pub fn occupied(&self, square: Square) -> bool {
		(self & square) > 0
	}

	#[inline(always)]
	pub fn wrapping_mul(self, rhs: u64) -> u64 {
		self.0.wrapping_mul(rhs)
	}

	#[inline(always)]
	pub fn wrapping_sub(self, rhs: Self) -> Self {
		Self(self.0.wrapping_sub(rhs.0))
	}

	#[inline(always)]
	pub fn count_ones(self) -> u32 {
		self.0.count_ones()
	}

	#[inline(always)]
	pub fn rotate_left(&self, count: u32) -> Self {
		Self(self.0.rotate_left(count))
	}

	#[inline(always)]
	pub fn lsb(&mut self) -> Square {
		let square = self.get_lsb();
		*self ^= square;
		square
	}

	#[inline(always)]
	pub fn get_lsb(&self) -> Square {
		let square = self.0.trailing_zeros() as usize;

		Square::from(square % 64)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ranks() {
		assert_eq!(Bitboard::from(Rank::First), Bitboard(0xFF));
		assert_eq!(Bitboard::from(Rank::Second), Bitboard(0xFF00));
		assert_eq!(Bitboard::from(Rank::Third), Bitboard(0xFF0000));
		assert_eq!(Bitboard::from(Rank::Fourth), Bitboard(0xFF000000));
		assert_eq!(Bitboard::from(Rank::Fifth), Bitboard(0xFF00000000));
		assert_eq!(Bitboard::from(Rank::Sixth), Bitboard(0xFF0000000000));
		assert_eq!(Bitboard::from(Rank::Seventh), Bitboard(0xFF000000000000));
		assert_eq!(Bitboard::from(Rank::Eighth), Bitboard(0xFF00000000000000));
	}

	#[test]
	fn files() {
		assert_eq!(Bitboard::from(File::A), Bitboard(0x0101010101010101));
		assert_eq!(Bitboard::from(File::B), Bitboard(0x0202020202020202));
		assert_eq!(Bitboard::from(File::C), Bitboard(0x0404040404040404));
		assert_eq!(Bitboard::from(File::D), Bitboard(0x0808080808080808));
		assert_eq!(Bitboard::from(File::E), Bitboard(0x1010101010101010));
		assert_eq!(Bitboard::from(File::F), Bitboard(0x2020202020202020));
		assert_eq!(Bitboard::from(File::G), Bitboard(0x4040404040404040));
		assert_eq!(Bitboard::from(File::H), Bitboard(0x8080808080808080));
	}

	#[test]
	fn square() {
		assert_eq!(Bitboard::from(Square::A1), Bitboard(0x01));
		assert_eq!(Bitboard::from(Square::H8), Bitboard(0x8000000000000000));
	}
}
