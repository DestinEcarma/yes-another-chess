use castle_right::CastleRightString;
use file_rank::Ranks;
use square::SquareString;

use super::*;
use super::{
	bitboard::BitboardOccupied,
	color::{ColorConsts, ColorString},
	file_rank::FileRankConsts,
	piece::PieceString,
	square::GetSquare,
};
use std::fmt;

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n{}", self.board_string(), self.fen_string())
	}
}

impl Board {
	fn get_piece(&self, square: Square) -> Option<(Piece, Color)> {
		let piece = self.piece_list[square];

		if piece == Piece::NONE {
			None
		} else {
			for color in Color::COLOR_RANGE {
				if self.pieces[color][piece].occupied(square) {
					return Some((piece, color));
				}
			}

			panic!("Piece not found: {:?}", piece);
		}
	}

	fn board_string(&self) -> String {
		let mut board = String::from("   +---+---+---+---+---+---+---+---+\n");

		for rank in usize::FILE_RANK_RANGE.rev() {
			board += &format!(" {rank} ");

			for file in usize::FILE_RANK_RANGE {
				let piece = match self.get_piece(Square::get_square((file, rank))) {
					Some((piece, color)) => piece.piece_string(color),
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

		for rank in usize::FILE_RANK_RANGE.rev() {
			let mut empty = 0;

			for file in usize::FILE_RANK_RANGE {
				let square = Square::get_square((file, rank));

				match self.get_piece(square) {
					Some((piece, color)) => {
						if empty > 0 {
							pieces.push_str(&empty.to_string());
							empty = 0;
						}

						pieces.push_str(&piece.piece_string(color));
					}
					None => empty += 1,
				}
			}

			if empty > 0 {
				pieces.push_str(&empty.to_string());
			}

			if rank > Rank::R1 {
				pieces += "/";
			}
		}

		let color = self.color.color_string();
		let castle_rights = &self.castle_rights.castle_right_string();
		let en_passant: String;

		if let Some(square) = self.en_passant {
			en_passant = square.square_string();
		} else {
			en_passant = String::from("-");
		}

		let halfmove_clock = self.halfmove_clock;
		let fullmove_number = self.fullmove_number;

		format!("{pieces} {color} {castle_rights} {en_passant} {halfmove_clock} {fullmove_number}")
	}
}
