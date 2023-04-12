#![feature(thread_local, local_key_cell_methods, option_get_or_insert_default)]
#![deny(unknown_lints)]

use allows_internals::{
    generate_allows_attribute_macro_definition_prefixed,
    generate_allows_attribute_macro_definition_standard,
};
use once_cell::unsync::OnceCell;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::cell::RefCell;
use std::collections::HashMap;

const USE_ANY_THREAD_LOCAL_CACHE: bool = false;
const USE_ATTRIB: bool = false;

type LintsMap = HashMap<&'static str, TokenStream>;

// `#[thread_local] static ONE: TokenStream = TokenStream::from(...)`
//
// failed, and rustc was suggesting `Lazy::new(...)`. However, that would involve a Mutex.
//
// See https://docs.rs/once_cell/latest/once_cell/unsync/struct.OnceCell.html
#[thread_local]
static HASH_ATTRIB: OnceCell<TokenStream> = OnceCell::new();
#[thread_local]
static COLON_ATTRIB: OnceCell<TokenTree> = OnceCell::new();
// If we ever have more tool prefixes than `clippy` and `rustdoc`, we may want to replace
// CLIPPY_ATTRIB and RUSTDOC_ATRRIB with a `Vec` or a `HashMap`.
#[thread_local]
static CLIPPY_ATTRIB: OnceCell<TokenTree> = OnceCell::new();
#[thread_local]
static RUSTDOC_ATTRIB: OnceCell<TokenTree> = OnceCell::new();
#[thread_local]
static ALLOW_ATTRIB: OnceCell<TokenTree> = OnceCell::new();

#[thread_local]
static LINTS_ATTRIB: RefCell<Option<LintsMap>> = RefCell::new(None);

thread_local! {
    // Read-only.
    static HASH_MACROED: TokenStream = generate_hash();
    static COLON_MACROED: TokenTree = generate_colon();
    static CLIPPY_MACROED: TokenTree = generate_clippy();
    static RUSTDOC_MACROED: TokenTree = generate_rustdoc();
    static ALLOW_MACROED: TokenTree = generate_allow();

    /// A map: lint name => token stream for: [allow(...)]. Read-write.
    static LINTS_MACROED: RefCell<LintsMap> = RefCell::new(LintsMap::new());
}

// Get the relevant parts from thread local storage (if enabled, and if stored already). Generate
// otherwise (and also put in thread local storage, if enabled). Then `.clone()` and return.
fn get_hash() -> TokenStream {
    if USE_ANY_THREAD_LOCAL_CACHE {
        if USE_ATTRIB {
            HASH_ATTRIB.get_or_init(generate_hash).clone()
        } else {
            HASH_MACROED.with(Clone::clone)
        }
    } else {
        generate_hash()
    }
}

fn get_colon() -> TokenTree {
    if USE_ANY_THREAD_LOCAL_CACHE {
        if USE_ATTRIB {
            COLON_ATTRIB.get_or_init(generate_colon).clone()
        } else {
            COLON_MACROED.with(Clone::clone)
        }
    } else {
        generate_colon()
    }
}

fn get_clippy() -> TokenTree {
    if USE_ANY_THREAD_LOCAL_CACHE {
        if USE_ATTRIB {
            CLIPPY_ATTRIB.get_or_init(generate_clippy).clone()
        } else {
            CLIPPY_MACROED.with(Clone::clone)
        }
    } else {
        generate_clippy()
    }
}

fn get_rustdoc() -> TokenTree {
    if USE_ANY_THREAD_LOCAL_CACHE {
        if USE_ATTRIB {
            RUSTDOC_ATTRIB.get_or_init(generate_rustdoc).clone()
        } else {
            RUSTDOC_MACROED.with(Clone::clone)
        }
    } else {
        generate_rustdoc()
    }
}

fn get_allow() -> TokenTree {
    if USE_ANY_THREAD_LOCAL_CACHE {
        if USE_ATTRIB {
            ALLOW_ATTRIB.get_or_init(generate_allow).clone()
        } else {
            ALLOW_MACROED.with(Clone::clone)
        }
    } else {
        generate_allow()
    }
}

//-----
//
// Generate the parts (but re-using other parts if stored in thread local already).
//
// Independent of thread local storage.

/// Generate [`TokenStream`] consisting of one hash character: `#`. It serves as the leading character of the injected code (just left of the injected "[allow(...)]").
fn generate_hash() -> TokenStream {
    TokenStream::from(TokenTree::Punct(Punct::new('#', Spacing::Joint)))
}

fn generate_colon() -> TokenTree {
    TokenTree::Punct(Punct::new(':', Spacing::Joint))
}

fn generate_clippy() -> TokenTree {
    TokenTree::Ident(Ident::new("clippy", Span::call_site()))
}

fn generate_rustdoc() -> TokenTree {
    TokenTree::Ident(Ident::new("rustdoc", Span::call_site()))
}

/// Generate [`TokenTree`] consisting of one identifier: `allow`.
fn generate_allow() -> TokenTree {
    TokenTree::Ident(Ident::new("allow", Span::call_site()))
}
// -----

