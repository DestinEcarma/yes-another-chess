use super::*;
use std::fmt;

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n{}", self.board_string(), self.fen_string())
	}
}

impl Board {
	fn get_piece(&self, square: Square) -> Option<Piece> {
		for (piece, bitboard) in self.pieces.0.iter().enumerate() {
			if bitboard & (1 << square) != 0 {
				return Some(Piece::from(piece as u8));
			}
		}

		None
	}

	fn board_string(&self) -> String {
		let mut board = String::from("   +---+---+---+---+---+---+---+---+\n");

		for rank in Rank::Eighth.iter().rev() {
			board += &format!(" {rank} ");

			for file in File::A.iter() {
				let piece = match self.get_piece(Square::from(RankFile(rank, file))) {
					Some(piece) => piece.to_string(),
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
		let mut color = String::new();
		let mut castle_rights = String::new();
		let mut en_passant = String::new();

		for rank in Rank::Eighth.iter().rev() {
			let mut empty = 0;

			for file in File::A.iter() {
				let square = Square::from(RankFile(rank, file));

				match self.get_piece(square) {
					Some(piece) => {
						if empty > 0 {
							pieces += &empty.to_string();
							empty = 0;
						}

						pieces += &piece.to_string();
					}
					None => empty += 1,
				}
			}

			if empty > 0 {
				pieces += &empty.to_string();
			}

			if rank > Rank::First {
				pieces += "/";
			}
		}

		color += &self.color.to_string();
		castle_rights += &self.castle_rights.to_string();

		if let Some(square) = self.en_passant {
			en_passant += &square.to_string();
		} else {
			en_passant += "-";
		}

		let halfmove_clock = self.halfmove_clock;
		let fullmove_number = self.fullmove_number;

		format!("{pieces} {color} {castle_rights} {en_passant} {halfmove_clock} {fullmove_number}")
	}
}
