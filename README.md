# Yes Another Chess

Yes Another Chess is a chess engine written in Rust. Despite its name, this chess engine offers unique features and enhancements over previous versions. This chess engine emphasizes type safety, ensuring that moves are generated and validated with stricter type constraints. However, this approach currently makes the engine about **4x slower** than a previous [engine](https://github.com/DestinEcarma/chess-rust) I developed. Future optimizations may involve balancing type safety with performance by allowing some raw value indexing.

## Usage

First, build the executable file or run it directly in release mode to ensure optimal performance:

```
cargo build --release
```

or

```
cargo run --release
```

Finally, you can display all available options and commands by running the program with the -h or --help argument:

```
cargo run --release -h
```

# License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE.md) file for details.
