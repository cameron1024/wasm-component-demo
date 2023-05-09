#!/usr/bin/env bash
set -e

cargo build --target wasm32-wasi --release --manifest-path component/Cargo.toml

wasm-tools component new ./component/target/wasm32-wasi/release/component.wasm -o component.wasm --adapt ./wasi_snapshot_preview1.wasm

echo "wasm binary written to ./component.wasm"
