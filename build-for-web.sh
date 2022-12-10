#!/bin/sh
set -e

# one-time setup:
# rustup target add wasm32-unknown-unknown
# cargo install -f wasm-bindgen-cli

# the "--no-default-features" setting is here to disable dynamic linking, which isn't supported for wasm
# see the [features] section of Cargo.toml for more detail

# for PROFILE in web-dev web-release; do
for PROFILE in web-dev; do
    cargo build --profile $PROFILE --no-default-features --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir ./target/web/$PROFILE ./target/wasm32-unknown-unknown/$PROFILE/scratch-bevy.wasm
done
