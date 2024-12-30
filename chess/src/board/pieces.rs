use crate::board::{bitboard::BitboardUtils, file_rank::RankUtils};

use super::{color::ColorUtils, piece::PieceUtils, square::SquareUtils, Bitboard, Color, Piece};

pub type PieceList = [Piece; SquareUtils::SIZE];
pub type BitboardPieces = [[Bitboard; PieceUtils::SIZE]; ColorUtils::SIZE];

pub trait PrintBitboards {
	fn print_bitboards(&self, color: Color);
}

impl PrintBitboards for BitboardPieces {
	fn print_bitboards(&self, color: Color) {
		if let Some(bitboards) = self.get(color) {
			let bitboards = bitboards
				.iter()
				.map(|bitboard| BitboardUtils::to_string(*bitboard));

			let lines = bitboards
				.map(|s| s.lines().map(|s| s.to_string()).collect::<Vec<_>>())
				.collect::<Vec<_>>();

			let piece = PieceUtils::RANGE
				.map(|piece| PieceUtils::to_string(piece, ColorUtils::BOTH))
				.collect::<Vec<_>>();

			let mut output = format!(
				"\n{:<17}{:<17}{:<17}{:<17}{:<17}{:<17}",
				piece[0], piece[1], piece[2], piece[3], piece[4], piece[5]
			);

			for rank in RankUtils::RANGE {
				let mut combined_line = String::new();

				for (piece, line) in lines.iter().enumerate() {
					if piece != 0 {
						combined_line += " ";
					}

					combined_line += &line[rank];
				}

				output += &format!("\n{combined_line}");
			}

			println!("{output}\n")
		}
	}
}
