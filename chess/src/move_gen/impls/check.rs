use crate::board::{Bitboard, Board, Color, Piece, Square};

use super::MoveGen;

impl MoveGen {
	#[inline(always)]
	pub fn square_attacked(&self, board: &Board, opponent: Color, square: Square) -> bool {
		let occupancy = board.occupancy;

		let king = self.king[square];
		let rook = self.rooks[self.rook_magics[square].index(occupancy)];
		let bishop = self.bishops[self.bishop_magics[square].index(occupancy)];
		let knight = self.knight[square];
		let pawn = self.pawns[!opponent][square];
		let queen = rook | bishop;

		(king & board.pieces[Piece::King(opponent)] > 0)
			|| (queen & board.pieces[Piece::Queen(opponent)] > 0)
			|| (rook & board.pieces[Piece::Rook(opponent)] > 0)
			|| (bishop & board.pieces[Piece::Bishop(opponent)] > 0)
			|| (knight & board.pieces[Piece::Knight(opponent)] > 0)
			|| (pawn & board.pieces[Piece::Pawn(opponent)] > 0)
	}
}
