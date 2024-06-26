# Yes Another Chess

Yes Another Chess is a chess engine written in Rust. Despite its name, this chess engine offers unique features and enhancements over its predecessor, [chess rust](https://github.com/DestinEcarma/chess-rust). The engine is heavily derived from [rustic](https://github.com/mvanthoor/rustic). Currently, the engine does not support hashing, but that will be my next project.

## Usage

First, build the executable file or run it directly in release mode for optimal performance:

```sh
cargo build --release
```

or

```sh
cargo run --release
```

Finally, you can display all available options and commands by running the program with the `-h` or `--help` argument:

```sh
cargo run --release -- -h
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
