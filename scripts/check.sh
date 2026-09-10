#!/bin/sh
set -eu

if ! command -v cargo >/dev/null 2>&1 && [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

cd "$(dirname "$0")/.."
cargo test --offline --locked
cargo run --offline --locked --example observations
