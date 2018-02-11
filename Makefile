ADDITIONAL_TRAVISCI_FEATURES ?= ''
ADDITIONAL_FEATURES := ${ADDITIONAL_TRAVISCI_FEATURES}

all: src/ascii/names.rs
	cargo build --release

fetch:
	./data/unicode/retrieve.sh

names: src/ascii/names.rs src/unicode/name_fst.bin

src/ascii/names.rs: generator/src/main.rs generator/src/ascii.rs generator/Cargo.lock generator/Cargo.toml data/ascii/nametable
	cd generator && cargo run -- ../data ../src

src/unicode/name_fst.bin: generator/src/unicode.rs generator/src/fst_generator.rs data/unicode/NameAliases.txt data/unicode/UnicodeData.txt
	cd generator && cargo run -- ../data ../src

install: names
	cargo install --force

test_travisci: test

test: names
	cd generator ; cargo test
	cargo test --features ${ADDITIONAL_FEATURES}

test_clippy: names
	cargo +nightly clippy

.PHONY: all names install
