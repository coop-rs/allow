//! Working around old Rust, which doesn't implement IntoIter for arrays.

use proc_macro::{TokenStream, TokenTree};
use std::iter::FromIterator;

// Duplicated in both `allow_prefixed` and `allow_internal` crates. Not ideal, but easier to manage than
// having a third crate on crates.io.
//
// Unlike `wrapper_macros.rs` symlink in `allow_tests`, this does NOT use a symlink, so it does
// compile on Windows.
pub fn token_trees_to_stream(tokens: &[TokenTree]) -> TokenStream {
    let mut v = Vec::with_capacity(tokens.len());
    v.extend_from_slice(tokens);
    TokenStream::from_iter(v)
}
