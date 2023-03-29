#![feature(thread_local, local_key_cell_methods, option_get_or_insert_default)]

use allows_internals::{generate_allows_attribute_macro_definition, path_to_ident_mac};
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

fn lint_squared(given_lint_name: &'static str) -> TokenStream {
    with_lints_squared(|lints| {
        let entry = lints.entry(given_lint_name);
        entry
            .or_insert_with(|| {
                let lint_name = TokenStream::from(TokenTree::Ident(Ident::new(
                    given_lint_name,
                    Span::call_site(),
                )));
                let parented_lint_name =
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, lint_name));

                let allow_lint_name = TokenStream::from_iter([get_allow(), parented_lint_name]);

                TokenStream::from(TokenTree::Group(Group::new(
                    Delimiter::Bracket,
                    allow_lint_name,
                )))
            })
            .clone()
    })
}

// cfg(target_thread_local)

/// NOT for public use. "Used" only by [`allows_internals::generate_allows_attribute_macro_definition`] macro. [`allows_internals::generate_allows_attribute_macro_definition`] doesn't invoke this, but it generates code that invokes it.
///
/// Define a `proc` macro to allow the given lint. The proc macro will have the same name as the
/// given `lint_name`, except that any package-like separators (pairs of colons) :: are replaced
/// with an underscore _.
///
/// BEWARE: If the lint name doesn't exist, then even if the consumer code uses
/// `#[deny(unknown_lints)]`, that (incorrect lint name) will NOT get reported.
macro_rules! generate_allows_attribute_macro_definition_internal {
    ( $lint_path:path, $new_macro_name:ident ) => {
        #[proc_macro_attribute]
        pub fn $new_macro_name(
            _given_attrs: ::proc_macro::TokenStream,
            item: ::proc_macro::TokenStream,
        ) -> ::proc_macro::TokenStream {
            ::proc_macro::TokenStream::from_iter([
                $crate::get_hash(),
                $crate::lint_squared("$lint_path"),
                item,
            ])
        }
    };
}
//----

macro_rules! generate_allows_attribute_macro_definition_by_example {
    // Note: Do NOT prefix in the following with `crate::` like:
    // `crate::generate_allows_attribute_macro_definition_internal!(..);`
    ( $lint_path:path ) => {
        generate_allows_attribute_macro_definition_internal!(
            $lint_path,
            path_to_ident_mac!($lint_path)
        );
    };
}

// ----
//
// EXAMPLE Actual macros for public use
#[proc_macro_attribute]
pub fn unused(_given_attrs: TokenStream, item: TokenStream) -> TokenStream {
    // @TODO accept & ignore any given_attrs?
    // Otherwise refuse them:
    //assert!(given_attrs.is_empty(), "This #[allow::] must have no extra tokens.");

    // The string source for `attrs` is  internal, hence well formed.
    //let attrs = TokenStream::from_str("#[allow(unused)]").unwrap();

    TokenStream::from_iter([get_hash(), lint_squared("unused"), item])
}

generate_allows_attribute_macro_definition!(::clippy::almost_swapped);
generate_allows_attribute_macro_definition_by_example!(::clippy::approx_constant);

//generate_allows_attribute_macro_definition_internal!(clippy::almost_swapped, clippy_almost_swapped);
