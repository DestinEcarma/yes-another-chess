use castle_right::GetCastleRight;
use square::GetSquare;

use super::*;
use super::{
	color::{ColorConsts, Colors, GetColor},
	file_rank::{Files, Ranks},
	piece::{GetPiece, PieceConsts},
	square::SquareConsts,
};

impl From<&str> for Board {
	fn from(value: &str) -> Self {
		let mut board = Self {
			pieces: [[Bitboard::default(); usize::PIECE_SIZE]; usize::COLOR_SIZE],
			color: Color::WHITE,
			en_passant: None,
			castle_rights: CastleRight::default(),

			piece_list: [Piece::NONE; usize::SQUARE_SIZE],
			occupancy: Bitboard::default(),
			occupancy_color: [Bitboard::default(); usize::COLOR_SIZE],

			halfmove_clock: 0,
			fullmove_number: 1,
		};

		let tokens = value.split_whitespace().collect::<Vec<&str>>();

		board.set_pieces(tokens[0]);
		board.set_color(tokens[1]);
		board.set_castling_rights(tokens[2]);
		board.set_en_passant(tokens[3]);

		if let Some(num) = tokens.get(4) {
			board.halfmove_clock = num.parse().unwrap_or(0);
		}

		if let Some(num) = tokens.get(5) {
			board.fullmove_number = num.parse().unwrap_or(1);
		}

		board
	}
}

impl Board {
	fn set_pieces(&mut self, pieces: &str) {
		let mut rank = Rank::R8;
		let mut file = File::A;

		for ch in pieces.chars() {
			match ch {
				'/' => {
					rank -= 1;
					file = File::A;
				}
				'1'..='8' => {
					for _ in 0..ch.to_digit(10).unwrap() {
						file += 1;
					}
				}
				_ => {
					self.add_piece(
						Piece::get_piece(ch),
						Color::get_color(ch.is_uppercase()),
						Square::get_square((file, rank)),
					);
					file += 1;
				}
			}
		}
	}

	fn set_color(&mut self, color: &str) {
		if let Some(ch) = color.chars().next() {
			self.color = Color::get_color(ch);
		}
	}

	fn set_castling_rights(&mut self, castling_rights: &str) {
		if castling_rights == "-" {
			return;
		}

		for ch in castling_rights.chars() {
			self.castle_rights |= CastleRight::get_castle_right(ch);
		}
	}

	fn set_en_passant(&mut self, en_passant: &str) {
		if en_passant != "-" {
			self.en_passant = Some(Square::get_square(en_passant));
		}
	}
}
