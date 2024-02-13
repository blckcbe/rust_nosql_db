# Rust NoSQL Database

A simple, in-memory NoSQL database implemented in Rust, inspired by Redis. This project provides basic functionalities like setting, getting, and deleting key-value pairs through a command-line interface (CLI), along with simple data persistence to a file.

## Installation

Clone this repository to your local machine:

```
git clone https://github.com/blckcbe/rust_nosql_db.git
cd rust_nosql_db
```

Build the project with Cargo, Rust's package manager and build system:

```
cargo build --release
```

## Usage

After building, you can run the database with the following command:

```
cargo run --release
```

### Basic Commands

- **Set a Key-Value Pair**: `cargo run -- set <key> <value>`
- **Get a Value by Key**: `cargo run -- get <key>`
- **Delete a Key**: `cargo run -- delete <key>`

## Configuration

The database stores data in `db.json` in the root directory by default. In the current version, the storage file location is not configurable, but future versions may include configurable storage options.

## License

This project is licensed under the MIT License - see the LICENSE file for details.


