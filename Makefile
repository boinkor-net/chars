ADDITIONAL_TRAVISCI_FEATURES ?= ''
ADDITIONAL_FEATURES := ${ADDITIONAL_TRAVISCI_FEATURES}

all:
	cargo build --release

fetch:
	./data/unicode/retrieve.sh

install:
	cargo install --force

test_travisci: test

test:
	cd generator ; cargo test
	cargo test --features ${ADDITIONAL_FEATURES}

test_clippy: names
	cargo +nightly clippy

.PHONY: all names install
