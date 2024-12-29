use bitboard::BitboardUtils;
use castle_right::GetCastleRight;
use file_rank::{FileUtils, RankUtils};
use square::SquareUtils;

use super::piece::{GetPiece, PieceConsts};
use super::*;

impl From<&str> for Board {
	fn from(value: &str) -> Self {
		Self::from((value, Arc::new(HashTable::default())))
	}
}

impl From<(&str, Arc<HashTable>)> for Board {
	fn from(value: (&str, Arc<HashTable>)) -> Self {
		let mut board = Self {
			pieces: [[BitboardUtils::EMPTY; usize::PIECE_SIZE]; ColorUtils::SIZE],
			color: ColorUtils::WHITE,
			en_passant: None,
			castle_rights: CastleRight::default(),

			piece_list: [Piece::NONE; SquareUtils::SIZE],
			occupancy: BitboardUtils::EMPTY,
			occupancy_color: [BitboardUtils::EMPTY; ColorUtils::SIZE],

			halfmove_clock: 0,
			fullmove_number: 1,

			hash: ZobristHash::default(),

			hash_table: value.1,
		};

		let tokens = value.0.split_whitespace().collect::<Vec<&str>>();

		BoardBuilder::set_pieces(&mut board, tokens[0]);
		BoardBuilder::set_color(&mut board, tokens[1]);
		BoardBuilder::set_castling_rights(&mut board, tokens[2]);
		BoardBuilder::set_en_passant(&mut board, tokens[3]);

		if let Some(num) = tokens.get(4) {
			board.halfmove_clock = num.parse().unwrap_or(0);
		}

		if let Some(num) = tokens.get(5) {
			board.fullmove_number = num.parse().unwrap_or(1);
		}

		board.init_hash();

		board
	}
}

impl Board {
	fn init_hash(&mut self) {
		self.hash = 0;

		let bb_white = self.pieces[ColorUtils::WHITE];
		let bb_black = self.pieces[ColorUtils::BLACK];

		for (piece, (white, black)) in bb_white.iter().zip(bb_black.iter()).enumerate() {
			let mut white_pieces = *white;
			let mut black_pieces = *black;

			while white_pieces > 0 {
				let square = BitboardUtils::pop_lsb(&mut white_pieces);

				self.hash ^= self.hash_table.piece(piece, ColorUtils::WHITE, square);
			}

			while black_pieces > 0 {
				let square = BitboardUtils::pop_lsb(&mut black_pieces);

				self.hash ^= self.hash_table.piece(piece, ColorUtils::BLACK, square);
			}
		}

		self.hash ^= self.hash_table.color(self.color);
		self.hash ^= self.hash_table.castle(self.castle_rights);
		self.hash ^= self.hash_table.en_passant(self.en_passant);
	}
}

struct BoardBuilder;

impl BoardBuilder {
	fn set_pieces(board: &mut Board, pieces: &str) {
		let mut rank = RankUtils::R8;
		let mut file = FileUtils::A;

		for ch in pieces.chars() {
			match ch {
				'/' => {
					rank -= 1;
					file = FileUtils::A;
				}
				'1'..='8' => {
					for _ in 0..ch.to_digit(10).unwrap() {
						file += 1;
					}
				}
				_ => {
					board.add_piece(
						Piece::get_piece(ch),
						ColorUtils::from_bool(ch.is_uppercase()),
						SquareUtils::from_location(file, rank),
					);
					file += 1;
				}
			}
		}
	}

	fn set_color(board: &mut Board, color: &str) {
		if let Some(ch) = color.chars().next() {
			board.color = ColorUtils::parse(ch);
		}
	}

	fn set_castling_rights(board: &mut Board, castling_rights: &str) {
		if castling_rights == "-" {
			return;
		}

		for ch in castling_rights.chars() {
			board.castle_rights |= CastleRight::get_castle_right(ch);
		}
	}

	fn set_en_passant(board: &mut Board, en_passant: &str) {
		if en_passant != "-" {
			board.en_passant = Some(SquareUtils::parse(en_passant));
		}
	}
}
