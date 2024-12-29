mod defs;
mod direction;
mod impls;
mod magic;
mod piece_move;
mod prelude;

use crate::{
	board::{
		bitboard::BitboardUtils,
		castle_right::CastleRights,
		color::{ColorConsts, ColorString, Colors},
		file_rank::RankUtils,
		piece::{PiecePromotions, Pieces},
		square::SquareUtils,
		Bitboard, Board, CastleRight, Color, Piece, Square,
	},
	move_list::MoveList,
};
pub use prelude::*;

#[derive(Debug)]
pub struct MoveGen {
	king: PieceMoves,
	rooks: AttackTable,
	bishops: AttackTable,
	rook_magics: PieceMagics,
	bishop_magics: PieceMagics,
	knight: PieceMoves,
	pawns: [PieceMoves; usize::COLOR_SIZE],
}

impl MoveGen {
	#[inline(always)]
	pub fn king(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let mut piece = board.pieces[color][Piece::KING];

		let square = BitboardUtils::pop_lsb(&mut piece);
		let moves = self.king[square] & !board.ally();

		self.add_move(board, Piece::KING, square, moves, list);
	}

	#[inline(always)]
	pub fn queens(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let occupancy = board.occupancy;
		let ally = board.ally();

		let mut pieces = board.pieces[color][Piece::QUEEN];

		while pieces > 0 {
			let square = BitboardUtils::pop_lsb(&mut pieces);

			let rook_index = self.rook_magics[square].index(occupancy);
			let bishop_index = self.bishop_magics[square].index(occupancy);
			let moves = (self.rooks[rook_index] | self.bishops[bishop_index]) & !ally;

			self.add_move(board, Piece::QUEEN, square, moves, list);
		}
	}

	#[inline(always)]
	pub fn rooks(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let occupancy = board.occupancy;
		let ally = board.ally();

		let mut pieces = board.pieces[color][Piece::ROOK];

		while pieces > 0 {
			let square = BitboardUtils::pop_lsb(&mut pieces);

			let index = self.rook_magics[square].index(occupancy);
			let moves = self.rooks[index] & !ally;

			self.add_move(board, Piece::ROOK, square, moves, list);
		}
	}

	#[inline(always)]
	pub fn bishops(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let occupancy = board.occupancy;
		let ally = board.ally();

		let mut pieces = board.pieces[color][Piece::BISHOP];

		while pieces > 0 {
			let square = BitboardUtils::pop_lsb(&mut pieces);

			let index = self.bishop_magics[square].index(occupancy);
			let moves = self.bishops[index] & !ally;

			self.add_move(board, Piece::BISHOP, square, moves, list);
		}
	}

	#[inline(always)]
	pub fn knights(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let ally = board.ally();

		let mut pieces = board.pieces[color][Piece::KNIGHT];

		while pieces > 0 {
			let square = BitboardUtils::pop_lsb(&mut pieces);
			let moves = self.knight[square] & !ally;

			self.add_move(board, Piece::KNIGHT, square, moves, list);
		}
	}

	#[inline(always)]
	pub fn pawns(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let enemy = board.enemy();
		let empty = !board.occupancy;

		let fourth = match color {
			Color::WHITE => RankUtils::R4,
			Color::BLACK => RankUtils::R5,
			_ => panic!("Invalid color: {}", color.color_string()),
		};

		let direction = match color {
			Color::WHITE => Direction::North,
			Color::BLACK => Direction::South,
			_ => panic!("Invalid color: {}", color.color_string()),
		};

		let rotation_count = (SquareUtils::SIZE + direction) as u32;

		let mut pieces = board.pieces[color][Piece::PAWN];

		while pieces > 0 {
			let square = BitboardUtils::pop_lsb(&mut pieces);
			let to = square + direction;

			let one_step = empty & BitboardUtils::SQUARES[to];
			let two_step =
				one_step.rotate_left(rotation_count) & empty & BitboardUtils::RANKS[fourth];
			let attacks = self.pawns[color][square];
			let captures = attacks & enemy;

			let en_passant = match board.en_passant {
				Some(square) => attacks & BitboardUtils::SQUARES[square],
				None => BitboardUtils::EMPTY,
			};

			let moves = one_step | two_step | captures | en_passant;

			self.add_move(board, Piece::PAWN, square, moves, list);
		}
	}

