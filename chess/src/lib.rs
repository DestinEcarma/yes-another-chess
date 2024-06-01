#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod playmove;

pub mod board;
pub mod move_gen;
mod perft;

use move_gen::MoveList;
use std::fmt;

#[derive(Debug, Default)]
pub struct Chess {
	pub board: board::Board,
	move_gen: move_gen::MoveGen,
}

impl From<&str> for Chess {
	fn from(fen: &str) -> Self {
		let board = board::Board::from(fen);
		let move_gen = move_gen::MoveGen::default();

		Self { board, move_gen }
	}
}

impl fmt::Display for Chess {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.board)
	}
}

impl Chess {
	#[inline(always)]
	pub fn generate_moves(&mut self) -> MoveList {
		let mut list = MoveList::default();

		self.move_gen.king(&self.board, &mut list);
		self.move_gen.queens(&self.board, &mut list);
		self.move_gen.rooks(&self.board, &mut list);
		self.move_gen.bishops(&self.board, &mut list);
		self.move_gen.knights(&self.board, &mut list);
		self.move_gen.pawns(&self.board, &mut list);
		self.move_gen.castling(&self.board, &mut list);

		// println!("{list:?}");

		list
	}
}
