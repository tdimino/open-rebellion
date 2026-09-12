.PHONY: all check test fmt-check clippy build run fmt clean

# Full workflow: validate first, then build. Stop on the first failure.
all: check
	$(MAKE) build

# Run checks in order, even when invoked with make -j.
check:
	$(MAKE) test
	$(MAKE) fmt-check
	$(MAKE) clippy

# 1. Tests
test:
	cargo test --workspace

# 2. Formatting validation
fmt-check:
	cargo fmt --check --all

# 3. Static analysis
clippy:
	cargo clippy --workspace --all-targets -- -D warnings

# 4. Build and run
build:
	cargo build -p rebellion-app

run:
	cargo run -p rebellion-app -- data/base

# Maintenance
fmt:
	cargo fmt --all

clean:
	cargo clean
