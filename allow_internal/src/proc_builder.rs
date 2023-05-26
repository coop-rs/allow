use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::auxiliary;

/// [`TokenTree`] consisting of one punctuation character.
pub fn get_punct_joint(c: char) -> TokenTree {
    TokenTree::Punct(Punct::new(c, Spacing::Joint))
}

pub fn get_punct_alone(c: char) -> TokenTree {
    TokenTree::Punct(Punct::new(c, Spacing::Alone))
}

/// [`TokenStream`] consisting of one hash character: `#`. It serves as the leading character of the
/// injected code (just left of the injected `#[allow(...)]`).
pub fn get_hash() -> TokenStream {
    TokenStream::from(get_punct_alone('#'))
}

/// [`TokenStream`] consisting of one identifier/keyword with the given name.
pub fn get_ident_tree(name: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(name, Span::call_site()))
}

pub fn get_colon_joint() -> TokenTree {
    get_punct_joint(':')
}

pub fn get_colon_alone() -> TokenTree {
    get_punct_alone(':')
}

fn get_clippy() -> TokenTree {
    get_ident_tree("clippy")
}

fn get_rustdoc() -> TokenTree {
    get_ident_tree("rustdoc")
}

/// [`TokenTree`] consisting of one identifier: `allow`.
fn get_allow() -> TokenTree {
    get_ident_tree("allow")
}

pub fn get_parens(enclosed_stream: TokenStream) -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Parenthesis, enclosed_stream))
}

/// Param `lint_str` is an empty string if the lint is prefixless (standard, "rustc" lint).
///
/// Param `span` must NOT be `Span::call_site()`, but it MUST come from the consumer's code. See
/// https://github.com/rust-lang/rust/issues/109881.
pub fn brackets_allow_lint_parts(prefix_str: &str, lint_str: &str, span: Span) -> TokenStream {
    let prefix_lint = {
        let lint = TokenTree::Ident(Ident::new(lint_str, span.clone()));
        if prefix_str.is_empty() {
            auxiliary::token_trees_to_stream(&[lint])
            //TokenStream::from_iter([lint])
        } else {
            let mut prefix = match prefix_str {
                "clippy" => get_clippy(),
                "rustdoc" => get_rustdoc(),
                _ => panic!("Unsupported prefix: {}.", prefix_str),
            };
            prefix.set_span(span);
            auxiliary::token_trees_to_stream(&[prefix, get_colon_joint(), get_colon_alone(), lint])
            //TokenStream::from_iter([prefix, get_colon_joint(), get_colon_alone(), lint])
        }
    };

    let parens_lint_path = get_parens(prefix_lint);

    let allow_parens_lint_path = auxiliary::token_trees_to_stream(&[get_allow(), parens_lint_path]);

    TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Bracket,
        allow_parens_lint_path,
    )))
}
