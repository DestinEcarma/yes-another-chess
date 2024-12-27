use super::{color::ColorConsts, piece::PieceConsts, square::SquareConsts, Color, Piece, Square};

use rand::Rng;

pub(super) type ZobristHash = u64;

type PieceTable = [[[ZobristHash; Square::SQUARE_SIZE]; Piece::PIECE_SIZE]; Color::COLOR_SIZE];

#[derive(Debug)]
pub(super) struct HashTable(PieceTable);

impl Default for HashTable {
	fn default() -> Self {
		let mut random = rand::thread_rng();

		let mut pieces = [[[0; Square::SQUARE_SIZE]; Piece::PIECE_SIZE]; Color::COLOR_SIZE];

		pieces.iter_mut().for_each(|color| {
			color.iter_mut().for_each(|piece| {
				piece.iter_mut().for_each(|square| *square = random.gen());
			})
		});

		Self(pieces)
	}
}

impl HashTable {
	#[inline(always)]
	pub(super) fn piece(&self, piece: Piece, color: Color, square: Square) -> ZobristHash {
		self.0[color][piece][square]
	}
}
