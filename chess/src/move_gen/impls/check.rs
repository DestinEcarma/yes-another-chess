use super::MoveGen;
use crate::board::{piece::Pieces, Board, Color, Piece, Square};

impl MoveGen {
	#[inline(always)]
	pub fn square_attacked(&self, board: &Board, opponent: Color, square: Square) -> bool {
		let occupancy = board.occupancy;

		let king = self.king[square];
		let rook = self.rooks[self.rook_magics[square].index(occupancy)];
		let bishop = self.bishops[self.bishop_magics[square].index(occupancy)];
		let knight = self.knight[square];
		let pawn = self.pawns[opponent ^ 1][square];
		let queen = rook | bishop;

		(king & board.pieces[opponent][Piece::KING] > 0)
			|| (queen & board.pieces[opponent][Piece::QUEEN] > 0)
			|| (rook & board.pieces[opponent][Piece::ROOK] > 0)
			|| (bishop & board.pieces[opponent][Piece::BISHOP] > 0)
			|| (knight & board.pieces[opponent][Piece::KNIGHT] > 0)
			|| (pawn & board.pieces[opponent][Piece::PAWN] > 0)
	}
}
