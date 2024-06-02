use super::Magic;
use crate::board::{square::SquareConsts, Bitboard};

pub type PieceMagics = [Magic; usize::SQUARE_SIZE];
pub type PieceMoves = [Bitboard; usize::SQUARE_SIZE];
pub type BlockerTable = Vec<Bitboard>;
pub type AttackTable = Vec<Bitboard>;

pub const ROOK_TABLE_SIZE: usize = 102_400;
pub const BISHOP_TABLE_SIZE: usize = 5_248;
