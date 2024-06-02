use super::{Magic, MoveGen, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE};
use crate::board::{
	bitboard::{BitboardFiles, BitboardRanks, BitboardSquares},
	color::{ColorConsts, Colors},
	file_rank::{Files, Ranks},
	piece::{PieceString, Pieces},
	square::SquareConsts,
	Bitboard, Color, File, Piece, Rank,
};

impl Default for MoveGen {
	fn default() -> Self {
		let mut move_gen = Self {
			king: [Bitboard::default(); usize::SQUARE_SIZE],
			rooks: vec![Bitboard::default(); ROOK_TABLE_SIZE],
			bishops: vec![Bitboard::default(); BISHOP_TABLE_SIZE],
			rook_magics: [Magic::default(); usize::SQUARE_SIZE],
			bishop_magics: [Magic::default(); usize::SQUARE_SIZE],
			knight: [Bitboard::default(); usize::SQUARE_SIZE],
			pawns: [[Bitboard::default(); usize::SQUARE_SIZE]; usize::COLOR_SIZE],
		};

		move_gen.init_king();
		move_gen.init_magic(Piece::ROOK);
		move_gen.init_magic(Piece::BISHOP);
		move_gen.init_knight();
		move_gen.init_pawn();

		move_gen
	}
}

impl MoveGen {
	fn init_king(&mut self) {
		for square in usize::SQUARE_RANGE {
			let bitboard = Bitboard::SQUARES[square];

			self.king[square] |=
				(bitboard & !Bitboard::FILES[File::A] & !Bitboard::RANKS[Rank::R8]) << 7
					| (bitboard & !Bitboard::RANKS[Rank::R8]) << 8
					| (bitboard & !Bitboard::RANKS[Rank::R8] & !Bitboard::FILES[File::H]) << 9
					| (bitboard & !Bitboard::FILES[File::H]) << 1
					| (bitboard & !Bitboard::RANKS[Rank::R1] & !Bitboard::FILES[File::H]) >> 7
					| (bitboard & !Bitboard::RANKS[Rank::R1]) >> 8
					| (bitboard & !Bitboard::RANKS[Rank::R1] & !Bitboard::FILES[File::A]) >> 9
					| (bitboard & !Bitboard::FILES[File::A]) >> 1;
		}
	}

	fn init_knight(&mut self) {
		for square in usize::SQUARE_RANGE {
			let bitboard = Bitboard::SQUARES[square];

			self.knight[square] |= (bitboard
				& !Bitboard::RANKS[Rank::R8]
				& !Bitboard::RANKS[Rank::R7]
				& !Bitboard::FILES[File::A])
				<< 15 | (bitboard
				& !Bitboard::RANKS[Rank::R8]
				& !Bitboard::RANKS[Rank::R7]
				& !Bitboard::FILES[File::H])
				<< 17 | (bitboard
				& !Bitboard::RANKS[Rank::R8]
				& !Bitboard::FILES[File::A]
				& !Bitboard::FILES[File::B])
				<< 6 | (bitboard
				& !Bitboard::RANKS[Rank::R8]
				& !Bitboard::FILES[File::G]
				& !Bitboard::FILES[File::H])
				<< 10 | (bitboard
				& !Bitboard::RANKS[Rank::R1]
				& !Bitboard::RANKS[Rank::R2]
				& !Bitboard::FILES[File::A])
				>> 17 | (bitboard
				& !Bitboard::RANKS[Rank::R1]
				& !Bitboard::RANKS[Rank::R2]
				& !Bitboard::FILES[File::H])
				>> 15 | (bitboard
				& !Bitboard::RANKS[Rank::R1]
				& !Bitboard::FILES[File::A]
				& !Bitboard::FILES[File::B])
				>> 10 | (bitboard
				& !Bitboard::RANKS[Rank::R1]
				& !Bitboard::FILES[File::G]
				& !Bitboard::FILES[File::H])
				>> 6;
		}
	}

	fn init_pawn(&mut self) {
		for square in usize::SQUARE_RANGE {
			let bitboard = Bitboard::SQUARES[square];

			let white = (bitboard & !Bitboard::FILES[File::A]) << 7
				| (bitboard & !Bitboard::FILES[File::H]) << 9;
			let black = (bitboard & !Bitboard::FILES[File::H]) >> 7
				| (bitboard & !Bitboard::FILES[File::A]) >> 9;

			self.pawns[Color::WHITE][square] = white;
			self.pawns[Color::BLACK][square] = black;
		}
	}

	// Marcel Vanthoor
	// https://github.com/mvanthoor/rustic
	fn init_magic(&mut self, piece: Piece) {
		let is_rook = match piece {
			Piece::ROOK => true,
			Piece::BISHOP => false,
			_ => panic!("Invalid magic piece: {}", piece.piece_string(Color::BOTH)),
		};

		let mut offset = 0;

		for square in usize::SQUARE_RANGE {
			let mask = match is_rook {
				true => MoveGen::rook_mask(square),
				false => MoveGen::bishop_mask(square),
			};

			let bits = mask.count_ones();
			let permutations = 2u64.pow(bits);
			let end = offset + permutations - 1;

			let blockers = MoveGen::blockers(mask);

			let attacks = match is_rook {
				true => MoveGen::rook_attacks(square, &blockers),
				false => MoveGen::bishop_attacks(square, &blockers),
			};

			let magic = Magic {
				mask,
				offset,
				shift: (64 - bits) as u8,
				nr: match is_rook {
					true => Magic::ROOK[square],
					false => Magic::BISHOP[square],
				},
			};

			for i in 0..permutations {
				let next = i as usize;
				let index = magic.index(blockers[next]);

				let table = match is_rook {
					true => &mut self.rooks,
					false => &mut self.bishops,
				};

				if table[index] == 0 {
					let fial_low = index < offset as usize;
					let fail_high = index > end as usize;
					assert!(!fial_low || !fail_high, "Indexing error. Error in Magics.");

					table[index] = attacks[next];
				} else {
					panic!("Invalid magic index: {}", magic.nr);
				}
			}

			match is_rook {
				true => self.rook_magics[square] = magic,
				false => self.bishop_magics[square] = magic,
			}

			offset += permutations;
		}
	}
}
