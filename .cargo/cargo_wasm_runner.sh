#!/bin/sh
# This script is invoked by cargo as the test runner for wasm32-wasip1.
# Cargo resolves this path relative to .cargo/config.toml's parent directory.
exec node "$(cd "$(dirname "$0")/.." && pwd)/tools/wasm_runner/wasm_runner.ts" "$@"
