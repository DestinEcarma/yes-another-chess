use super::*;

impl From<&str> for Board {
	fn from(value: &str) -> Self {
		let mut board = Self {
			pieces: Pieces([Bitboard::default(); 12]),
			color: Color::White,
			en_passant: None,
			castle_rights: CastleRights::default(),

			piece_list: PieceList::default(),
			occupancy: Bitboard::default(),
			ally: Bitboard::default(),
			enemy: Bitboard::default(),

			history: History::default(),
			halfmove_clock: 0,
			fullmove_number: 1,
		};

		let tokens = value.split_whitespace().collect::<Vec<&str>>();

		board.set_pieces(tokens[0]);
		board.set_color(tokens[1]);
		board.set_castling_rights(tokens[2]);
		board.set_en_passant(tokens[3]);
		board.update_occupancies();

		board
	}
}

impl Board {
	fn set_pieces(&mut self, pieces: &str) {
		let mut rank = Rank::Eighth;
		let mut file = File::A;

		for ch in pieces.chars() {
			match ch {
				'/' => {
					rank.next_back();
					file = File::A;
				}
				'1'..='8' => {
					for _ in 0..ch.to_digit(10).unwrap() {
						file.next();
					}
				}
				_ => {
					self.add_piece(Piece::from(ch), Square::from(RankFile(rank, file)));
					file.next();
				}
			}
		}
	}

	fn set_color(&mut self, color: &str) {
		if let Some(ch) = color.chars().next() {
			self.color = Color::from(ch);
		}
	}

	fn set_castling_rights(&mut self, castling_rights: &str) {
		if castling_rights == "-" {
			return;
		}

		for ch in castling_rights.chars() {
			self.castle_rights |= CastleRight::from(ch);
		}
	}

	fn set_en_passant(&mut self, en_passant: &str) {
		if en_passant != "-" {
			self.en_passant = Some(Square::from(en_passant));
		}
	}
}
