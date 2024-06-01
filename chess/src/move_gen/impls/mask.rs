use super::{AttackTable, BlockerTable, Direction, MoveGen};
use crate::board::{Bitboard, File, Rank, RankFile, Square};

impl MoveGen {
	pub fn rook_mask(square: Square) -> Bitboard {
		let edges = Self::edges(square);
		let mask = Rank::from(square) | File::from(square);

		mask & !edges & !square
	}

	pub fn bishop_mask(square: Square) -> Bitboard {
		let edges = Self::edges(square);
		let mask = Self::ray(Bitboard::default(), square, Direction::NorthEast)
			| Self::ray(Bitboard::default(), square, Direction::NorthWest)
			| Self::ray(Bitboard::default(), square, Direction::SouthEast)
			| Self::ray(Bitboard::default(), square, Direction::SouthWest);

		mask & !edges & !square
	}

	pub fn rook_attacks(square: Square, blockers: &BlockerTable) -> AttackTable {
		let mut attacks = AttackTable::default();

		for bitboards in blockers {
			let attack = MoveGen::ray(*bitboards, square, Direction::North)
				| MoveGen::ray(*bitboards, square, Direction::South)
				| MoveGen::ray(*bitboards, square, Direction::East)
				| MoveGen::ray(*bitboards, square, Direction::West);

			attacks.push(attack);
		}

		attacks
	}

	pub fn bishop_attacks(square: Square, blockers: &BlockerTable) -> AttackTable {
		let mut attacks = AttackTable::default();

		for bitboards in blockers {
			let attack = MoveGen::ray(*bitboards, square, Direction::NorthEast)
				| MoveGen::ray(*bitboards, square, Direction::NorthWest)
				| MoveGen::ray(*bitboards, square, Direction::SouthEast)
				| MoveGen::ray(*bitboards, square, Direction::SouthWest);

			attacks.push(attack);
		}

		attacks
	}

	pub fn blockers(mask: Bitboard) -> BlockerTable {
		let mut blockers = BlockerTable::default();
		let mut bitboard = Bitboard::default();

		// Carry-Rippler
		// https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
		loop {
			blockers.push(bitboard);
			bitboard = bitboard.wrapping_sub(mask) & mask;

			if bitboard == 0 {
				break;
			}
		}

		blockers
	}

	fn edges(square: Square) -> Bitboard {
		let rank = Rank::from(square);
		let file = File::from(square);

		(!file & File::A) | (!file & File::H) | (!rank & Rank::First) | (!rank & Rank::Eighth)
	}

	fn ray(bitboard: Bitboard, mut square: Square, direction: Direction) -> Bitboard {
		let mut ray = Bitboard::default();

		loop {
			match !(direction == square) {
				true => {
					square += direction;
					ray |= square;

					if bitboard.occupied(square) {
						break;
					}
				}
				false => break,
			}
		}

		ray
	}
}
