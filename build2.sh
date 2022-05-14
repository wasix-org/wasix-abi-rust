#!/bin/bash -e
cargo run -p witx-bindgen -- ../wasi-witx/phases/snapshot/witx/wasix_snapshot_preview1.witx > src/lib_generated.rs
