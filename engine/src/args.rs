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
		/// The FEN string to test
		depth: u8,
		/// The depth to test
		fen: Option<String>,
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

pub fn perft(depth: u8, fen: Option<String>) {
	let mut chess = match fen {
		Some(fen) => Chess::from(fen.as_str()),
		None => Chess::default(),
	};

	let start = Instant::now();
	let nodes = chess.perft(depth);
	let elapsed = start.elapsed().as_millis();

	let nodes_per_seconds = match nodes.checked_div(match elapsed.checked_div(1000) {
		Some(num) => num as usize,
		None => 1,
	}) {
		Some(nodes) => nodes,
		None => nodes,
	};

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
