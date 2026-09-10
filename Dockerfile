# Open Rebellion — WASM build + browser serving stack.
#
# This image only builds the browser (WASM) target and serves it with
# `python3 -m http.server`, per README.md's "Or build for browser" path.
# It does not build/run the native macroquad app, so it does not need
# ALSA/X11/GL dev headers (those are only required for `cargo run` natively).
#
# Go is pulled from the official golang image rather than Debian's apt
# package, because tools/stage-ui-assets/go.mod requires Go >= 1.22 and
# Debian bookworm's golang-go is older than that.

FROM golang:1.23-bookworm AS go-toolchain

FROM rust:1.98.1-bookworm AS runtime

COPY --from=go-toolchain /usr/local/go /usr/local/go
ENV PATH="/usr/local/go/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
        ffmpeg \
        python3 \
        binaryen \
        zip \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

WORKDIR /workspace
