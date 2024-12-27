use std::thread::{self, JoinHandle};

use crate::Chess;

impl Chess {
	#[inline(always)]
	pub fn perft(&mut self, depth: u8, mut num_threads: usize) -> usize {
		if depth == 0 {
			return 0;
		}

		let num_cores = num_cpus::get();

		if num_threads >= num_cores {
			println!("\x1b[33m\x1b[1mWarning:\x1b[0m Number of threads exceeds cores. Using all cores.\x1b[0m");
			num_threads = num_cores;
		}

		let list = self.generate_moves();

		let move_chunks = list.chunks(list.len().div_ceil(num_threads.min(list.len())));

		let handles = move_chunks
			.iter()
			.map(|chunk| {
				let mut chess_clone = self.clone();

				let chunk = chunk.clone();

				thread::spawn(move || {
					let mut nodes = 0;

					for m in chunk {
						if chess_clone.play_move(m) {
							let move_nodes = chess_clone.perft_driver(depth - 1);
							chess_clone.undo_move();

							nodes += move_nodes;
							println!("{m}: {move_nodes}");
						}
					}

					nodes
				})
			})
			.collect::<Vec<JoinHandle<usize>>>();

		let mut nodes = 0;

		for handle in handles {
			nodes += handle.join().unwrap();
		}

		nodes
	}

	#[inline(always)]
	pub fn perft_driver(&mut self, depth: u8) -> usize {
		if depth == 0 {
			return 1;
		}

		let mut nodes = 0;

		let list = self.generate_moves();

		for m in &list {
			if self.play_move(*m) {
				nodes += self.perft_driver(depth - 1);
				self.undo_move();
			}
		}

		nodes
	}
}