	#[inline(always)]
	pub fn castling(&self, board: &Board, list: &mut MoveList) {
		let color = board.color;
		let opponent = color ^ 1;
		let occupancy = board.occupancy;

		let rights = board.castle_rights;

		if color == Color::WHITE && rights & CastleRight::WHITE > 0 {
			if rights & CastleRight::WHITE_KING > 0 {
				let blockers = BitboardUtils::SQUARES[SquareUtils::F1]
					| BitboardUtils::SQUARES[SquareUtils::G1];

				if occupancy & blockers == 0
					&& !self.square_attacked(board, opponent, SquareUtils::E1)
					&& !self.square_attacked(board, opponent, SquareUtils::F1)
				{
					self.add_move(
						board,
						Piece::KING,
						SquareUtils::E1,
						BitboardUtils::SQUARES[SquareUtils::G1],
						list,
					);
				}
			}

			if rights & CastleRight::WHITE_QUEEN > 0 {
				let blockers = BitboardUtils::SQUARES[SquareUtils::D1]
					| BitboardUtils::SQUARES[SquareUtils::C1]
					| BitboardUtils::SQUARES[SquareUtils::B1];

				if occupancy & blockers == 0
					&& !self.square_attacked(board, opponent, SquareUtils::E1)
					&& !self.square_attacked(board, opponent, SquareUtils::D1)
				{
					self.add_move(
						board,
						Piece::KING,
						SquareUtils::E1,
						BitboardUtils::SQUARES[SquareUtils::C1],
						list,
					);
				}
			}
		} else if color == Color::BLACK && rights & CastleRight::BLACK > 0 {
			if rights & CastleRight::BLACK_KING > 0 {
				let blockers = BitboardUtils::SQUARES[SquareUtils::F8]
					| BitboardUtils::SQUARES[SquareUtils::G8];

				if occupancy & blockers == 0
					&& !self.square_attacked(board, opponent, SquareUtils::E8)
					&& !self.square_attacked(board, opponent, SquareUtils::F8)
				{
					self.add_move(
						board,
						Piece::KING,
						SquareUtils::E8,
						BitboardUtils::SQUARES[SquareUtils::G8],
						list,
					);
				}
			}

			if rights & CastleRight::BLACK_QUEEN > 0 {
				let blockers = BitboardUtils::SQUARES[SquareUtils::D8]
					| BitboardUtils::SQUARES[SquareUtils::C8]
					| BitboardUtils::SQUARES[SquareUtils::B8];

				if occupancy & blockers == 0
					&& !self.square_attacked(board, opponent, SquareUtils::E8)
					&& !self.square_attacked(board, opponent, SquareUtils::D8)
				{
					self.add_move(
						board,
						Piece::KING,
						SquareUtils::E8,
						BitboardUtils::SQUARES[SquareUtils::C8],
						list,
					);
				}
			}
		}
	}
}

impl MoveGen {
	#[inline(always)]
	pub fn add_move(
		&self,
		board: &Board,
		piece: Piece,
		from: Square,
		moves: Bitboard,
		list: &mut MoveList,
	) {
		let mut moves = moves;
		let color = board.color;
		let is_pawn = Piece::PAWN == piece;

		let promotion_rank = match color {
			Color::WHITE => RankUtils::R8,
			Color::BLACK => RankUtils::R1,
			_ => panic!("Invalid color: {}", color.color_string()),
		};

		while moves > 0 {
			let to = BitboardUtils::pop_lsb(&mut moves);

			let capture = board.piece_list[to];
			let en_passant = match board.en_passant {
				Some(square) => is_pawn && square == to,
				None => false,
			};

			let promotion =
				is_pawn && BitboardUtils::occupied(BitboardUtils::RANKS[promotion_rank], to);
			let two_step = is_pawn && (to as i8 - from as i8).abs() == 16;
			let castling = piece == Piece::KING && (from as i8 - to as i8).abs() == 2;

			let move_data = piece
				| from << Move::FROM_SQUARE
				| to << Move::TO_SQUARE
				| capture << Move::CAPTURE
				| (en_passant as usize) << Move::EN_PASSANT
				| (two_step as usize) << Move::TWO_STEP
				| (castling as usize) << Move::CASTLING;

			if !promotion {
				list.push(Move::new(move_data | Piece::NONE << Move::PROMOTION));
			} else {
				for piece in Piece::PROMOTIONS {
					list.push(Move::new(move_data | piece << Move::PROMOTION));
				}
			}
		}
	}
}
