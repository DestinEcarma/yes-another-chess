mod bitboard;
mod castle_rights;
mod color;
mod error;
mod history;
mod impls;
mod piece;
mod piece_list;
mod pieces;
mod prelude;
mod rank_file;
mod square;

pub use prelude::*;

#[derive(Debug)]
pub struct Board {
	pub pieces: Pieces,
	pub color: Color,
	pub en_passant: Option<Square>,
	pub castle_rights: CastleRights,

	pub piece_list: PieceList,
	pub occupancy: Bitboard,
	pub ally: Bitboard,
	pub enemy: Bitboard,

	pub halfmove_clock: u8,
	pub fullmove_number: u16,

	pub history: History,
}

impl Default for Board {
	fn default() -> Self {
		Self::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	}
}

impl Board {
	#[inline(always)]
	pub fn add_piece(&mut self, piece: Piece, square: Square) {
		self.piece_list[square] = Some(piece);
		self.pieces[piece] |= square;
	}

	#[inline(always)]
	pub fn remove_piece(&mut self, piece: Piece, square: Square) {
		self.piece_list[square] = None;
		self.pieces[piece] &= !square;
	}
}

impl Board {
	#[inline(always)]
	pub fn update_occupancies(&mut self) {
		self.occupancy = Bitboard::default();
		self.ally = Bitboard::default();
		self.enemy = Bitboard::default();

		for piece in Piece::King(self.color).iter() {
			self.occupancy |= self.pieces[piece];
			self.ally |= self.pieces[piece];
		}

		for piece in Piece::King(!self.color).iter() {
			self.occupancy |= self.pieces[piece];
			self.enemy |= self.pieces[piece];
		}
	}
}
