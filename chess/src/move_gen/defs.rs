use super::Magic;
use crate::board::{square::SquareUtils, Bitboard};

pub type PieceMagics = [Magic; SquareUtils::SIZE];
pub type PieceMoves = [Bitboard; SquareUtils::SIZE];
pub type BlockerTable = Vec<Bitboard>;
pub type AttackTable = Vec<Bitboard>;

pub const ROOK_TABLE_SIZE: usize = 102_400;
pub const BISHOP_TABLE_SIZE: usize = 5_248;
