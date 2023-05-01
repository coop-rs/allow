//! Working around old Rust, which doesn't implement IntoIter for arrays.

use proc_macro::{TokenStream, TokenTree};
use std::iter::FromIterator;

pub fn token_streams_to_stream(tokens: &[TokenStream]) -> TokenStream {
    let mut v = Vec::with_capacity(tokens.len());
    v.extend_from_slice(tokens);
    TokenStream::from_iter(v)
}

// - @TODO 1 Note here that if we ever increase MSRV, we can replace this with:
// `TokenStream::from_iter(array-or-slice-here))`.
// - @TODO 2 update the same comments of token_trees_to_stream in `allow_internal`, too.
//
// Duplicated in both `allow_prefixed` and `allow_internal` crates. Not ideal, but easier to manage
// than having a third crate on crates.io.
//
// Unlike the existing `wrapper_macros.rs` symlink in `allow_tests` (which links to the file with
// the same name in `allow_prefixed`), this does NOT use a symlink, so that we can compile it on
// Windows.
pub fn token_trees_to_stream(tokens: &[TokenTree]) -> TokenStream {
    let mut v = Vec::with_capacity(tokens.len());
    v.extend_from_slice(tokens);
    TokenStream::from_iter(v)
}
