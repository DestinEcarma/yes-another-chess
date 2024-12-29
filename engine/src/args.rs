use chess::{
	board::{color::Colors, piece::Pieces, pieces::PrintBitboards, Board, Color},
	move_gen::MoveGen,
	Chess,
};
use clap::{command, Parser, Subcommand, ValueEnum};
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
	#[command(subcommand)]
	pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
	/// Display the board state
	Display {
		/// The FEN string to display
		fen: Option<String>,
		/// Display individual bitboards
		#[arg(short, long)]
		bitboards: bool,
	},
	/// Does a performance test
	Perft {
		/// The depth to test
		depth: u8,
		/// The FEN string to test
		#[arg(default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
		fen: String,
		/// Use multi-threading, e.g. 4
		#[arg(short, long, default_value = "1")]
		threads: usize,
		/// Use hashing, e.g. 1024kb, 1024mb or 1gb
		#[arg(long)]
		hash: Option<String>,
	},

	#[cfg(debug_assertions)]
	/// Generate magic bitboards
	Magic {
		#[arg(value_enum)]
		piece: MagicPiece,
	},
}

#[derive(Debug, Clone, ValueEnum)]
pub enum MagicPiece {
	#[value(alias = "b", alias = "B")]
	Bishop,
	#[value(alias = "r", alias = "R")]
	Rook,
}

pub fn display(fen: Option<String>, bitboards: bool) {
	let board = match fen {
		Some(fen) => Board::from(fen.as_str()),
		None => Board::default(),
	};

	match bitboards {
		false => println!("{board}"),
		true => {
			println!("White Pieces:");
			board.pieces.print_bitboards(Color::WHITE);
			println!("Black Pieces:");
			board.pieces.print_bitboards(Color::BLACK);
		}
	}
}

pub fn perft(depth: u8, fen: String, threads: usize, hash: Option<String>) {
	let mut chess = Chess::from(fen.as_str());

	let (nodes, elapsed) = chess.perft(depth, threads, hash.and_then(to_bytes));

	let nodes_per_seconds = (nodes as f64 / (elapsed as f64 / 1000f64)).floor();

	println!("\nTotal Time (ms)\t: {elapsed}");
	println!("Nodes searched\t: {nodes}");
	println!("Nodes/second\t: {nodes_per_seconds}");
}

#[cfg(debug_assertions)]
pub fn magic(piece: MagicPiece) {
	use chess::{board::Piece, move_gen::Magic};

	let piece = match piece {
		MagicPiece::Rook => Piece::ROOK,
		MagicPiece::Bishop => Piece::BISHOP,
	};

	Magic::generate(piece);
}

fn to_bytes(size: String) -> Option<usize> {
	let size = size.trim().to_lowercase();
	let (value_str, unit) = size.split_at(size.len() - 2);

	let value = match value_str.parse::<usize>() {
		Ok(value) => value,
		Err(_) => return None,
	};

	if value < 1 {
		println!("Warning: Invalid size. No hashing will be used. Use a positive integer.");
		return None;
	}

	match unit {
		"kb" => Some(value * 1024),
		"mb" => Some(value * 1024 * 1024),
		"gb" => Some(value * 1024 * 1024 * 1024),

		_ => {
			println!("Warning: Invalid unit. No hashing will be used. Use 'kb', 'mb' or 'gb'.");
			None
		}
	}
}
