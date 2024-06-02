use super::{
	file_rank::FileRankConsts,
	square::{GetSquare, SquareConsts},
	File, Rank, Square,
};

pub type Bitboard = u64;

pub trait BitboardFiles {
	#[rustfmt::skip]
	const FILES: [Bitboard; usize::FILE_RANK_SIZE] = [
		0x0101010101010101, 0x0202020202020202, 0x0404040404040404, 0x0808080808080808,
		0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080,
	];
}

pub trait BitboardRanks {
	#[rustfmt::skip]
	const RANKS: [Bitboard; usize::FILE_RANK_SIZE] = [
		0x00000000000000FF, 0x000000000000FF00, 0x0000000000FF0000, 0x00000000FF000000,
		0x000000FF00000000, 0x0000FF0000000000, 0x00FF000000000000, 0xFF00000000000000,
	];
}

pub trait BitboardSquares {
	#[rustfmt::skip]
	const SQUARES: [Bitboard; usize::SQUARE_SIZE] = [
		0x0000000000000001, 0x0000000000000002, 0x0000000000000004, 0x0000000000000008,
		0x0000000000000010, 0x0000000000000020, 0x0000000000000040, 0x0000000000000080,
		0x0000000000000100, 0x0000000000000200, 0x0000000000000400, 0x0000000000000800,
		0x0000000000001000, 0x0000000000002000, 0x0000000000004000, 0x0000000000008000,
		0x0000000000010000, 0x0000000000020000, 0x0000000000040000, 0x0000000000080000,
		0x0000000000100000, 0x0000000000200000, 0x0000000000400000, 0x0000000000800000,
		0x0000000001000000, 0x0000000002000000, 0x0000000004000000, 0x0000000008000000,
		0x0000000010000000, 0x0000000020000000, 0x0000000040000000, 0x0000000080000000,
		0x0000000100000000, 0x0000000200000000, 0x0000000400000000, 0x0000000800000000,
		0x0000001000000000, 0x0000002000000000, 0x0000004000000000, 0x0000008000000000,
		0x0000010000000000, 0x0000020000000000, 0x0000040000000000, 0x0000080000000000,
		0x0000100000000000, 0x0000200000000000, 0x0000400000000000, 0x0000800000000000,
		0x0001000000000000, 0x0002000000000000, 0x0004000000000000, 0x0008000000000000,
		0x0010000000000000, 0x0020000000000000, 0x0040000000000000, 0x0080000000000000,
		0x0100000000000000, 0x0200000000000000, 0x0400000000000000, 0x0800000000000000,
		0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000,
	];
}

impl BitboardFiles for Bitboard {}
impl BitboardRanks for Bitboard {}
impl BitboardSquares for Bitboard {}

pub trait BitboardLSB {
	fn pop_lsb(&mut self) -> Square;
	fn lsb(&self) -> Square;
}

impl BitboardLSB for Bitboard {
	#[inline(always)]
	fn pop_lsb(&mut self) -> Square {
		let lsb = self.lsb();
		*self &= *self - 1;
		lsb
	}

	#[inline(always)]
	fn lsb(&self) -> Square {
		self.trailing_zeros() as Square
	}
}

pub trait BitboardOccupied<T: std::ops::BitAnd> {
	fn occupied(&self, square: T) -> bool;
}

impl BitboardOccupied<Square> for Bitboard {
	#[inline(always)]
	fn occupied(&self, square: Square) -> bool {
		self & Self::SQUARES[square] > 0
	}
}

pub trait BitboardString {
	fn bitboard_string(&self) -> String;
}

impl BitboardString for Bitboard {
	fn bitboard_string(&self) -> String {
		let mut string = String::new();

		for rank in Rank::FILE_RANK_RANGE.rev() {
			for file in File::FILE_RANK_RANGE {
				let square = usize::get_square((file, rank));

				string.push_str(match self.occupied(square) {
					true => "1 ",
					false => "0 ",
				});
			}

			string.push('\n');
		}

		string
	}
}
