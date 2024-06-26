use super::{AttackTable, BlockerTable, Direction, MoveGen};
use crate::board::{
	bitboard::{BitboardFiles, BitboardOccupied, BitboardRanks, BitboardSquares},
	file_rank::{Files, Ranks},
	square::SquareLocation,
	Bitboard, File, Rank, Square,
};

impl MoveGen {
	pub fn rook_mask(square: Square) -> Bitboard {
		let edges = Self::edges(square);
		let (file, rank) = square.location();

		let mask = Bitboard::RANKS[rank] | Bitboard::FILES[file];

		mask & !edges & !Bitboard::SQUARES[square]
	}

	pub fn bishop_mask(square: Square) -> Bitboard {
		let edges = Self::edges(square);
		let mask = Self::ray(Bitboard::default(), square, Direction::NorthEast)
			| Self::ray(Bitboard::default(), square, Direction::NorthWest)
			| Self::ray(Bitboard::default(), square, Direction::SouthEast)
			| Self::ray(Bitboard::default(), square, Direction::SouthWest);

		mask & !edges & !Bitboard::SQUARES[square]
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
		let (file, rank) = square.location();

		let bitboard_file = Bitboard::FILES[file];
		let bitboard_rank = Bitboard::RANKS[rank];

		(!bitboard_file & Bitboard::FILES[File::A])
			| (!bitboard_file & Bitboard::FILES[File::H])
			| (!bitboard_rank & Bitboard::RANKS[Rank::R1])
			| (!bitboard_rank & Bitboard::RANKS[Rank::R8])
	}

	fn ray(bitboard: Bitboard, mut square: Square, direction: Direction) -> Bitboard {
		let mut ray = Bitboard::default();

		while !(direction == square) {
			square += direction;
			ray |= Bitboard::SQUARES[square];

			if bitboard.occupied(square) {
				break;
			}
		}

		ray
	}
}
