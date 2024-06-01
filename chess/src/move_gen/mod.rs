mod defs;
mod direction;
mod error;
mod impls;
mod magic;
mod piece_move;
mod prelude;

use crate::board::{Bitboard, Board, CastleRight, Color, Piece, Rank, Square};
pub use prelude::*;

#[derive(Debug)]
pub struct MoveGen {
	king: PieceMoves,
	rooks: AttackTable,
	bishops: AttackTable,
	rook_magics: PieceMagics,
	bishop_magics: PieceMagics,
	knight: PieceMoves,
	pawns: [PieceMoves; Color::SIZE],
}

impl MoveGen {
	#[inline(always)]
	pub(crate) fn king(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let mut piece = board.pieces[Piece::King(color)];
		let square = piece.lsb();
		let moves = self.king[square] & !board.ally;

		self.add_move(board, Piece::King(color), square, moves, list);
	}

	#[inline(always)]
	pub(crate) fn queens(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let mut pieces = board.pieces[Piece::Queen(color)];
		let occupancy = board.occupancy;
		let ally = board.ally;

		while pieces > 0 {
			let square = pieces.lsb();
			let rook_index = self.rook_magics[square].index(board.occupancy);
			let bishop_index = self.bishop_magics[square].index(board.occupancy);
			let moves = (self.rooks[rook_index] | self.bishops[bishop_index]) & !board.ally;

			self.add_move(board, Piece::Queen(color), square, moves, list);
		}
	}

	#[inline(always)]
	pub(crate) fn rooks(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let mut pieces = board.pieces[Piece::Rook(color)];

		while pieces > 0 {
			let square = pieces.lsb();
			let index = self.rook_magics[square].index(board.occupancy);
			let moves = self.rooks[index] & !board.ally;

			self.add_move(board, Piece::Rook(color), square, moves, list);
		}
	}

	#[inline(always)]
	pub(crate) fn bishops(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let mut pieces = board.pieces[Piece::Bishop(color)];

		while pieces > 0 {
			let square = pieces.lsb();
			let index = self.bishop_magics[square].index(board.occupancy);
			let moves = self.bishops[index] & !board.ally;

			self.add_move(board, Piece::Bishop(color), square, moves, list);
		}
	}

	#[inline(always)]
	pub(crate) fn knights(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let mut pieces = board.pieces[Piece::Knight(color)];

		while pieces > 0 {
			let square = pieces.lsb();
			let moves = self.knight[square] & !board.ally;

			self.add_move(board, Piece::Knight(color), square, moves, list);
		}
	}

	#[inline(always)]
	pub(crate) fn pawns(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let enemy = board.enemy;
		let empty = !board.occupancy;

		let fourth = match color {
			Color::White => Rank::Fourth,
			Color::Black => Rank::Fifth,
			_ => panic!("{}", Error::InvalidColor(color)),
		};

		let direction = match color {
			Color::White => Direction::North,
			Color::Black => Direction::South,
			_ => panic!("{}", Error::InvalidColor(color)),
		};

		let rotation_count = (Square::SIZE + direction) as u32;

		let mut pieces = board.pieces[Piece::Pawn(color)];

		while pieces > 0 {
			let square = pieces.lsb();
			let to = square + direction;

			let one_step = empty & to;
			let two_step = one_step.rotate_left(rotation_count) & empty & fourth;
			let attacks = self.pawns[color][square];
			let captures = attacks & enemy;

			let en_passant = match board.en_passant {
				Some(square) => attacks & square,
				None => Bitboard::default(),
			};

			let moves = one_step | two_step | captures | en_passant;

			self.add_move(board, Piece::Pawn(color), square, moves, list);
		}
	}

	#[inline(always)]
	pub(crate) fn castling(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let opponent = !color;
		let occupancy = board.occupancy;
		let mut rights = board.castle_rights & color;

		let square = match color {
			Color::White => Square::E1,
			Color::Black => Square::E8,
			_ => panic!("{}", Error::InvalidColor(color)),
		};

		while rights > 0 {
			let castle_right_index = rights.trailing_zeros() as u8;

			let castle_right = CastleRight::from(castle_right_index);
			let blocker = castle_right.blocker();
			let neighbor = castle_right.neighbor();

			if (occupancy & blocker) == 0
				&& !self.square_attacked(board, opponent, square)
				&& !self.square_attacked(board, opponent, neighbor)
			{
				let to = castle_right.square();

				self.add_move(board, Piece::King(color), square, Bitboard::from(to), list)
			}

			rights ^= 1 << castle_right_index;
		}
	}
}

impl MoveGen {
	#[inline(always)]
	pub(crate) fn add_move(
		&self,
		board: &Board,
		piece: Piece,
		from: Square,
		moves: Bitboard,
		list: &mut MoveList,
	) {
		let mut moves = moves;
		let color = board.color;
		let is_pawn = Piece::Pawn(color) == piece;

		let promotion_rank = match color {
			Color::White => Rank::Eighth,
			Color::Black => Rank::First,
			_ => panic!("{}", Error::InvalidColor(color)),
		};

		while moves > 0 {
			let to = moves.lsb();

			let capture = match board.piece_list[to] {
				Some(piece) => piece.index(),
				None => 0,
			};
			let en_passant = match board.en_passant {
				Some(square) => is_pawn && square == to,
				None => false,
			};

			let promotion = is_pawn && Bitboard::from(promotion_rank).occupied(to);
			let two_step = is_pawn && (to - from).abs() == 16;
			let castling = piece == Piece::King(color) && (from - to).abs() == 2;

			let move_data = piece.index()
				| from << Move::FROM_SQUARE
				| to << Move::TO_SQUARE
				| capture << Move::CAPTURE
				| (en_passant as usize) << Move::EN_PASSANT
				| (two_step as usize) << Move::TWO_STEP
				| (castling as usize) << Move::CASTLING;

			if !promotion {
				list.push(Move::new(move_data));
			} else {
				for piece in Piece::promotions(color) {
					list.push(Move::new(move_data | piece.index() << Move::PROMOTION));
				}
			}
		}
	}
}
