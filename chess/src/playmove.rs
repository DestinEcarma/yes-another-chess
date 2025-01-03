use crate::{
	board::{
		bitboard::BitboardUtils, castle_right::CastleRightUtils, color::ColorUtils,
		piece::PieceUtils, square::SquareUtils,
	},
	history::OldState,
	move_gen::Move,
	Chess,
};

impl Chess {
	#[inline(always)]
	pub fn play_move(&mut self, m: Move) -> bool {
		let board = &mut self.board;

		self.history.push(OldState::new(board, m));

		board.halfmove_clock += 1;

		let color = board.color;
		let opponent = color ^ 1;
		let has_caslte_rights = board.castle_rights != CastleRightUtils::NONE;

		let piece = m.piece();
		let from = m.from();
		let to = m.to();
		let captured = m.captured();
		let promoted = m.promoted();

		if board.en_passant.is_some() {
			board.clear_en_passant();
		}

		if captured != PieceUtils::NONE {
			board.halfmove_clock = 0;
			board.remove_piece(captured, opponent, to);

			if captured == PieceUtils::ROOK && has_caslte_rights {
				board.update_castle_rights(board.castle_rights & !CastleRightUtils::SQUARES[to]);
			}
		}

		if piece == PieceUtils::PAWN {
			board.halfmove_clock = 0;

			board.remove_piece(piece, color, from);
			board.add_piece(
				match promoted {
					PieceUtils::NONE => piece,
					_ => promoted,
				},
				color,
				to,
			);

			if m.en_passant() {
				board.remove_piece(PieceUtils::PAWN, opponent, to ^ 8);
			}

			if m.two_step() {
				board.set_en_passant(to ^ 8);
			}
		} else {
			board.remove_piece(piece, color, from);
			board.add_piece(piece, color, to);

			if (piece == PieceUtils::KING || piece == PieceUtils::ROOK) && has_caslte_rights {
				board.update_castle_rights(board.castle_rights & !CastleRightUtils::SQUARES[from]);
			}

			if m.castling() {
				match to {
					SquareUtils::G1 => {
						board.remove_piece(PieceUtils::ROOK, color, SquareUtils::H1);
						board.add_piece(PieceUtils::ROOK, color, SquareUtils::F1);
					}
					SquareUtils::C1 => {
						board.remove_piece(PieceUtils::ROOK, color, SquareUtils::A1);
						board.add_piece(PieceUtils::ROOK, color, SquareUtils::D1);
					}
					SquareUtils::G8 => {
						board.remove_piece(PieceUtils::ROOK, color, SquareUtils::H8);
						board.add_piece(PieceUtils::ROOK, color, SquareUtils::F8);
					}
					SquareUtils::C8 => {
						board.remove_piece(PieceUtils::ROOK, color, SquareUtils::A8);
						board.add_piece(PieceUtils::ROOK, color, SquareUtils::D8);
					}
					_ => panic!("Invalid castling move: {}", SquareUtils::to_string(to)),
				}
			}
		}

		board.switch_color();

		if color == ColorUtils::BLACK {
			board.fullmove_number += 1;
		}

		let legal = !self.move_gen.square_attacked(
			board,
			opponent,
			BitboardUtils::lsb(board.pieces[color][PieceUtils::KING]),
		);

		#[cfg(debug_assertions)]
		debug_assert!(debug::check_incrementals(board));

		if !legal {
			self.undo_move();
		}

		legal
	}
}

impl Chess {
	#[inline(always)]
	pub fn undo_move(&mut self) {
		let board = &mut self.board;

		if let Some(state) = self.history.pop() {
			board.en_passant = state.en_passant;
			board.castle_rights = state.castle_rights;
			board.halfmove_clock = state.halfmove_clock;
			board.fullmove_number = state.fullmove_number;
			board.color = state.color;

			let color = board.color;

			let m = state.move_made;
			let piece = m.piece();
			let from = m.from();
			let to = m.to();
			let captured = m.captured();
			let promoted = m.promoted();

			if m.promoted() == PieceUtils::NONE {
				board.remove_piece(piece, color, to);
				board.add_piece(piece, color, from);

				if m.castling() {
					match to {
						SquareUtils::G1 => {
							board.remove_piece(PieceUtils::ROOK, color, SquareUtils::F1);
							board.add_piece(PieceUtils::ROOK, color, SquareUtils::H1);
						}
						SquareUtils::C1 => {
							board.remove_piece(PieceUtils::ROOK, color, SquareUtils::D1);
							board.add_piece(PieceUtils::ROOK, color, SquareUtils::A1);
						}
						SquareUtils::G8 => {
							board.remove_piece(PieceUtils::ROOK, color, SquareUtils::F8);
							board.add_piece(PieceUtils::ROOK, color, SquareUtils::H8);
						}
						SquareUtils::C8 => {
							board.remove_piece(PieceUtils::ROOK, color, SquareUtils::D8);
							board.add_piece(PieceUtils::ROOK, color, SquareUtils::A8);
						}
						_ => panic!("Invalid castling move: {}", SquareUtils::to_string(to)),
					}
				}
			} else {
				board.remove_piece(promoted, color, to);
				board.add_piece(PieceUtils::PAWN, color, from);
			}

			if captured != PieceUtils::NONE {
				board.add_piece(captured, color ^ 1, to);
			}

			if m.en_passant() {
				board.add_piece(PieceUtils::PAWN, color ^ 1, to ^ 8);
			}

			board.hash = state.hash;
		}
	}
}

#[cfg(debug_assertions)]
mod debug {
	use crate::board;

	pub fn check_incrementals(board: &board::Board) -> bool {
		let from_scratch_key = board.init_hash();
		let mut result = true;

		if result && from_scratch_key != board.hash {
			println!("Check Incrementals: Error in Zobrist key.");
			result = false;
		};

		result
	}
}
