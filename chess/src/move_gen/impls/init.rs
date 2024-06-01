use super::{Error, Magic, MoveGen, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE};
use crate::board::{Bitboard, Color, File, Piece, Rank, Square};

impl Default for MoveGen {
	fn default() -> Self {
		let mut move_gen = Self {
			king: [Bitboard::default(); Square::SIZE],
			rooks: vec![Bitboard::default(); ROOK_TABLE_SIZE],
			bishops: vec![Bitboard::default(); BISHOP_TABLE_SIZE],
			rook_magics: [Magic::default(); Square::SIZE],
			bishop_magics: [Magic::default(); Square::SIZE],
			knight: [Bitboard::default(); Square::SIZE],
			pawns: [[Bitboard::default(); Square::SIZE]; Color::SIZE],
		};

		move_gen.init_king();
		move_gen.init_magic(Piece::Rook(Color::Both));
		move_gen.init_magic(Piece::Bishop(Color::Both));
		move_gen.init_knight();
		move_gen.init_pawn();

		move_gen
	}
}

impl MoveGen {
	fn init_king(&mut self) {
		for square in Square::iter() {
			let bitboard = Bitboard::from(square);

			self.king[square] |= (bitboard & !File::A & !Rank::Eighth) << 7
				| (bitboard & !Rank::Eighth) << 8
				| (bitboard & !Rank::Eighth & !File::H) << 9
				| (bitboard & !File::H) << 1
				| (bitboard & !Rank::First & !File::H) >> 7
				| (bitboard & !Rank::First) >> 8
				| (bitboard & !Rank::First & !File::A) >> 9
				| (bitboard & !File::A) >> 1;
		}
	}

	fn init_knight(&mut self) {
		for square in Square::iter() {
			let bitboard = Bitboard::from(square);

			self.knight[square] |= (bitboard & !Rank::Eighth & !Rank::Seventh & !File::A) << 15
				| (bitboard & !Rank::Eighth & !Rank::Seventh & !File::H) << 17
				| (bitboard & !Rank::Eighth & !File::A & !File::B) << 6
				| (bitboard & !Rank::Eighth & !File::G & !File::H) << 10
				| (bitboard & !Rank::First & !Rank::Second & !File::A) >> 17
				| (bitboard & !Rank::First & !Rank::Second & !File::H) >> 15
				| (bitboard & !Rank::First & !File::A & !File::B) >> 10
				| (bitboard & !Rank::First & !File::G & !File::H) >> 6;
		}
	}

	fn init_pawn(&mut self) {
		for square in Square::iter() {
			let bitboard = Bitboard::from(square);

			let white = (bitboard & !File::A) << 7 | (bitboard & !File::H) << 9;
			let black = (bitboard & !File::H) >> 7 | (bitboard & !File::A) >> 9;

			self.pawns[Color::White][square] = white;
			self.pawns[Color::Black][square] = black;
		}
	}

	// Marcel Vanthoor
	// https://github.com/mvanthoor/rustic
	fn init_magic(&mut self, piece: Piece) {
		let is_rook = match piece {
			Piece::Rook(Color::Both) => true,
			Piece::Bishop(Color::Both) => false,
			_ => panic!("{}", Error::InvalidMagicPiece(piece)),
		};

		let mut offset = 0;

		for square in Square::iter() {
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

			let mut magic = Magic {
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
					panic!("{}", Error::InvalidMagicAttack(table[index]));
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
