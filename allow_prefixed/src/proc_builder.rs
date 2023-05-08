use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::auxiliary;

/// [`TokenStream`] consisting of one hash character: `#`. It serves as the leading character of the
/// injected code (just left of the injected `#[allow(...)]`).
pub fn get_hash() -> TokenStream {
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
/// Instead, `lint_path` contains no spaces. For example: `clippy::almost_swapped`.
///
/// For our purpose only. (It can contain only one pair of colons `::`, and NOT at the very
/// beginning.)
pub fn brackets_allow_lint(lint_path: &'static str) -> TokenStream {
    let (prefix_str, lint_str) = match lint_path.find(':') {
        Some(colon_index) => (&lint_path[..colon_index], &lint_path[colon_index + 2..]),
        None => ("", lint_path),
    };

    let prefix_lint = {
        let lint = TokenTree::Ident(Ident::new(lint_str, Span::call_site()));
        if prefix_str.is_empty() {
            auxiliary::token_trees_to_stream(&[lint])
        //TokenStream::from_iter([lint])
        } else {
            let prefix = match prefix_str {
                "clippy" => get_clippy(),
                "rustdoc" => get_rustdoc(),
                _ => panic!("Unsupported prefix: {}.", prefix_str),
            };
            let colon = get_colon();
            auxiliary::token_trees_to_stream(&[prefix, colon.clone(), colon, lint])
            //TokenStream::from_iter([prefix, colon.clone(), colon, lint])
        }
    };

    let parens_lint_path = TokenTree::Group(Group::new(Delimiter::Parenthesis, prefix_lint));

    let allow_parens_lint_path = auxiliary::token_trees_to_stream(&[get_allow(), parens_lint_path]);

    TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Bracket,
        allow_parens_lint_path,
    )))
}
