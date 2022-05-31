#!/bin/bash -e
cargo run -p witx-bindgen -- ../wasix-witx/phases/snapshot/witx/wasix_v1.witx > src/lib_generated32.rs
cargo run -p witx-bindgen -- ../wasix-witx/phases/snapshot/witx/wasix_v1.witx 64bit > src/lib_generated64.rs
