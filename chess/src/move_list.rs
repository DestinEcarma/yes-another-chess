use crate::move_gen::Move;
use std::mem;

#[derive(Clone, Copy)]
pub struct MoveList {
	list: [Move; 218],
	count: usize,
}

impl Default for MoveList {
	#[inline(always)]
	fn default() -> Self {
		// Marcel Vanthoor
		// https://github.com/mvanthoor/rustic
		Self {
			list: unsafe {
				let block = mem::MaybeUninit::uninit();
				block.assume_init()
			},
			count: 0,
		}
	}
}

//impl<'a> IntoIterator for &'a MoveList {
//	type Item = &'a Move;
//	type IntoIter = std::slice::Iter<'a, Move>;
//
//	fn into_iter(self) -> Self::IntoIter {
//		self.list[..self.count].iter()
//	}
//}

impl MoveList {
	#[inline(always)]
	pub fn push(&mut self, m: Move) {
		self.list[self.count] = m;
		self.count += 1;
	}

	#[inline(always)]
	pub fn len(&self) -> usize {
		self.count
	}

	#[inline(always)]
	pub fn chunks(&self, chunk_size: usize) -> Vec<Vec<Move>> {
		self.list[..self.count]
			.chunks(chunk_size)
			.map(|chunk| chunk.to_vec())
			.collect()
	}

	pub fn iter(&self) -> std::slice::Iter<Move> {
		self.list[..self.count].iter()
	}
}
