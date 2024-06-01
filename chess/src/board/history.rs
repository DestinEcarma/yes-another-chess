use super::{CastleRights, Color, Square};
use crate::move_gen::Move;

pub type History = Vec<OldState>;

#[derive(Debug)]
pub struct OldState {
	pub color: Color,
	pub en_passant: Option<Square>,
	pub castle_rights: CastleRights,
	pub halfmove_clock: u8,
	pub fullmove_number: u16,
	pub move_made: Move,
}
