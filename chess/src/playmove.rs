use crate::{
	board::{CastleRights, Color, Error, OldState, Piece, Square},
	move_gen::{Move, MoveGen},
	Chess,
};

impl Chess {
	#[inline(always)]
	pub fn play_move(&mut self, m: Move) -> bool {
		let board = &mut self.board;

		board.history.push(OldState {
			color: board.color,
			en_passant: board.en_passant,
			castle_rights: board.castle_rights,
			halfmove_clock: board.halfmove_clock,
			fullmove_number: board.fullmove_number,
			move_made: m,
		});

		board.en_passant = None;
		board.halfmove_clock += 1;

		let color = board.color;
		let opponent = !color;

		let piece = m.piece();
		let from = m.from();
		let to = m.to();

		if let Some(piece) = m.captured() {
			board.halfmove_clock = 0;
			board.remove_piece(piece, to);

			if piece == Piece::Rook(opponent) {
				board.castle_rights &= !CastleRights::square(to);
			}
		}

		if piece != Piece::Pawn(color) {
			board.remove_piece(piece, from);
			board.add_piece(piece, to);

			if piece == Piece::King(color) || piece == Piece::Rook(color) {
				board.castle_rights &= !CastleRights::square(from);
			}

			if m.castling() {
				match to {
					Square::G1 => {
						board.remove_piece(Piece::Rook(color), Square::H1);
						board.add_piece(Piece::Rook(color), Square::F1);
					}
					Square::C1 => {
						board.remove_piece(Piece::Rook(color), Square::A1);
						board.add_piece(Piece::Rook(color), Square::D1);
					}
					Square::G8 => {
						board.remove_piece(Piece::Rook(color), Square::H8);
						board.add_piece(Piece::Rook(color), Square::F8);
					}
					Square::C8 => {
						board.remove_piece(Piece::Rook(color), Square::A8);
						board.add_piece(Piece::Rook(color), Square::D8);
					}
					_ => panic!("{}", Error::InvalidCastlingMove(to)),
				}
			}
		} else {
			board.halfmove_clock = 0;

			board.remove_piece(piece, from);
			board.add_piece(
				match m.promoted() {
					Some(promoted) => promoted,
					None => piece,
				},
				to,
			);

			if m.en_passant() {
				board.remove_piece(Piece::Pawn(opponent), to ^ 8);
			}

			if m.two_step() {
				board.en_passant = Some(to ^ 8);
			}
		}

		board.color = opponent;

		if color == Color::Black {
			board.fullmove_number += 1;
		}

		board.update_occupancies();

		let legal = !self.move_gen.square_attacked(
			board,
			opponent,
			board.pieces[Piece::King(color)].get_lsb(),
		);

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

		if let Some(state) = board.history.pop() {
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

			if m.promoted().is_none() {
				board.remove_piece(piece, to);
				board.add_piece(piece, from);

				if m.castling() {
					match to {
						Square::G1 => {
							board.remove_piece(Piece::Rook(color), Square::F1);
							board.add_piece(Piece::Rook(color), Square::H1);
						}
						Square::C1 => {
							board.remove_piece(Piece::Rook(color), Square::D1);
							board.add_piece(Piece::Rook(color), Square::A1);
						}
						Square::G8 => {
							board.remove_piece(Piece::Rook(color), Square::F8);
							board.add_piece(Piece::Rook(color), Square::H8);
						}
						Square::C8 => {
							board.remove_piece(Piece::Rook(color), Square::D8);
							board.add_piece(Piece::Rook(color), Square::A8);
						}
						_ => panic!("{}", Error::InvalidCastlingMove(to)),
					}
				}
			} else {
				board.remove_piece(m.promoted().unwrap(), to);
				board.add_piece(Piece::Pawn(color), from);
			}

			if let Some(piece) = m.captured() {
				board.add_piece(piece, to);
			}

			if m.en_passant() {
				board.add_piece(Piece::Pawn(!color), to ^ 8);
			}

			board.update_occupancies();
		}
	}
}
