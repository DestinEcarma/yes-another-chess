use super::{
	castle_right::CastleRightConsts, color::ColorConsts, piece::PieceConsts, square::SquareUtils,
	CastleRight, Color, Piece, Square,
};

use rand::Rng;

type PieceTable = [[[ZobristHash; SquareUtils::SIZE]; Piece::PIECE_SIZE]; Color::COLOR_SIZE];
type ColorTable = [ZobristHash; Color::COLOR_SIZE];
type CastleTable = [ZobristHash; CastleRight::CASTLE_RIGHT_SIZE];
type EnPassantTable = [ZobristHash; SquareUtils::SIZE];

pub(crate) type ZobristHash = u64;

#[derive(Debug)]
pub(super) struct HashTable {
	pieces: PieceTable,
	colors: ColorTable,
	castles: CastleTable,
	en_passant: EnPassantTable,
}

impl Default for HashTable {
	fn default() -> Self {
		let mut random = rand::thread_rng();

		let mut hash_table = Self {
			pieces: [[[0; SquareUtils::SIZE]; Piece::PIECE_SIZE]; Color::COLOR_SIZE],
			colors: [0; Color::COLOR_SIZE],
			castles: [0; CastleRight::CASTLE_RIGHT_SIZE],
			en_passant: [0; SquareUtils::SIZE],
		};

		hash_table.pieces.iter_mut().for_each(|color| {
			color.iter_mut().for_each(|piece| {
				piece.iter_mut().for_each(|square| *square = random.gen());
			})
		});

		hash_table
			.colors
			.iter_mut()
			.for_each(|color| *color = random.gen());

		hash_table
			.castles
			.iter_mut()
			.for_each(|castle| *castle = random.gen());

		hash_table
			.en_passant
			.iter_mut()
			.for_each(|en_passant| *en_passant = random.gen());

		hash_table
	}
}

impl HashTable {
	#[inline(always)]
	pub(super) fn piece(&self, piece: Piece, color: Color, square: Square) -> ZobristHash {
		self.pieces[color][piece][square]
	}

	#[inline(always)]
	pub(super) fn color(&self, color: Color) -> ZobristHash {
		self.colors[color]
	}

	#[inline(always)]
	pub(super) fn castle(&self, castle: CastleRight) -> ZobristHash {
		self.castles[castle.trailing_zeros() as usize]
	}

	#[inline(always)]
	pub(super) fn en_passant(&self, square: Square) -> ZobristHash {
		self.en_passant[square]
	}
}
