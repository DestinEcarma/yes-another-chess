use castle_right::CastleRightUtils;
use file_rank::{FileUtils, RankUtils};
use piece::PieceUtils;
use square::SquareUtils;

use super::*;
use std::fmt;

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n{}", self.board_string(), self.fen_string())
	}
}

impl Board {
	fn get_piece(&self, square: Square) -> Option<(Piece, Color)> {
		let piece = self.piece_list[square];

		if piece == PieceUtils::NONE {
			None
		} else {
			for color in ColorUtils::RANGE {
				if BitboardUtils::occupied(self.pieces[color][piece], square) {
					return Some((piece, color));
				}
			}

			panic!("Piece not found: {:?}", piece);
		}
	}

	fn board_string(&self) -> String {
		let mut board = String::from("   +---+---+---+---+---+---+---+---+\n");

		for rank in RankUtils::RANGE.rev() {
			board += &format!(" {rank} ");

			for file in FileUtils::RANGE {
				let piece = match self.get_piece(SquareUtils::from_location(file, rank)) {
					Some((piece, color)) => PieceUtils::to_string(piece, color),
					None => " ".to_string(),
				};

				board += &format!("| {piece} ")
			}

			board += "|\n   +---+---+---+---+---+---+---+---+\n";
		}

		board + "     a   b   c   d   e   f   g   h\n"
	}

	fn fen_string(&self) -> String {
		let mut pieces = String::new();

		for rank in RankUtils::RANGE.rev() {
			let mut empty = 0;

			for file in FileUtils::RANGE {
				let square = SquareUtils::from_location(file, rank);

				match self.get_piece(square) {
					Some((piece, color)) => {
						if empty > 0 {
							pieces.push_str(&empty.to_string());
							empty = 0;
						}

						pieces.push_str(&PieceUtils::to_string(piece, color));
					}
					None => empty += 1,
				}
			}

			if empty > 0 {
				pieces.push_str(&empty.to_string());
			}

			if rank > RankUtils::R1 {
				pieces += "/";
			}
		}

		let color = ColorUtils::to_string(self.color);
		let castle_rights = CastleRightUtils::to_string(self.castle_rights);

		let en_passant = match self.en_passant {
			Some(square) => SquareUtils::to_string(square),
			None => "-".to_string(),
		};

		let halfmove_clock = self.halfmove_clock;
		let fullmove_number = self.fullmove_number;

		format!("{pieces} {color} {castle_rights} {en_passant} {halfmove_clock} {fullmove_number}")
	}
}
