all: src/ascii/names.rs
	cargo build --release

ascii_names: src/ascii/names.rs

src/ascii/names.rs: generator/src/main.rs generator/Cargo.lock generator/Cargo.toml data/ascii/nametable
	cd generator && cargo run -- ../data/ascii/nametable > ../src/ascii/names.rs
