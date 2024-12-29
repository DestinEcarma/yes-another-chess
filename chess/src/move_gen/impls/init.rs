use super::{Magic, MoveGen, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE};
use crate::board::{
	bitboard::BitboardUtils,
	color::ColorUtils,
	file_rank::{FileUtils, RankUtils},
	piece::{PieceString, Pieces},
	square::SquareUtils,
	Piece,
};

impl Default for MoveGen {
	fn default() -> Self {
		let mut move_gen = Self {
			king: [BitboardUtils::EMPTY; SquareUtils::SIZE],
			rooks: vec![BitboardUtils::EMPTY; ROOK_TABLE_SIZE],
			bishops: vec![BitboardUtils::EMPTY; BISHOP_TABLE_SIZE],
			rook_magics: [Magic::default(); SquareUtils::SIZE],
			bishop_magics: [Magic::default(); SquareUtils::SIZE],
			knight: [BitboardUtils::EMPTY; SquareUtils::SIZE],
			pawns: [[BitboardUtils::EMPTY; SquareUtils::SIZE]; ColorUtils::SIZE],
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
		for square in SquareUtils::RANGE {
			let bitboard = BitboardUtils::SQUARES[square];

			self.king[square] |= (bitboard
				& !BitboardUtils::FILES[FileUtils::A]
				& !BitboardUtils::RANKS[RankUtils::R8])
				<< 7 | (bitboard & !BitboardUtils::RANKS[RankUtils::R8]) << 8
				| (bitboard
					& !BitboardUtils::RANKS[RankUtils::R8]
					& !BitboardUtils::FILES[FileUtils::H])
					<< 9 | (bitboard & !BitboardUtils::FILES[FileUtils::H]) << 1
				| (bitboard
					& !BitboardUtils::RANKS[RankUtils::R1]
					& !BitboardUtils::FILES[FileUtils::H])
					>> 7 | (bitboard & !BitboardUtils::RANKS[RankUtils::R1]) >> 8
				| (bitboard
					& !BitboardUtils::RANKS[RankUtils::R1]
					& !BitboardUtils::FILES[FileUtils::A])
					>> 9 | (bitboard & !BitboardUtils::FILES[FileUtils::A]) >> 1;
		}
	}

	fn init_knight(&mut self) {
		for square in SquareUtils::RANGE {
			let bitboard = BitboardUtils::SQUARES[square];

			self.knight[square] |= (bitboard
				& !BitboardUtils::RANKS[RankUtils::R8]
				& !BitboardUtils::RANKS[RankUtils::R7]
				& !BitboardUtils::FILES[FileUtils::A])
				<< 15 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R8]
				& !BitboardUtils::RANKS[RankUtils::R7]
				& !BitboardUtils::FILES[FileUtils::H])
				<< 17 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R8]
				& !BitboardUtils::FILES[FileUtils::A]
				& !BitboardUtils::FILES[FileUtils::B])
				<< 6 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R8]
				& !BitboardUtils::FILES[FileUtils::G]
				& !BitboardUtils::FILES[FileUtils::H])
				<< 10 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R1]
				& !BitboardUtils::RANKS[RankUtils::R2]
				& !BitboardUtils::FILES[FileUtils::A])
				>> 17 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R1]
				& !BitboardUtils::RANKS[RankUtils::R2]
				& !BitboardUtils::FILES[FileUtils::H])
				>> 15 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R1]
				& !BitboardUtils::FILES[FileUtils::A]
				& !BitboardUtils::FILES[FileUtils::B])
				>> 10 | (bitboard
				& !BitboardUtils::RANKS[RankUtils::R1]
				& !BitboardUtils::FILES[FileUtils::G]
				& !BitboardUtils::FILES[FileUtils::H])
				>> 6;
		}
	}

	fn init_pawn(&mut self) {
		for square in SquareUtils::RANGE {
			let bitboard = BitboardUtils::SQUARES[square];

			let white = (bitboard & !BitboardUtils::FILES[FileUtils::A]) << 7
				| (bitboard & !BitboardUtils::FILES[FileUtils::H]) << 9;
			let black = (bitboard & !BitboardUtils::FILES[FileUtils::H]) >> 7
				| (bitboard & !BitboardUtils::FILES[FileUtils::A]) >> 9;

			self.pawns[ColorUtils::WHITE][square] = white;
			self.pawns[ColorUtils::BLACK][square] = black;
		}
	}

	// Marcel Vanthoor
	// https://github.com/mvanthoor/rustic
	fn init_magic(&mut self, piece: Piece) {
		let is_rook = match piece {
			Piece::ROOK => true,
			Piece::BISHOP => false,
			_ => panic!(
				"Invalid magic piece: {}",
				piece.piece_string(ColorUtils::BOTH)
			),
		};

		let mut offset = 0;

		for square in SquareUtils::RANGE {
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
