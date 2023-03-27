//#![feature(proc_macro_quote)]

//use std::str::FromStr;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn prefix_attrs_before_item(attrs: TokenStream, item: TokenStream) -> TokenStream {
    //assert!(attrs.is_empty());
    let streams = [attrs, item];
    TokenStream::from_iter(streams)
}
// ------

fn hash() -> TokenStream {
    TokenStream::from(TokenTree::Punct(Punct::new('#', Spacing::Alone)))
}

fn allow() -> TokenTree {
    TokenTree::Ident(Ident::new("allow", Span::call_site()))
}

thread_local! {
    static HASH: TokenStream = hash();
    static ALLOW: TokenTree = allow();
}

// https://github.com/rust-lang/rust/issues/29594
// cfg(target_thread_local)

// ----
//
// Actual macros for public use
#[proc_macro_attribute]
pub fn unused(given_attrs: TokenStream, item: TokenStream) -> TokenStream {
    assert!(given_attrs.is_empty());
    if false {
        return item;
    }
    // The string source for `attrs` is  internal, hence well formed.
    //let attrs = TokenStream::from_str("#[allow(unused)]").unwrap();

    let hash_local = TokenStream::from(TokenTree::Punct(Punct::new('#', Spacing::Alone)));

    let allow_local = TokenTree::Ident(Ident::new("allow", Span::call_site()));

    // BEWARE: If the lint name doesn't exist, then even if the consumer code uses
    // `#[deny(unknown_lints)]`, that (incorrect lint name) will NOT get reported.
    let unused_as_stream =
        TokenStream::from(TokenTree::Ident(Ident::new("unused", Span::call_site())));
    let parented_unused = TokenTree::Group(Group::new(Delimiter::Parenthesis, unused_as_stream));

    //let allow_unused = ALLOW.with(|allow| TokenStream::from_iter([allow.clone(), parented_unused]));
    let allow_unused = TokenStream::from_iter([allow_local, parented_unused]);

    let squared = TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Bracket,
        allow_unused,
    )));

    HASH.with(|hash| TokenStream::from_iter([hash.clone(), squared, item]))
    //TokenStream::from_iter([hash_local, squared, item])
}
