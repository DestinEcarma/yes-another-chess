mod impls;
mod prelude;

pub mod bitboard;
pub mod castle_right;
pub mod color;
pub mod file_rank;
pub mod piece;
pub mod pieces;
pub mod square;

use bitboard::BitboardSquares;
use color::ColorConsts;
use piece::Pieces;

pub use prelude::*;

#[derive(Debug)]
pub struct Board {
	pub pieces: BitboardPieces,
	pub color: Color,
	pub en_passant: Option<Square>,
	pub castle_rights: CastleRight,

	pub piece_list: PieceList,

	pub occupancy: Bitboard,
	pub occupancy_color: [Bitboard; usize::COLOR_SIZE],

	pub halfmove_clock: u8,
	pub fullmove_number: u16,
}

impl Default for Board {
	fn default() -> Self {
		Self::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	}
}

impl Board {
	#[inline(always)]
	pub fn add_piece(&mut self, piece: Piece, color: Color, square: Square) {
		self.piece_list[square] = piece;
		self.pieces[color][piece] |= Bitboard::SQUARES[square];

		self.occupancy |= Bitboard::SQUARES[square];
		self.occupancy_color[color] |= Bitboard::SQUARES[square];
	}

	#[inline(always)]
	pub fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
		self.piece_list[square] = Piece::NONE;
		self.pieces[color][piece] &= !(Bitboard::SQUARES[square]);

		self.occupancy &= !(Bitboard::SQUARES[square]);
		self.occupancy_color[color] &= !(Bitboard::SQUARES[square]);
	}
}

impl Board {
	#[inline(always)]
	pub fn ally(&self) -> Bitboard {
		self.occupancy_color[self.color]
	}

	#[inline(always)]
	pub fn enemy(&self) -> Bitboard {
		self.occupancy_color[self.color ^ 1]
	}
}
