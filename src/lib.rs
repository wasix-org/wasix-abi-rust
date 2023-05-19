//! Raw API bindings to the WebAssembly System Interface (WASI)
//!
//! This crate provides Rust API bindings to WASI APIs. All WASI APIs are
//! exported from this crate and provided with the appropriate type signatures.
//! This crate is entirely procedurally generated from the `*.witx` files that
//! describe the WASI API.
//!
//! # WASI API Version
//!
//! The WASI API is evolving over time. It is both gaining new features as well
//! as tweaking the ABI of existing features. As a result it's important to
//! understand what version of this crate you're using and how it relates to
//! the WASI version of the spec.
//!
//! The WASI specification is organized into phases where there is a snapshot
//! at any one point in time describing the current state of the specification.
//! This crate implements a particular snapshot. You can find the snapshot
//! version implemented in this crate in the build metadata of the crate
//! version number. For example something like `0.9.0+wasi-snapshot-preview1`
//! means that this crate's own personal version is 0.9.0 and it implements the
//! `wasi-snapshot-preview1` snapshot. A major release of this crate (i.e.
//! bumping the "0.9.0") is expected whenever the generated code changes
//! or a new WASI snapshot is used.
//!
//! # Crate Features
//!
//! This crate supports one feature, `std`, which implements the standard
//! `Error` trait for the exported [`Error`] type in this crate. This is
//! enabled by default but can be disabled to make the library `no_std`
//! compatible.

#![no_std]

#[cfg(target_pointer_width = "32")]
pub mod lib_generated32;
#[cfg(target_pointer_width = "64")]
pub mod lib_generated64;

#[cfg(target_pointer_width = "32")]
pub use lib_generated32 as lib_generated;
#[cfg(target_pointer_width = "64")]
pub use lib_generated64 as lib_generated;

pub use lib_generated as x;

// Re-export
pub use wasi;

// We add the sub-set of WASI
#[cfg(target_pointer_width = "32")]
pub use wasi::*;

// WASI does not yet support 64 bit so we take it all from WASIX
#[cfg(target_pointer_width = "64")]
pub use x::*;

// Additional types needed for more advanced use-cases
pub use x::Pointersize;
pub use x::Longsize;
pub use x::TlKey;
pub use x::TlVal;
pub use x::ShortHash;
pub use x::Hugesize;
pub use x::Hash;
pub use x::StackSnapshot;
pub use x::Bool;
pub use x::BOOL_FALSE;
pub use x::BOOL_TRUE;

// Additional error codes
pub use x::ERRNO_SHUTDOWN;
pub use x::ERRNO_OVERFLOW;
pub use x::ERRNO_MEMVIOLATION;
pub use x::ERRNO_UNKNOWN;

// Additional permission rights
pub use x::RIGHTS_SOCK_SHUTDOWN;
pub use x::RIGHTS_SOCK_ACCEPT;
pub use x::RIGHTS_SOCK_CONNECT;
pub use x::RIGHTS_SOCK_LISTEN;
pub use x::RIGHTS_SOCK_BIND;
pub use x::RIGHTS_SOCK_RECV;
pub use x::RIGHTS_SOCK_SEND;
pub use x::RIGHTS_SOCK_ADDR_LOCAL;
pub use x::RIGHTS_SOCK_ADDR_REMOTE;
pub use x::RIGHTS_SOCK_RECV_FROM;
pub use x::RIGHTS_SOCK_SEND_TO;

// Optional
pub use x::Option;
pub use x::OPTION_NONE;
pub use x::OPTION_SOME;
pub use x::OptionTimestampU;
pub use x::OptionTimestamp;
pub use x::OptionHashU;
pub use x::OptionHash;
pub use x::OptionPidU;
pub use x::OptionPid;
pub use x::OptionFdU;
pub use x::OptionFd;

// Other handles
pub use x::Pid;
pub use x::Tid;

// Additional file types
pub use x::FILETYPE_SOCKET_DGRAM;
pub use x::FILETYPE_SOCKET_STREAM;
pub use x::FILETYPE_SYMBOLIC_LINK;
pub use x::FILETYPE_SOCKET_RAW;
pub use x::FILETYPE_SOCKET_SEQPACKET;

