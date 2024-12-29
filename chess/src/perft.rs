use crate::{
	transposition::{Depth as _, PerftData, TranspositionTable},
	Chess,
};

use std::{sync::Arc, thread};

impl Chess {
	#[inline(always)]
	pub fn perft(
		&mut self,
		depth: u8,
		mut num_threads: usize,
		bytes: Option<usize>,
	) -> (usize, u128) {
		if depth == 0 {
			return (0, 0);
		}

		let num_cores = num_cpus::get();

		if num_threads >= num_cores {
			println!("\x1b[33m\x1b[1mWarning:\x1b[0m Number of threads exceeds cores. Using all cores.\x1b[0m");
			num_threads = num_cores;
		}

		num_threads = num_threads.max(1);

		let list = self.generate_moves();

		let move_chunks = list.chunks(list.len().div_ceil(num_threads));

		let tt_enabled = bytes.is_some();

		let tt = Arc::new(match bytes {
			Some(mb) => TranspositionTable::new(mb / num_threads),
			None => TranspositionTable::default(),
		});

		let start = std::time::Instant::now();

		let handles = move_chunks
			.iter()
			.map(|chunk| {
				let mut chess_clone = self.clone();

				let chunk = chunk.clone();

				let tt_clone = tt.clone();

				thread::spawn(move || {
					let mut nodes = 0;

					for m in chunk {
						if chess_clone.play_move(m) {
							let move_nodes =
								chess_clone.perft_driver(depth - 1, &tt_clone, tt_enabled);
							chess_clone.undo_move();

							nodes += move_nodes;
							println!("{m}: {move_nodes}");
						}
					}

					nodes
				})
			})
			.collect::<Vec<_>>();

		let mut nodes = 0;

		for handle in handles {
			nodes += handle.join().unwrap()
		}

		(nodes, start.elapsed().as_millis())
	}

	#[inline(always)]
	pub fn perft_driver(
		&mut self,
		depth: u8,
		tt: &Arc<TranspositionTable<PerftData>>,
		tt_enabled: bool,
	) -> usize {
		if depth == 0 {
			return 1;
		}

		if tt_enabled {
			if let Some(data) = tt.get(&self.board.hash, &tt.guard()) {
				if data.depth() == depth {
					return data.nodes();
				}
			}
		}

		let mut nodes = 0;

		let list = self.generate_moves();

		for m in &list {
			if self.play_move(*m) {
				nodes += self.perft_driver(depth - 1, tt, tt_enabled);
				self.undo_move();
			}
		}

		if tt_enabled {
			tt.insert(self.board.hash, PerftData::new(depth, nodes));
		}

		nodes
	}
}
