use super::MoveGen;
use crate::board::{piece::PieceUtils, Board, Color, Square};

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

		(king & board.pieces[opponent][PieceUtils::KING] > 0)
			|| (queen & board.pieces[opponent][PieceUtils::QUEEN] > 0)
			|| (rook & board.pieces[opponent][PieceUtils::ROOK] > 0)
			|| (bishop & board.pieces[opponent][PieceUtils::BISHOP] > 0)
			|| (knight & board.pieces[opponent][PieceUtils::KNIGHT] > 0)
			|| (pawn & board.pieces[opponent][PieceUtils::PAWN] > 0)
	}
}
