use super::{
	bitboard::BitboardString, color::ColorConsts, file_rank::FileRankConsts, piece::PieceConsts,
	square::SquareConsts, Bitboard, Color, Piece,
};

pub type PieceList = [Piece; usize::SQUARE_SIZE];
pub type BitboardPieces = [[Bitboard; usize::PIECE_SIZE]; usize::COLOR_SIZE];

pub trait PrintBitboards {
	fn print_bitboards(&self, color: Color);
}

impl PrintBitboards for BitboardPieces {
	fn print_bitboards(&self, color: Color) {
		if let Some(bitboards) = self.get(color) {
			let bitboards = bitboards.iter().map(|bitboard| bitboard.bitboard_string());

			let lines = bitboards
				.map(|s| s.lines().map(|s| s.to_string()).collect::<Vec<String>>())
				.collect::<Vec<Vec<String>>>();

			let mut output = format!(
				"\n{:<17}{:<17}{:<17}{:<17}{:<17}{:<17}",
				"King", "Queen", "Rook", "Bishop", "Knight", "Pawn"
			);

			for rank in usize::FILE_RANK_RANGE.rev() {
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
