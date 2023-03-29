#![feature(thread_local, local_key_cell_methods, option_get_or_insert_default)]
#![deny(unknown_lints)]

use allows_internals::{generate_allows_attribute_macro_definition, token_stream_to_str_literal};
use once_cell::unsync::OnceCell;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::cell::RefCell;
use std::collections::HashMap;

const USE_ATTRIB: bool = true;

type LintsMap = HashMap<&'static str, TokenStream>;

// `#[thread_local] static ONE: TokenStream = TokenStream::from(...)`
//
// failed, and rustc was suggesting `Lazy::new(...)`. However, that would involve a Mutex.
//
// See https://docs.rs/once_cell/latest/once_cell/unsync/struct.OnceCell.html
#[thread_local]
static HASH_ATTRIB: OnceCell<TokenStream> = OnceCell::new();
#[thread_local]
static ALLOW_ATTRIB: OnceCell<TokenTree> = OnceCell::new();
#[thread_local]
static LINTS_ATTRIB: RefCell<Option<LintsMap>> = RefCell::new(None);

thread_local! {
    // Read-only.
    static HASH_MACROED: TokenStream = generate_hash();
    static ALLOW_MACROED: TokenTree = generate_allow();

    /// A map: lint name => token stream for: [allow(...)]. Read-write.
    static LINTS_MACROED: RefCell<LintsMap> = RefCell::new(LintsMap::new());
}

// Get the relevant parts from thread local storage (if enabled, and if stored already). Generate
// otherwise (and also put in thread local storage, if enabled). Then `.clone()` and return.
fn get_hash() -> TokenStream {
    if USE_ATTRIB {
        HASH_ATTRIB.get_or_init(generate_hash).clone()
    } else {
        HASH_MACROED.with(Clone::clone)
    }
}

fn get_allow() -> TokenTree {
    if USE_ATTRIB {
        ALLOW_ATTRIB.get_or_init(generate_allow).clone()
    } else {
        ALLOW_MACROED.with(Clone::clone)
    }
}

//-----
//
// Generate the parts (but re-using other parts if stored in thread local already).
//
// Independent of thread local storage.

/// Generate [`TokenStream`] consisting of one hash character: `#`. It serves as the leading character of the injected code (just left of the injected "[allow(...)]").
fn generate_hash() -> TokenStream {
    TokenStream::from(TokenTree::Punct(Punct::new('#', Spacing::Alone)))
}

/// Generate [`TokenTree`] consisting of one identifier: `allow`.
fn generate_allow() -> TokenTree {
    TokenTree::Ident(Ident::new("allow", Span::call_site()))
}
// -----

fn with_lints_squared<F>(f: F) -> TokenStream
where
    F: FnOnce(&mut HashMap<&'static str, TokenStream>) -> TokenStream,
{
    if USE_ATTRIB {
        let mut lints = LINTS_ATTRIB.borrow_mut();
        f(lints.get_or_insert_default())
    } else {
        LINTS_MACROED.with_borrow_mut(|lints| f(lints))
    }
}

fn brackets_allow_lint(lint_path: &'static str) -> TokenStream {
    with_lints_squared(|lints| {
        let entry = lints.entry(lint_path);
        entry
            .or_insert_with(|| {
                // TODO: NOT an Ident, but split!
                let lint_name =
                    TokenStream::from(TokenTree::Ident(Ident::new(lint_path, Span::call_site())));
                let parens_lint_name =
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, lint_name));

                let allow_lint_name = TokenStream::from_iter([get_allow(), parens_lint_name]);

                TokenStream::from(TokenTree::Group(Group::new(
                    Delimiter::Bracket,
                    allow_lint_name,
                )))
            })
            .clone()
    })
}

// cfg(target_thread_local)

/// NOT for public use. "Used" only by
/// [`allows_internals::generate_allows_attribute_macro_definition`] macro.
/// [`allows_internals::generate_allows_attribute_macro_definition`] doesn't invoke this, but it
/// generates code that invokes it.
///
/// Define a `proc` macro to allow the given lint. The proc macro will have the same name as the
/// given `lint_name`, except that any package-like separators (pairs of colons) :: are replaced
/// with an underscore _.
///
/// BEWARE: If the lint name doesn't exist, then even if the consumer code uses
/// `#[deny(unknown_lints)]`, if we generate its usage from our macro, that (incorrect lint name)
/// would NOT get reported. That's why _check_the_lint_is_valid() below.
macro_rules! generate_allows_attribute_macro_definition_internal {
    ( $lint_path:path, $new_macro_name:ident ) => {
        #[proc_macro_attribute]
        pub fn $new_macro_name(
            given_attrs: ::proc_macro::TokenStream,
            item: ::proc_macro::TokenStream,
        ) -> ::proc_macro::TokenStream {
            // The following is why we have #![deny(unknown_lints)] for this file.
            #[cfg(test)]
            #[allow($lint_path)]
            fn _check_the_lint_is_valid() {}

            assert!(
                given_attrs.is_empty(),
                "Do not pass any attribute parameters."
            );
            ::proc_macro::TokenStream::from_iter([
                $crate::get_hash(),
                $crate::brackets_allow_lint(allows_internals::token_stream_to_str_literal!(
                    $lint_path
                )),
                item,
            ])
        }
    };
}
// ----

// @TODO
macro_rules! standard_lints {
    () => {};
}

macro_rules! prefixed_lints {
    () => {};
}

//-----

// EXAMPLE Actual macros for public use
#[proc_macro_attribute]
pub fn unused(_given_attrs: TokenStream, item: TokenStream) -> TokenStream {
    // The string source for `attrs` is  internal, hence well formed.
    //let attrs = TokenStream::from_str("#[allow(unused)]").unwrap();
    TokenStream::from_iter([get_hash(), brackets_allow_lint("unused"), item])
}

//#[allow(unused_braces)]
generate_allows_attribute_macro_definition!(clippy::almost_swapped);
generate_allows_attribute_macro_definition!(unused_braces);
