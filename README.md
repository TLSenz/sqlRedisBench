# sqlRedisBench

A simple Terminal User Interface (TUI) tool to benchmark and compare performance between SQLite (on-disk and in-memory) and Redis.

## Features

- Benchmark SQLite on-disk setup and operations.
- Benchmark SQLite in-memory setup and operations.
- Benchmark Redis connection and operations.
- Real-time results display using `ratatui`.
- Interactive TUI: press `r` to run benchmarks, `q` to quit.

## Installation

```bash
cargo install sqlRedisBench
```

## Requirements

- **Redis**: For Redis benchmarks, a Redis server should be running on `localhost:6379`.
- **SQLite**: No additional setup required as it uses the `rusqlite` crate.

## Usage

If you have cloned the repository, you can run it directly:

```bash
cargo run
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
