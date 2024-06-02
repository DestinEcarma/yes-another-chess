use crate::{
	board::{
		bitboard::BitboardLSB,
		castle_right::CastleRightSquares,
		color::Colors,
		piece::Pieces,
		square::{SquareString, Squares},
		CastleRight, Color, Piece, Square,
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

		board.en_passant = None;
		board.halfmove_clock += 1;

		let color = board.color;
		let opponent = color ^ 1;

		let piece = m.piece();
		let from = m.from();
		let to = m.to();
		let captured = m.captured();
		let promoted = m.promoted();

		if captured != Piece::NONE {
			board.halfmove_clock = 0;
			board.remove_piece(captured, opponent, to);

			if captured == Piece::ROOK {
				board.castle_rights &= !CastleRight::SQUARES[to];
			}
		}

		if piece == Piece::PAWN {
			board.halfmove_clock = 0;

			board.remove_piece(piece, color, from);
			board.add_piece(
				match promoted {
					Piece::NONE => piece,
					_ => promoted,
				},
				color,
				to,
			);

			if m.en_passant() {
				board.remove_piece(Piece::PAWN, opponent, to ^ 8);
			}

			if m.two_step() {
				board.en_passant = Some(to ^ 8);
			}
		} else {
			board.remove_piece(piece, color, from);
			board.add_piece(piece, color, to);

			if piece == Piece::KING || piece == Piece::ROOK {
				board.castle_rights &= !CastleRight::SQUARES[from];
			}

			if m.castling() {
				match to {
					Square::G1 => {
						board.remove_piece(Piece::ROOK, color, Square::H1);
						board.add_piece(Piece::ROOK, color, Square::F1);
					}
					Square::C1 => {
						board.remove_piece(Piece::ROOK, color, Square::A1);
						board.add_piece(Piece::ROOK, color, Square::D1);
					}
					Square::G8 => {
						board.remove_piece(Piece::ROOK, color, Square::H8);
						board.add_piece(Piece::ROOK, color, Square::F8);
					}
					Square::C8 => {
						board.remove_piece(Piece::ROOK, color, Square::A8);
						board.add_piece(Piece::ROOK, color, Square::D8);
					}
					_ => panic!("Invalid castling move: {}", to.square_string()),
				}
			}
		}

		board.color = opponent;

		if color == Color::BLACK {
			board.fullmove_number += 1;
		}

		let legal =
			!self
				.move_gen
				.square_attacked(board, opponent, board.pieces[color][Piece::KING].lsb());

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

			if m.promoted() == Piece::NONE {
				board.remove_piece(piece, color, to);
				board.add_piece(piece, color, from);

				if m.castling() {
					match to {
						Square::G1 => {
							board.remove_piece(Piece::ROOK, color, Square::F1);
							board.add_piece(Piece::ROOK, color, Square::H1);
						}
						Square::C1 => {
							board.remove_piece(Piece::ROOK, color, Square::D1);
							board.add_piece(Piece::ROOK, color, Square::A1);
						}
						Square::G8 => {
							board.remove_piece(Piece::ROOK, color, Square::F8);
							board.add_piece(Piece::ROOK, color, Square::H8);
						}
						Square::C8 => {
							board.remove_piece(Piece::ROOK, color, Square::D8);
							board.add_piece(Piece::ROOK, color, Square::A8);
						}
						_ => panic!("Invalid castling move: {}", to.square_string()),
					}
				}
			} else {
				board.remove_piece(promoted, color, to);
				board.add_piece(Piece::PAWN, color, from);
			}

			if captured != Piece::NONE {
				board.add_piece(captured, color ^ 1, to);
			}

			if m.en_passant() {
				board.add_piece(Piece::PAWN, color ^ 1, to ^ 8);
			}
		}
	}
}
