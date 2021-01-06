//! Bootstrap and client code for Bazel server.
//!
//! Responsible for:
//! - extracting the Python, C++ and Java components.
//! - starting the server or finding the existing one.
//! - client options parsing.
//! - passing the argv array, and printing the out/err streams.
//! - signal handling.
//! - exiting with the right error/WTERMSIG code.
//! - debugger + profiler support.
//! - mutual exclusion between batch invocations.

pub mod rc_file;
pub mod workspace_layout;
