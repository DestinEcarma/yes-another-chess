#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

mod args;

use args::{Args, Command};
use clap::{CommandFactory, Parser};

fn main() {
	let args = Args::parse();

	match args.command {
		None => Args::command().print_help().unwrap(),
		Some(Command::Display { fen, bitboards }) => args::display(fen, bitboards),
		Some(Command::Perft {
			depth,
			fen,
			threads,
			hash,
		}) => args::perft(depth, fen, threads, hash),
		#[cfg(debug_assertions)]
		Some(Command::Magic { piece }) => args::magic(piece),
	}
}
