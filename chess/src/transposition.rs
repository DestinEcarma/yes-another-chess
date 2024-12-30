use crate::board::zobrist::ZobristHash;

const ENTRIES_PER_BUCKET: usize = 4;
const HIGH_FOUR_BYTES: u64 = 0xFF_FF_FF_FF_00_00_00_00;
const LOW_FOUR_BYTES: u64 = 0x00_00_00_00_FF_FF_FF_FF;
const SHIFT_TO_LOWER: u64 = 32;

pub trait Depth {
	fn depth(&self) -> u8;
}

#[derive(Default, Clone)]
pub struct Entry<V> {
	data: V,
	verification: u32,
}

#[derive(Default, Clone)]
pub struct Bucket<V>([Entry<V>; ENTRIES_PER_BUCKET]);

impl<V: Depth> Bucket<V> {
	pub fn insert(&mut self, verification: u32, data: V) {
		let mut idx_lowest_depth = 0;

		for (idx, entry) in self.0.iter().enumerate() {
			if entry.data.depth() < data.depth() {
				idx_lowest_depth = idx;
			}
		}

		self.0[idx_lowest_depth] = Entry { data, verification };
	}

	pub fn get(&self, verification: u32) -> Option<&V> {
		for entry in self.0.iter() {
			if entry.verification == verification {
				return Some(&entry.data);
			}
		}

		None
	}
}

#[derive(Default)]
pub struct TranspositionTable<V> {
	entries: Vec<Bucket<V>>,
}

impl<V: Depth + Default + Clone> TranspositionTable<V> {
	pub fn new(bytes: usize) -> Self {
		let total_buckets = bytes / std::mem::size_of::<Bucket<V>>();

		Self {
			entries: vec![Bucket::default(); total_buckets],
		}
	}

	pub fn insert(&mut self, hash: ZobristHash, data: V) {
		let index = self.calculate_index(hash);
		let verification = self.calculate_verification(hash);

		self.entries[index].insert(verification, data);
	}

	pub fn get(&self, hash: ZobristHash) -> Option<&V> {
		let index = self.calculate_index(hash);
		let verification = self.calculate_verification(hash);

		self.entries[index].get(verification)
	}
}

impl<V> TranspositionTable<V> {
	fn calculate_index(&self, hash: ZobristHash) -> usize {
		let key = (hash & HIGH_FOUR_BYTES) >> SHIFT_TO_LOWER;
		let total = self.entries.len() as u64;

		(key % total) as usize
	}

	fn calculate_verification(&self, hash: ZobristHash) -> u32 {
		(hash & LOW_FOUR_BYTES) as u32
	}
}

#[derive(Default, Clone)]
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
