mod impls;
mod prelude;

pub mod bitboard;
pub mod castle_right;
pub mod color;
pub mod file_rank;
pub mod piece;
pub mod pieces;
pub mod square;
pub mod zobrist;

use bitboard::BitboardUtils;
use color::ColorConsts;
use piece::Pieces;
use std::sync::Arc;
use zobrist::{HashTable, ZobristHash};

pub use prelude::*;

#[derive(Debug, Clone)]
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

	pub hash: ZobristHash,

	hash_table: Arc<HashTable>,
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
		self.pieces[color][piece] |= BitboardUtils::SQUARES[square];

		self.occupancy |= BitboardUtils::SQUARES[square];
		self.occupancy_color[color] |= BitboardUtils::SQUARES[square];

		self.hash ^= self.hash_table.piece(piece, color, square);
	}

	#[inline(always)]
	pub fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
		self.piece_list[square] = Piece::NONE;
		self.pieces[color][piece] &= !(BitboardUtils::SQUARES[square]);

		self.occupancy &= !(BitboardUtils::SQUARES[square]);
		self.occupancy_color[color] &= !(BitboardUtils::SQUARES[square]);

		self.hash ^= self.hash_table.piece(piece, color, square);
	}

	#[inline(always)]
	pub fn switch_color(&mut self) {
		self.hash ^= self.hash_table.color(self.color);
		self.color ^= 1;
		self.hash ^= self.hash_table.color(self.color);
	}

	#[inline(always)]
	pub fn update_castle_rights(&mut self, castle_rights: CastleRight) {
		self.hash ^= self.hash_table.castle(self.castle_rights);
		self.castle_rights = castle_rights;
		self.hash ^= self.hash_table.castle(self.castle_rights);
	}

	#[inline(always)]
	pub fn set_en_passant(&mut self, square: Square) {
		self.hash ^= self.hash_table.en_passant(self.en_passant);
		self.en_passant = Some(square);
		self.hash ^= self.hash_table.en_passant(self.en_passant);
	}

	#[inline(always)]
	pub fn clear_en_passant(&mut self) {
		self.hash ^= self.hash_table.en_passant(self.en_passant);
		self.en_passant = None;
		self.hash ^= self.hash_table.en_passant(self.en_passant);
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
