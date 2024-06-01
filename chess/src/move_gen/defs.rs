use super::{piece_move::Move, Magic};
use crate::board::{Bitboard, Color, Square};
use std::ops;

pub type MoveList = Vec<Move>;

pub type PieceMagics = [Magic; Square::SIZE];
pub type PieceMoves = [Bitboard; Square::SIZE];
pub type BlockerTable = Vec<Bitboard>;
pub type AttackTable = Vec<Bitboard>;
pub type MagicTable = Vec<Bitboard>;

pub const ROOK_TABLE_SIZE: usize = 102_400;
pub const BISHOP_TABLE_SIZE: usize = 5_248;

impl ops::Index<Square> for PieceMagics {
	type Output = Magic;

	fn index(&self, index: Square) -> &Self::Output {
		&self[index as usize]
	}
}

impl ops::IndexMut<Square> for PieceMagics {
	fn index_mut(&mut self, index: Square) -> &mut Self::Output {
		&mut self[index as usize]
	}
}

impl ops::Index<Square> for PieceMoves {
	type Output = Bitboard;

	fn index(&self, index: Square) -> &Self::Output {
		&self[index as usize]
	}
}

impl ops::IndexMut<Square> for PieceMoves {
	fn index_mut(&mut self, index: Square) -> &mut Self::Output {
		&mut self[index as usize]
	}
}

impl ops::Index<Color> for [PieceMoves; Color::SIZE] {
	type Output = PieceMoves;

	fn index(&self, index: Color) -> &Self::Output {
		&self[index as usize]
	}
}

impl ops::IndexMut<Color> for [PieceMoves; Color::SIZE] {
	fn index_mut(&mut self, index: Color) -> &mut Self::Output {
		&mut self[index as usize]
	}
}
