use super::{Bitboard, Color, Piece, Rank};
use std::ops;

#[derive(Debug)]
pub struct Pieces(pub [Bitboard; Piece::SIZE * Color::SIZE]);

impl ops::Index<Piece> for Pieces {
	type Output = Bitboard;

	fn index(&self, index: Piece) -> &Self::Output {
		&self.0[index.index()]
	}
}

impl ops::IndexMut<Piece> for Pieces {
	fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
		&mut self.0[index.index()]
	}
}

impl Pieces {
	pub fn iter(&self, color: Color) -> impl Iterator<Item = &Bitboard> {
		self.0
			.iter()
			.enumerate()
			.filter(move |(i, _)| color == *i)
			.map(|(_, bb)| bb)
	}

	pub fn print_bitboards(&self, color: Color) {
		let bitboards = self.iter(color).map(|bitboard| format!("{bitboard}"));

		let lines = bitboards
			.map(|s| s.lines().map(|s| s.to_string()).collect::<Vec<String>>())
			.collect::<Vec<Vec<String>>>();

		let mut output = format!(
			"\n{:<17}{:<17}{:<17}{:<17}{:<17}{:<17}",
			"King", "Queen", "Rook", "Bishop", "Knight", "Pawn"
		);

		for rank in Rank::First.iter() {
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
