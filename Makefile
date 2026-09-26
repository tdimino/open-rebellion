.PHONY: all check test test-assets fmt-check clippy build run fmt clean

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
	$(MAKE) test-assets

# These tests require original DATs: the rebellion-data unit tests marked
# ignored plus the integration suites below. Ignored doc examples
# remain ignored. Any DAT presence opts in; incomplete/corrupt data must fail.
test-assets:
ifneq ($(wildcard data/base/*.[Dd][Aa][Tt]),)
	cargo test -p rebellion-data --lib --test replay_manifest --test troop_classes --test telemetry_coverage -- --ignored
else
	@echo "Skipping asset-dependent integration tests: no DAT files in data/base."
endif
ifneq ($(wildcard data/base/ui/*),)
	cargo test -p rebellion-render --lib tactical_view -- --ignored
else
	@echo "Skipping bitmap hit-mask tests: no extracted bitmaps in data/base/ui."
endif

# 2. Formatting validation
fmt-check:
	cargo fmt --check --all

# 3. Static analysis
clippy:
	cargo clippy --workspace --all-targets -- -W clippy::pedantic -D warnings

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

# Browser harness synchronization and launch-policy regressions (no browser).
.PHONY: test-interface-harness
test-interface-harness:
	node --test tools/interface-parity/command-events.test.mjs tools/interface-parity/browser-launch.test.mjs
