use super::{AttackTable, BlockerTable, Direction, MoveGen};
use crate::board::{
	bitboard::BitboardUtils,
	file_rank::{FileUtils, RankUtils},
	square::SquareUtils,
	Bitboard, Square,
};

impl MoveGen {
	pub fn rook_mask(square: Square) -> Bitboard {
		let edges = Self::edges(square);
		let (file, rank) = SquareUtils::location(square);

		let mask = BitboardUtils::RANKS[rank] | BitboardUtils::FILES[file];

		mask & !edges & !BitboardUtils::SQUARES[square]
	}

	pub fn bishop_mask(square: Square) -> Bitboard {
		let edges = Self::edges(square);
		let mask = Self::ray(BitboardUtils::EMPTY, square, Direction::NorthEast)
			| Self::ray(BitboardUtils::EMPTY, square, Direction::NorthWest)
			| Self::ray(BitboardUtils::EMPTY, square, Direction::SouthEast)
			| Self::ray(BitboardUtils::EMPTY, square, Direction::SouthWest);

		mask & !edges & !BitboardUtils::SQUARES[square]
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
		let mut bitboard = BitboardUtils::EMPTY;

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
		let (file, rank) = SquareUtils::location(square);

		let bitboard_file = BitboardUtils::FILES[file];
		let bitboard_rank = BitboardUtils::RANKS[rank];

		(!bitboard_file & BitboardUtils::FILES[FileUtils::A])
			| (!bitboard_file & BitboardUtils::FILES[FileUtils::H])
			| (!bitboard_rank & BitboardUtils::RANKS[RankUtils::R1])
			| (!bitboard_rank & BitboardUtils::RANKS[RankUtils::R8])
	}

	fn ray(bitboard: Bitboard, mut square: Square, direction: Direction) -> Bitboard {
		let mut ray = BitboardUtils::EMPTY;

		while !(direction == square) {
			square += direction;
			ray |= BitboardUtils::SQUARES[square];

			if BitboardUtils::occupied(bitboard, square) {
				break;
			}
		}

		ray
	}
}
