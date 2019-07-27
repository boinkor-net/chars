ADDITIONAL_TRAVISCI_FEATURES ?= ''
ADDITIONAL_FEATURES := ${ADDITIONAL_TRAVISCI_FEATURES}

all:
	cargo build --release

fetch:
	./chars_data/data/unicode/retrieve.sh

install:
	cargo install --force --path chars/

test_travisci: test

test:
	cd chars/generator ; cargo test
	cargo test --features ${ADDITIONAL_FEATURES}

.PHONY: all names install
