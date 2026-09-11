.PHONY: fmt-check clippy test run build fmt clean check all

all: check build

check: test fmt-check clippy

clean:
	cargo clean

fmt-check:
	cargo fmt --check --all

clippy:
	cargo clippy --workspace --all-targets -- -W clippy::pedantic -D warnings

test:
	cargo test --workspace

run:
	cargo run -p rebellion-app -- data/base

build:
	cargo build -p rebellion-app

fmt:
	cargo fmt --all
