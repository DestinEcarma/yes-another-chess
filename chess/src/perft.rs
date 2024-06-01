use crate::Chess;

impl Chess {
	#[inline(always)]
	pub fn perft(&mut self, depth: u8) -> usize {
		if depth == 0 {
			return 0;
		}

		let mut nodes = 0;

		let moves = self.generate_moves();

		for m in moves {
			if self.play_move(m) {
				let move_nodes = self.perft_driver(depth - 1);
				self.undo_move();

				nodes += move_nodes;
				println!("{m}: {move_nodes}");
			}
		}

		nodes
	}

	#[inline(always)]
	pub fn perft_driver(&mut self, depth: u8) -> usize {
		if depth == 0 {
			return 1;
		}

		let mut nodes = 0;

		let moves = self.generate_moves();

		for m in moves {
			if self.play_move(m) {
				nodes += self.perft_driver(depth - 1);
				self.undo_move();
			}
		}

		nodes
	}
}