// Signals are now supported
pub use x::Signal;
pub use x::SIGNAL_NONE;
pub use x::SIGNAL_HUP;
pub use x::SIGNAL_INT;
pub use x::SIGNAL_QUIT;
pub use x::SIGNAL_ILL;
pub use x::SIGNAL_TRAP;
pub use x::SIGNAL_ABRT;
pub use x::SIGNAL_BUS;
pub use x::SIGNAL_FPE;
pub use x::SIGNAL_KILL;
pub use x::SIGNAL_USR1;
pub use x::SIGNAL_SEGV;
pub use x::SIGNAL_USR2;
pub use x::SIGNAL_PIPE;
pub use x::SIGNAL_ALRM;
pub use x::SIGNAL_TERM;
pub use x::SIGNAL_CHLD;
pub use x::SIGNAL_CONT;
pub use x::SIGNAL_STOP;
pub use x::SIGNAL_TSTP;
pub use x::SIGNAL_TTIN;
pub use x::SIGNAL_TTOU;
pub use x::SIGNAL_URG;
pub use x::SIGNAL_XCPU;
pub use x::SIGNAL_XFSZ;
pub use x::SIGNAL_VTALRM;
pub use x::SIGNAL_PROF;
pub use x::SIGNAL_WINCH;
pub use x::SIGNAL_POLL;
pub use x::SIGNAL_PWR;
pub use x::SIGNAL_SYS;

// Add all the socket types
pub use x::SockType;
pub use x::SOCK_TYPE_SOCKET_DGRAM;
pub use x::SOCK_TYPE_SOCKET_STREAM;
pub use x::SOCK_TYPE_SOCKET_RAW;
pub use x::SOCK_TYPE_SOCKET_SEQPACKET;

// Add all the protocol types
pub use x::SockProto;
pub use x::SOCK_PROTO_IP;
pub use x::SOCK_PROTO_ICMP;
pub use x::SOCK_PROTO_IGMP;
pub use x::SOCK_PROTO_IPIP;
pub use x::SOCK_PROTO_TCP;
pub use x::SOCK_PROTO_EGP;
pub use x::SOCK_PROTO_PUP;
pub use x::SOCK_PROTO_UDP;
pub use x::SOCK_PROTO_IDP;
pub use x::SOCK_PROTO_DCCP;
pub use x::SOCK_PROTO_IPV6;
pub use x::SOCK_PROTO_ROUTING;
pub use x::SOCK_PROTO_FRAGMENT;
pub use x::SOCK_PROTO_RSVP;
pub use x::SOCK_PROTO_GRE;
pub use x::SOCK_PROTO_ESP;
pub use x::SOCK_PROTO_AH;
pub use x::SOCK_PROTO_ICMPV6;
pub use x::SOCK_PROTO_NONE;
pub use x::SOCK_PROTO_DSTOPTS;
pub use x::SOCK_PROTO_MTP;
pub use x::SOCK_PROTO_BEETPH;
pub use x::SOCK_PROTO_ENCAP;
pub use x::SOCK_PROTO_PIM;
pub use x::SOCK_PROTO_COMP;
pub use x::SOCK_PROTO_SCTP;
pub use x::SOCK_PROTO_MH;
pub use x::SOCK_PROTO_UDPLITE;
pub use x::SOCK_PROTO_MPLS;
pub use x::SOCK_PROTO_ETHERNET;
pub use x::SOCK_PROTO_MPTCP;
pub use x::SOCK_PROTO_MAX;

/// Sockets can now have a status
pub use x::SockStatus;
pub use x::SOCK_STATUS_OPENING;
pub use x::SOCK_STATUS_OPENED;
pub use x::SOCK_STATUS_CLOSED;
pub use x::SOCK_STATUS_FAILED;

