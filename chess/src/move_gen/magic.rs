// Derived from https://github.com/mvanthoor/rustic

use crate::board::{Bitboard, Square};

#[derive(Debug, Copy, Clone, Default)]
pub struct Magic {
	pub mask: Bitboard,
	pub shift: u8,
	pub offset: u64,
	pub nr: u64,
}

impl Magic {
	// #[rustfmt::skip]
	pub const ROOK: [u64; Square::SIZE] = [
		0x008000108a204000,
		0x0140012000441000,
		0x0c80100080200508,
		0x0180080006100080,
		0x0080040058008042,
		0x80801400800a0001,
		0x0400021021018824,
		0x04800080006a4100,
		0x098080008c29c002,
		0xf011004000810020,
		0x0401001900402000,
		0x010200102200401a,
		0x0000800400080080,
		0x03c0801200040180,
		0x8004000430680326,
		0x3108800080004900,
		0x00004d0021008001,
		0x1120004000403000,
		0xa00a020020809042,
		0x0010008008011380,
		0x0201010004114800,
		0x0001010008028400,
		0x0210e40003288a10,
		0x0027460001048344,
		0x0d45400480002890,
		0x0001400080200081,
		0x0000200100410010,
		0x0008080080801000,
		0x5008240080080082,
		0x00820c0080800200,
		0x00290001000200b4,
		0x2100008200010044,
		0x8a40400090800021,
		0x0004402000401000,
		0x0102c30011002000,
		0x4002300080800800,
		0x0ac0041101000800,
		0x8040800400802201,
		0x40341d1804001006,
		0x3410800040800900,
		0x0680102000404000,
		0x0010062008c04008,
		0x5280410020030010,
		0x042100100061000a,
		0x0022780011010004,
		0x0012220004008080,
		0x4220020310040008,
		0x80001415c0820001,
		0x8008800100214100,
		0x1001004000708100,
		0x02b9032000154500,
		0x9008001000840880,
		0x240d080084008080,
		0x0080800201040080,
		0x0602281022810400,
		0x0240008441040200,
		0x0001450080061021,
		0x0010210040820032,
		0x8003002002401813,
		0x0100a92004100101,
		0x000200a104100802,
		0x60410018060c0041,
		0x0044420083301804,
		0x104a0100428400e6,
	];

	// #[rustfmt::skip]
	pub const BISHOP: [u64; Square::SIZE] = [
		0x0842084131140100,
		0x000810268ea10009,
		0x004808010a200084,
		0x4208218620880004,
		0x0044050402204020,
		0x2009041242881000,
		0x0011880808041091,
		0x0d12004242082010,
		0x0000258428020401,
		0x0000100401144201,
		0x0082300400425001,
		0x0208040c12800016,
		0x14042e0210000000,
		0x0022008220200404,
		0x0814348410021000,
		0x0004005404052810,
		0x2219084022040420,
		0x8022022004040480,
		0x00a0442202040060,
		0x0304000802542001,
		0x4093800c00a00029,
		0x0880200e10042000,
		0x0203080584104602,
		0x0000600202050409,
		0x0148048108208880,
		0x098a1e0020040401,
		0x2004500403050200,
		0x0001180204004010,
		0x0241010102304000,
		0xc001010080300800,
		0x000802000080a463,
		0x0221202002020100,
		0x1252202268b00e02,
		0x01485804000a2410,
		0x0804202802100080,
		0x0081860080480280,
		0x098c080a00102008,
		0x00a014050000a080,
		0x80442400860400c0,
		0x005aa40080a24a00,
		0x800e842022404800,
		0x4290622804042000,
		0x0001104128001000,
		0x0414060164020200,
		0x4021400102121100,
		0x0214901040400a00,
		0x8ea02421a2000080,
		0xc4100102002c2190,
		0x0065010832400080,
		0x0802028404064000,
		0x0000908158081000,
		0x0142080484040004,
		0x00650010e2020002,
		0x0088200410008000,
		0x002c850408020910,
		0x0204010404008118,
		0x40820041c4100802,
		0x0014850108822000,
		0x47000200c0441040,
		0x8202201802050400,
		0x00c002c070020201,
		0x8008a01060110100,
		0x0000a212020a0c00,
		0x0188105009405081,
	];

	pub fn index(&self, occupancy: Bitboard) -> usize {
		let blocker = occupancy & self.mask;
		((blocker.wrapping_mul(self.nr) >> self.shift) + self.offset) as usize
	}
}

impl std::ops::Index<Square> for [u64; Square::SIZE] {
	type Output = u64;

	fn index(&self, index: Square) -> &Self::Output {
		&self[index as usize]
	}
}

#[cfg(debug_assertions)]
pub mod gen {
	use std::time::Instant;

	use super::super::{Error, MagicTable};
	use super::Magic;
	use crate::board::{Bitboard, Color, Piece, Square};
	use crate::move_gen::{MoveGen, BISHOP_TABLE_SIZE, ROOK_TABLE_SIZE};
	use rand::{Rng, SeedableRng};

	fn random_u64() -> u64 {
		let mut rng = rand::thread_rng();

		rng.gen::<u64>()
	}

	impl Magic {
		pub fn generate(piece: Piece) {
			let start = Instant::now();

			let is_rook = match piece {
				Piece::Rook(Color::Both) => true,
				Piece::Bishop(Color::Both) => false,
				_ => panic!("{}", Error::InvalidMagicPiece(piece)),
			};

			let mut table = match is_rook {
				true => vec![Bitboard::default(); ROOK_TABLE_SIZE],
				false => vec![Bitboard::default(); BISHOP_TABLE_SIZE],
			};

			let mut random = rand::thread_rng();
			let mut offset = 0;

			println!("Generating magics for {piece}");

			for square in Square::iter() {
				let mask = match is_rook {
					true => MoveGen::rook_mask(square),
					false => MoveGen::bishop_mask(square),
				};

				let bits = mask.0.count_ones();
				let permutations = 2u64.pow(bits);
				let end = offset + permutations - 1;

				let blockers = MoveGen::blockers(mask);

				let attacks = match is_rook {
					true => MoveGen::rook_attacks(square, &blockers),
					false => MoveGen::bishop_attacks(square, &blockers),
				};

				let mut magic = Self {
					mask,
					offset,
					nr: permutations,
					shift: (64 - bits) as u8,
				};

				let mut found = false;

				while !found {
					found = true;

					magic.nr = random.gen::<u64>() & random.gen::<u64>() & random.gen::<u64>();

					for i in 0..permutations {
						let next = i as usize;
						let index = magic.index(blockers[next]);

						if table[index] == 0 {
							let fial_low = index < offset as usize;
							let fail_high = index > end as usize;
							assert!(!fial_low || !fail_high, "Indexing error.");

							table[index] = attacks[next];
						} else {
							for wipe_index in offset..=end {
								table[wipe_index as usize] = Bitboard::default();
							}

							found = false;
							break;
						}
					}
				}

				offset += permutations;

				println!("Magic on square {square} is {:<#018x}", magic.nr);
			}

			println!(
				"Generated magics for {piece} in {} seconds.",
				start.elapsed().as_secs_f64()
			);
		}
	}
}
