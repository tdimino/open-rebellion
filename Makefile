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

# Go asset extractor
.PHONY: test-go fmt-go vet-go
test-go:
	go test ./tools/stage-ui-assets
fmt-go:
	gofmt -w tools/stage-ui-assets/*.go
vet-go:
	go vet ./tools/stage-ui-assets

GAME_SOURCE ?= data/base
MDATA_DIR ?= $(GAME_SOURCE)/MDATA
.PHONY: extract-assets
extract-assets:
	go run ./tools/stage-ui-assets --source "$(GAME_SOURCE)" --mdata "$(MDATA_DIR)"

.PHONY: verify-assets
verify-assets:
	go run ./tools/stage-ui-assets --verify