fn with_lints<F>(f: F) -> TokenStream
where
    F: FnOnce(&mut LintsMap) -> TokenStream,
{
    if USE_ANY_THREAD_LOCAL_CACHE {
        if USE_ATTRIB {
            let mut lints = LINTS_ATTRIB.borrow_mut();
            f(lints.get_or_insert_default())
        } else {
            LINTS_MACROED.with_borrow_mut(|lints| f(lints))
        }
    } else {
        let mut lints = LintsMap::with_capacity(1);
        f(&mut lints)
    }
}

/// Param `lint_path` is NOT an &str of proc macro representation of macro_rules! type `path` -
/// because such a proc macro representation is a Group of Ident, and when transformed by
/// `to_string()` (`or format!(...)`), it gets one space inserted on each side of `::`.
///
/// Instead, `lint_path` contains no spaces. For example: `clippy::all`.
///
/// For our purpose only. (It can contain only one pair of colons `::`, and NOT at the very
/// beginning.)
fn brackets_allow_lint(lint_path: &'static str) -> TokenStream {
    with_lints(|lints| {
        let entry = lints.entry(lint_path);
        entry
            .or_insert_with(|| {
                let (prefix_str, lint_str) = match lint_path.find(':') {
                    Some(colon_index) => (&lint_path[..colon_index], &lint_path[colon_index + 2..]),
                    None => ("", lint_path),
                };

                let prefix_lint = {
                    let lint = TokenTree::Ident(Ident::new(lint_str, Span::call_site()));
                    if prefix_str.is_empty() {
                        TokenStream::from_iter([lint])
                    } else {
                        let prefix = match prefix_str {
                            "clippy" => get_clippy(),
                            "rustdoc" => get_rustdoc(),
                            _ => panic!("Unsupported prefix: {prefix_str}."),
                        };
                        let colon = get_colon();
                        TokenStream::from_iter([prefix, colon.clone(), colon, lint])
                    }
                };

                let parens_lint_path =
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, prefix_lint));

                let allow_parens_lint_path =
                    TokenStream::from_iter([get_allow(), parens_lint_path]);

                TokenStream::from(TokenTree::Group(Group::new(
                    Delimiter::Bracket,
                    allow_parens_lint_path,
                )))
            })
            .clone()
    })
}

// cfg(target_thread_local)

/// NOT for public use. "Used" only by
/// [`allows_internals::generate_allows_attribute_macro_definition_standard`] and
/// [`allows_internals::generate_allows_attribute_macro_definition_prefixed`] macros. Those macros
/// don't invoke this, but instead they generate code that invokes it.
///
/// This generates a definition of a `proc` attribute macro to allow the given lint. The proc macro
/// will have the same name as the given `lint_path`, except that any package-like separators (pairs
/// of colons) :: are replaced with an underscore _.
///
/// Param `lint_path` must NOT contain any whitespace, and it can contain max. one pair of colons
/// `::` (for `clippy::` or `rustdoc::` lints).
#[allow(unused_macros)]
macro_rules! generate_allows_attribute_macro_definition_internal {
    ( $lint_path:path, $new_macro_name:ident ) => {
        #[deny(unknown_lints)]
        #[proc_macro_attribute]
        pub fn $new_macro_name(
            given_attrs: ::proc_macro::TokenStream,
            item: ::proc_macro::TokenStream,
        ) -> ::proc_macro::TokenStream {
            #![deny(unknown_lints)]
            //@TODO uncomment the following - once all fixed:
            //#[allow($lint_path)]
            let _checking_the_lint_name_is_valid: ();

            // @TODO discuss allowing (any well formed) attribute parameters
            assert!(
                given_attrs.is_empty(),
                "Do not pass any attribute parameters."
            );
            ::proc_macro::TokenStream::from_iter([
                $crate::get_hash(),
                // [allow(lint_path_here_unquoted)]
                $crate::brackets_allow_lint(::allows_internals::path_to_str_literal!($lint_path)),
                item,
            ])
        }
    };
}

macro_rules! standard_lints {
    ($( $lint_name:ident ),*) => {
        $(
            generate_allows_attribute_macro_definition_standard!($lint_name);
        )*
    };
}
macro_rules! prefixed_lints {
    ($prefix:ident, $( $lint_name:ident ),*) => {
        $(
            generate_allows_attribute_macro_definition_prefixed!($prefix, $lint_name);
        )*
    };
}
// @TODO test that e.g. non_existing_std_lint fails
standard_lints!(array_into_iter, unused, bufo);

prefixed_lints!(clippy, assign_ops);

/*#[proc_macro_attribute] // EXAMPLE Actual macros for public use
pub fn unused(_given_attrs: TokenStream, item: TokenStream) -> TokenStream {
    // The string source for `attrs` is  internal, hence well formed.
    //let attrs = TokenStream::from_str("#[allow(unused)]").unwrap();
    TokenStream::from_iter([get_hash(), brackets_allow_lint("unused"), item])
}*/
