#!/bin/bash -e
git submodule update --init
cd crates/witx-bindgen/WASI
git reset --hard
git pull origin main
cd ../../..

# Generate the main files
cargo run --manifest-path crates/witx-bindgen/Cargo.toml -- ./crates/witx-bindgen/WASI/phases/snapshot/witx/wasix_v1.witx > src/lib_generated32.rs
cargo run --manifest-path crates/witx-bindgen/Cargo.toml -- ./crates/witx-bindgen/WASI/phases/snapshot/witx/wasix_v1.witx 64bit > src/lib_generated64.rs

# Slight corrections
sed -i 's|clock_time_set(id.0 as i32, timestamp as i64);|clock_time_set(id.raw() as i32, timestamp as i64);|g' src/lib_generated32.rs
sed -i 's|clock_time_set(id.0 as i32, timestamp as i64);|clock_time_set(id.raw() as i32, timestamp as i64);|g' src/lib_generated64.rs

# Build it
cargo build
cargo wasix build
