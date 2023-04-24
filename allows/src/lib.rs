#![deny(unknown_lints)]

use allows_internals::{
    generate_allows_attribute_macro_definition_prefixed,
    generate_allows_attribute_macro_definition_standard,
};
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

/// [`TokenStream`] consisting of one hash character: `#`. It serves as the leading character of
/// the injected code (just left of the injected "[allow(...)]").
fn get_hash() -> TokenStream {
    TokenStream::from(TokenTree::Punct(Punct::new('#', Spacing::Joint)))
}

fn get_colon() -> TokenTree {
    TokenTree::Punct(Punct::new(':', Spacing::Joint))
}

fn get_clippy() -> TokenTree {
    TokenTree::Ident(Ident::new("clippy", Span::call_site()))
}

fn get_rustdoc() -> TokenTree {
    TokenTree::Ident(Ident::new("rustdoc", Span::call_site()))
}

/// [`TokenTree`] consisting of one identifier: `allow`.
fn get_allow() -> TokenTree {
    TokenTree::Ident(Ident::new("allow", Span::call_site()))
}
// -----

/// Param `lint_path` is NOT an &str of proc macro representation of `macro_rules!` type `path` -
/// because such a proc macro representation is a Group of Ident, and when transformed by
/// `to_string()` (`or format!(...)`), it gets one space inserted on each side of `::`.
///
/// Instead, `lint_path` contains no spaces. For example: `clippy::all`.
///
/// For our purpose only. (It can contain only one pair of colons `::`, and NOT at the very
/// beginning.)
fn brackets_allow_lint(lint_path: &'static str) -> TokenStream {
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

    let parens_lint_path = TokenTree::Group(Group::new(Delimiter::Parenthesis, prefix_lint));

    let allow_parens_lint_path = TokenStream::from_iter([get_allow(), parens_lint_path]);

    TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Bracket,
        allow_parens_lint_path,
    )))
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
        #[proc_macro_attribute]
        pub fn $new_macro_name(
            given_attrs: ::proc_macro::TokenStream,
            item: ::proc_macro::TokenStream,
        ) -> ::proc_macro::TokenStream {
            assert!(
                given_attrs.is_empty(),
                "Do not pass any attribute parameters."
            );
            ::proc_macro::TokenStream::from_iter([
                $crate::get_hash(),
                $crate::brackets_allow_lint(::allows_internals::path_to_str_literal!($lint_path)),
                item,
            ])
        }
    };
}

macro_rules! standard_lint {
    // the `const _` is to check that the lint name is valid
    ($lint_name:ident) => {
        #[deny(unknown_lints)]
        #[allow($lint_name)]
        const _: () = ();
        generate_allows_attribute_macro_definition_standard!($lint_name);
    };
}
macro_rules! prefixed_lint {
    // the `const _` is to check that the lint name is valid
    ($lint_path:path) => {
        #[deny(unknown_lints)]
        #[allow($lint_path)]
        const _: () = ();
        generate_allows_attribute_macro_definition_prefixed!($lint_path);
    };
}
// @TODO test that e.g. non_existing_std_lint fails
standard_lint!(array_into_iter);
standard_lint!(unused);
// TODO compile test that the following fails
//standard_lint!(wrong_lint);

prefixed_lint!(clippy::assign_ops);
// TODO compile test that the following fails - BUT ONLY with `cargo clippy`
prefixed_lint!(clippy::WRONG_LINT);
