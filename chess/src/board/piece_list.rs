use super::{Piece, Square};
use std::ops;

#[derive(Debug)]
pub struct PieceList(pub [Option<Piece>; Square::SIZE]);

impl Default for PieceList {
	fn default() -> Self {
		Self([None; Square::SIZE])
	}
}

impl ops::Index<Square> for PieceList {
	type Output = Option<Piece>;

	fn index(&self, square: Square) -> &Self::Output {
		&self.0[square as usize]
	}
}

impl ops::IndexMut<Square> for PieceList {
	fn index_mut(&mut self, square: Square) -> &mut Self::Output {
		&mut self.0[square as usize]
	}
}
