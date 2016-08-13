all: src/ascii/names.rs
	cargo build --release

names: src/ascii/names.rs src/unicode/name_fst.bin

src/ascii/names.rs: generator/src/main.rs generator/src/ascii.rs generator/Cargo.lock generator/Cargo.toml data/ascii/nametable
	cd generator && cargo run -- ../data ../src

src/unicode/name_fst.bin: generator/src/unicode.rs generator/src/fst_generator.rs data/unicode/NameAliases.txt data/unicode/UnicodeData.txt
	cd generator && cargo run -- ../data ../src

install: names
	cargo install --force

.PHONY: all names install
