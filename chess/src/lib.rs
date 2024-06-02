mod history;
mod move_list;
mod perft;
mod playmove;

use move_list::MoveList;
use std::fmt;

pub mod board;
pub mod move_gen;

#[derive(Debug, Default)]
pub struct Chess {
	pub board: board::Board,
	move_gen: move_gen::MoveGen,
	history: history::History,
}

impl From<&str> for Chess {
	fn from(fen: &str) -> Self {
		let board = board::Board::from(fen);
		let move_gen = move_gen::MoveGen::default();
		let history = history::History::default();

		Self {
			board,
			move_gen,
			history,
		}
	}
}

impl fmt::Display for Chess {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.board)
	}
}

impl Chess {
	#[inline(always)]
	pub fn generate_moves(&self) -> MoveList {
		let mut list = MoveList::default();

		self.move_gen.king(&self.board, &mut list);
		self.move_gen.queens(&self.board, &mut list);
		self.move_gen.rooks(&self.board, &mut list);
		self.move_gen.bishops(&self.board, &mut list);
		self.move_gen.knights(&self.board, &mut list);
		self.move_gen.pawns(&self.board, &mut list);
		self.move_gen.castling(&self.board, &mut list);

		list
	}
}
