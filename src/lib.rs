#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces,
    clippy::all
)]
// `error_chain`'s `Error` is 152 bytes; boxing it would change the public `Result` type.
#![allow(clippy::needless_doctest_main, clippy::result_large_err)]
#![warn(
    clippy::wildcard_imports,
    clippy::manual_string_new,
    clippy::single_match_else,
    clippy::implicit_clone,
    clippy::semicolon_if_nothing_returned
)]

mod client;
pub mod errors;
pub mod util;

pub mod model;

pub mod account;
pub mod api;
pub mod config;
pub mod general;
pub mod market;
pub mod savings;
pub mod userstream;
pub mod websockets;

pub mod futures;