/// Options can be passed to the sockets
pub use x::SockOption;
pub use x::SOCK_OPTION_NOOP;
pub use x::SOCK_OPTION_REUSE_PORT;
pub use x::SOCK_OPTION_REUSE_ADDR;
pub use x::SOCK_OPTION_NO_DELAY;
pub use x::SOCK_OPTION_DONT_ROUTE;
pub use x::SOCK_OPTION_ONLY_V6;
pub use x::SOCK_OPTION_BROADCAST;
pub use x::SOCK_OPTION_MULTICAST_LOOP_V4;
pub use x::SOCK_OPTION_MULTICAST_LOOP_V6;
pub use x::SOCK_OPTION_PROMISCUOUS;
pub use x::SOCK_OPTION_LISTENING;
pub use x::SOCK_OPTION_LAST_ERROR;
pub use x::SOCK_OPTION_KEEP_ALIVE;
pub use x::SOCK_OPTION_LINGER;
pub use x::SOCK_OPTION_OOB_INLINE;
pub use x::SOCK_OPTION_RECV_BUF_SIZE;
pub use x::SOCK_OPTION_SEND_BUF_SIZE;
pub use x::SOCK_OPTION_RECV_LOWAT;
pub use x::SOCK_OPTION_SEND_LOWAT;
pub use x::SOCK_OPTION_RECV_TIMEOUT;
pub use x::SOCK_OPTION_SEND_TIMEOUT;
pub use x::SOCK_OPTION_CONNECT_TIMEOUT;
pub use x::SOCK_OPTION_ACCEPT_TIMEOUT;
pub use x::SOCK_OPTION_TTL;
pub use x::SOCK_OPTION_MULTICAST_TTL_V4;
pub use x::SOCK_OPTION_TYPE;
pub use x::SOCK_OPTION_PROTO;

/// Security associated with a stream
pub use x::StreamSecurity;
pub use x::STREAM_SECURITY_UNENCRYPTED;
pub use x::STREAM_SECURITY_ANY_ENCRYPTION;
pub use x::STREAM_SECURITY_CLASSIC_ENCRYPTION;
pub use x::STREAM_SECURITY_DOUBLE_ENCRYPTION;

/// Other socket types
pub use x::HardwareAddress;
pub use x::AddressFamily;
pub use x::ADDRESS_FAMILY_UNSPEC;
pub use x::ADDRESS_FAMILY_INET4;
pub use x::ADDRESS_FAMILY_INET6;
pub use x::ADDRESS_FAMILY_UNIX;
pub use x::AddrUnspec;
pub use x::AddrUnspecPort;
pub use x::AddrUnspecCidr;
pub use x::AddrIp4;
pub use x::AddrIp4Port;
pub use x::AddrIp4Cidr;
pub use x::AddrIp6;
pub use x::AddrUnix;
pub use x::AddrUnixPort;
pub use x::AddrUnixCidr;
pub use x::AddrIp6Port;
pub use x::AddrIp6Cidr;
pub use x::AddrU;
pub use x::Addr;
pub use x::AddrPortU;
pub use x::AddrPort;
pub use x::AddrPortArray;
pub use x::AddrCidrU;
pub use x::AddrCidr;
pub use x::Route;
pub use x::HttpHandles;
pub use x::HttpStatus;

// The STDIO types are used for launching subprocesses
pub use x::StdioMode;
pub use x::STDIO_MODE_PIPED;
pub use x::STDIO_MODE_INHERIT;
pub use x::STDIO_MODE_NULL;
pub use x::STDIO_MODE_LOG;

// When joining on process there are some flags we pass
pub use x::JoinFlags;
pub use x::JOIN_FLAGS_NON_BLOCKING;
pub use x::JOIN_FLAGS_WAKE_STOPPED;
pub use x::JoinStatusType;
pub use x::JOIN_STATUS_TYPE_NOTHING;
pub use x::JOIN_STATUS_TYPE_EXIT_NORMAL;
pub use x::JOIN_STATUS_TYPE_EXIT_SIGNAL;
pub use x::JOIN_STATUS_TYPE_STOPPED;
pub use x::JoinStatusU;
pub use x::JoinStatus;

// Additional structs used by threads
pub use x::ThreadFlags;
pub use x::THREAD_FLAGS_TSD_USED;
pub use x::THREAD_FLAGS_DLERROR_FLAG;
pub use x::ThreadState;
pub use x::ThreadStart;

// The errno signal type is used to strongly type an error code
// while retaining the ability to return arbitrary numbers
pub use x::ErrnoSignal;

// Add the TTY objects
pub use x::Tty;

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
pub use x::sock_open;
pub use x::sock_recv_from;
pub use x::sock_send_to;
pub use x::sock_send_file;

// Ability to perform DNS queries
pub use x::resolve;

// Allows WASIX types to interoperate with WASI types
mod conv;
pub use conv::*;

/// Special `Dircookie` value indicating the start of a directory.
pub const DIRCOOKIE_START: Dircookie = 0;

/// The "standard input" descriptor number.
pub const FD_STDIN: Fd = 0;

/// The "standard output" descriptor number.
pub const FD_STDOUT: Fd = 1;

/// The "standard error" descriptor number.
pub const FD_STDERR: Fd = 2;
