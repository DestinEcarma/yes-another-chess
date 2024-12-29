use crate::board::zobrist::ZobristHash;

use papaya::{HashMap, LocalGuard};

#[derive(Default)]
pub struct TranspositionTable<V> {
	entries: HashMap<ZobristHash, V>,
	max_capacity: usize,
}

impl<V> TranspositionTable<V>
where
	V: Depth,
{
	pub fn new(bytes: usize) -> Self {
		let max_capacity = bytes / std::mem::size_of::<(ZobristHash, V)>();

		Self {
			max_capacity,
			entries: HashMap::with_capacity(max_capacity),
		}
	}

	pub fn insert(&self, key: ZobristHash, data: V) {
		let guard = self.entries.guard();

		if self.entries.len() >= self.max_capacity {
			// TODO: Remove the hightest depth entry.
			//       Might have to consider using a different data structure.

			return;
		}

		self.entries.insert(key, data, &guard);
	}

	pub fn get<'a>(&self, key: &ZobristHash, guard: &'a LocalGuard) -> Option<&'a V> {
		self.entries.get(key, guard)
	}

	pub fn clear(&self) {
		self.entries.clear(&self.entries.guard());
	}

	pub fn guard(&self) -> LocalGuard {
		self.entries.guard()
	}
}

pub trait Depth {
	fn depth(&self) -> u8;
}

#[derive(Default)]
pub struct PerftData {
	depth: u8,
	nodes: usize,
}

impl Depth for PerftData {
	fn depth(&self) -> u8 {
		self.depth
	}
}

impl PerftData {
	pub fn new(depth: u8, nodes: usize) -> Self {
		Self { depth, nodes }
	}

	pub fn nodes(&self) -> usize {
		self.nodes
	}
}
