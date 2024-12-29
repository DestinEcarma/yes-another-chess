use crate::{
	board::{zobrist::ZobristHash, Board, CastleRight, Color, Square},
	move_gen::Move,
};

pub type History = Vec<OldState>;

#[derive(Debug, Clone)]
pub struct OldState {
	pub color: Color,
	pub en_passant: Option<Square>,
	pub castle_rights: CastleRight,
	pub halfmove_clock: u8,
	pub fullmove_number: u16,
	pub move_made: Move,
	pub hash: ZobristHash,
}

impl OldState {
	#[inline(always)]
	pub fn new(board: &Board, move_made: Move) -> Self {
		Self {
			hash: board.hash,
			color: board.color,
			en_passant: board.en_passant,
			castle_rights: board.castle_rights,
			halfmove_clock: board.halfmove_clock,
			fullmove_number: board.fullmove_number,
			move_made,
		}
	}
}
