//! Working around old Rust, which doesn't implement IntoIter for arrays.

// Duplicated in both `allow_prefixed` and `allow_internal` crates. Not ideal, but we can't have it
// as a separate crate - because `proc_macro` types/functions are available only in crates that have
// `lib.proc-macro = true` (in `Cargo.toml`), but such crates can't export their functions other
// than proc macros themselves.
//
// Unlike the existing `wrapper_macros.rs` symlink in `allow_tests` (which links to the file with
// the same name in `allow_prefixed`), this does NOT use a symlink, so that we can compile it on
// Windows.

use proc_macro::{TokenStream, TokenTree};
use std::iter::FromIterator;

// If we ever increase the min. Rust version, or edition to 2021, we can replace this with:
// `TokenStream::from_iter(array-or-slice-here))`. The same in `allow_internal` crate.
pub fn token_streams_to_stream(tokens: &[TokenStream]) -> TokenStream {
    let mut v = Vec::with_capacity(tokens.len());
    v.extend_from_slice(tokens);
    TokenStream::from_iter(v)
}

// If we ever increase the min. Rust version, or edition to 2021, we can replace this with:
// `TokenStream::from_iter(array-or-slice-here))`. The same in `allow_internal` crate.
pub fn token_trees_to_stream(tokens: &[TokenTree]) -> TokenStream {
    let mut v = Vec::with_capacity(tokens.len());
    v.extend_from_slice(tokens);
    TokenStream::from_iter(v)
}
