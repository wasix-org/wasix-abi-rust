<div align="center">
  <h1><code>WASI(X)</code></h1>

<strong>WASI is a <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong><br />
<strong>WASI(X) adds extensions and is managed by the <a href="https://wasmer.io/">Wasmer Community</a></strong>

  <p>
    <strong>WASI API Bindings for Rust</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wasi"><img src="https://img.shields.io/crates/v/wasi.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wasi"><img src="https://img.shields.io/crates/d/wasi.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/wasi/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <p>
    <strong>WASI(X) API Bindings for Rust</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/wasix"><img src="https://img.shields.io/crates/v/wasix.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wasix"><img src="https://img.shields.io/crates/d/wasix.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/wasix/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

This crate contains API bindings for [WASI](https://github.com/WebAssembly/WASI)
system calls in Rust, and currently reflects the `wasi_snapshot_preview1` namespace

[WASIX](https://github.com/john-sharratt/wasix) adds extensions to [WASI](https://github.com/WebAssembly/WASI)
as a superset., and currently reflects the `wasix_32v1` namespace

# What is WASIX?

WASIX is the long term stabilization and support of the existing WASI ABI *plus* additional non-invasive syscall extensions that complete the missing gaps sufficiently enough to enable real, practical and useful applications to be compiled now. It aims to speed up the ecosystem around the WASI so that the WASM’ification of code bases around the world can really start *today*.

# Additional extensions

```rust
// Its now possible to duplicate file handles
pub use x::fd_dup;

// Events are used by polling functions that can be interrupted such
// as `tokio` and `mio`
pub use x::fd_event;

// Pipes are required to stream data to and from subprocesses
pub use x::fd_pipe;

// Yields CPU time without the bloat of `poll_oneoff`
pub use x::sched_yield;

// Getting and setting the TTY properties
pub use x::tty_get;
pub use x::tty_set;

// Changing the current directory is now natively supported
pub use x::getcwd;
pub use x::chdir;

// Signals can be blocked (needed by `libc`)
pub use x::callback_signal;

// Spawning threads as per the experimental threads spec
pub use x::thread_spawn;

// Extra thread related functions
pub use x::thread_sleep;
pub use x::thread_id;
pub use x::thread_join;
pub use x::thread_parallelism;
pub use x::thread_signal;
pub use x::thread_exit;

// Operating system futex support used for multithread constructs
pub use x::futex_wait;
pub use x::futex_wake;
pub use x::futex_wake_all;

// Longjmp and setjmp used by `libc`
pub use x::stack_checkpoint;
pub use x::stack_restore;

// Subprocess support
pub use x::proc_raise_interval;
pub use x::proc_fork;
pub use x::proc_exec;
pub use x::proc_spawn;
pub use x::proc_id;
pub use x::proc_parent;
pub use x::proc_join;
pub use x::proc_signal;

// Interface support
pub use x::port_bridge;
pub use x::port_unbridge;
pub use x::port_dhcp_acquire;
pub use x::port_addr_add;
pub use x::port_addr_remove;
pub use x::port_addr_clear;
pub use x::port_mac;
pub use x::port_addr_list;
pub use x::port_gateway_set;
pub use x::port_route_add;
pub use x::port_route_remove;
pub use x::port_route_clear;
pub use x::port_route_list;

// All the missing socket functionality
pub use x::sock_status;
pub use x::sock_addr_local;
pub use x::sock_addr_peer;
pub use x::sock_set_opt_flag;
pub use x::sock_get_opt_flag;
pub use x::sock_set_opt_time;
pub use x::sock_get_opt_time;
pub use x::sock_set_opt_size;
pub use x::sock_get_opt_size;
pub use x::sock_join_multicast_v4;
pub use x::sock_leave_multicast_v4;
pub use x::sock_join_multicast_v6;
pub use x::sock_leave_multicast_v6;
pub use x::sock_bind;
pub use x::sock_listen;
pub use x::sock_connect;
pub use x::sock_recv_from;
pub use x::sock_send_to;
pub use x::sock_send_file;

// Ability to perform DNS queries
pub use x::resolve;
```

# Usage

First you can depend on this crate via `Cargo.toml`:

```toml
[dependencies]
wasix = "0.11"
```

Next you can use the APIs in the root of the module like so:

```rust
fn main() {
    let stdout = 1;
    let message = "Hello, World!\n";
    let data = [wasix::Ciovec {
        buf: message.as_ptr(),
        buf_len: message.len(),
    }];
    wasix::fd_write(stdout, &data).unwrap();
}
```

Next you can use a tool like [`cargo
wasix`](https://github.com/wasmerio/cargo-wasix) to compile and run your
project:

To compile Rust projects to wasm using WASI, use the `wasm32-wasix` target,
like this:

```
$ wasmer run my-all
   Compiling wasix v0.11.0
   Compiling wasi v0.11.0+wasix-snapshot-preview1
   Compiling wut v0.1.0 (/code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `/.cargo/bin/cargo-wasix target/wasm64-wasix/debug/wut.wasm`
     Running `target/wasm64-wasix/debug/wut.wasm`
Hello, World!
```

# Development

The bulk of the `wasix` crate is generated by the `witx-bindgen` tool, which lives at
`crates/witx-bindgen` and is part of the cargo workspace.

The `src/lib_generated.rs` file can be re-generated with the following
command:

```
cargo run -p witx-bindgen -- crates/witx-bindgen/WASI/phases/snapshot/witx/wasix_v1.witx > src/lib_generated.rs
```

Note that this uses the WASIX standard repository as a submodule. If you do not
have this submodule present in your source tree, run:
```
git submodule update --init
```

# License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
