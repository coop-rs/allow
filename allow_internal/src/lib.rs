//! NOT for public use. Only to be used by `allow` crate.

extern crate proc_macro; // TODO remove if we upgrade Rust edition

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::FromIterator; // TODO remove if we upgrade Rust edition

#[proc_macro]
pub fn path_to_str_literal(lint_path_input: TokenStream) -> TokenStream {
    let mut lint_path_input = lint_path_input.into_iter();
    let lint_path = lint_path_input
        .next()
        .unwrap_or_else(|| panic!("Expecting a path, but reached an end of macro input."));
    let leftover = lint_path_input.next();
    assert!(
        leftover.is_none(),
        "Expecting nothing else after path {}, but received: {:?}.",
        lint_path,
        leftover
    );

    let lint_path = lint_path.to_string();
    // Once https://github.com/rust-lang/rust/issues/72826 is stable, use:
    //
    // lint_path.remove_matches(char::is_whitespace);
    //
    // Or, once https://github.com/rust-lang/rust/issues/94780 is stable, use:
    //
    // let literal = String::with_capacity(lint_path.len());
    // lint_path.chars().filter(|c| *c!=' ').collect_into(&mut literal);
    let literal = lint_path.chars().filter(|c| *c != ' ').collect::<String>();

    //let mut result = Vec::with_capacity(1);
    //result.push();
    TokenStream::from(TokenTree::Literal(Literal::string(&literal)))
}

fn generate_allow_attribute_macro_definition_from_iter(
    lint_prefix: Option<Ident>,
    mut lint_name_input: impl Iterator<Item = TokenTree>,
) -> TokenStream {
    let lint_name = lint_name_input.next().unwrap_or_else(|| {
        panic!("Expecting a lint name (Identifier), but reached the end of the input.")
    });
    // @TODO in Rust 1.42: revert to: if !matches!(&lint_name, TokenTree::Ident(_)) {..}
    match &lint_name {
        TokenTree::Ident(_) => (),
        _ => panic!(
            "Expecting a TokenTree::Ident(lint_name), but received {:?}.",
            lint_name
        ),
    };

    let mut lint_name_input = lint_name_input.peekable();
    assert!(
        lint_name_input.peek().is_none(),
        "Expecting no more tokens, but received: {:?}.",
        lint_name_input.collect::<Vec<_>>()
    );

    // Note: Do NOT prefix the generated Rust invocation (from `allow` itself) in the following
    // with `crate::` like: `crate::generate_allow_attribute_macro_definition_internal!(...);`
    let generate_internal = TokenTree::Ident(Ident::new(
        "generate_allow_attribute_macro_definition_internal",
        Span::call_site(),
    ));
    let exclamation = TokenTree::Punct(Punct::new('!', Spacing::Joint));

    let mut generate_internal_params = Vec::with_capacity(6);
    if let Some(lint_prefix) = &lint_prefix {
        generate_internal_params.push(TokenTree::Ident(lint_prefix.clone()));
        generate_internal_params.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        generate_internal_params.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
    }
    generate_internal_params.push(lint_name.clone());
    generate_internal_params.push(TokenTree::Punct(Punct::new(',', Spacing::Joint)));

    if let Some(lint_prefix) = lint_prefix {
        let mut lint_prefix = lint_prefix.to_string();
        lint_prefix.push('_');
        lint_prefix.extend(lint_name.to_string().chars());
        let lint_name = TokenTree::Ident(Ident::new(&lint_prefix, Span::call_site()));
        generate_internal_params.push(lint_name);
    } else {
        generate_internal_params.push(lint_name);
    }

    let generate_internal_params_parens = TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::from_iter(generate_internal_params),
    ));
    let tokens_arr = [
        generate_internal,
        exclamation,
        generate_internal_params_parens,
        TokenTree::Punct(Punct::new(';', Spacing::Joint)),
    ];
    // TODO remove if we upgrade Rust min. version, or edition to 2021
    let mut tokens = Vec::with_capacity(4);
    tokens.extend_from_slice(&tokens_arr);
    TokenStream::from_iter(tokens)
    //TokenStream::from_iter(tokens) // use if we upgrade Rust min. version, or edition to 2021
}

#[proc_macro]
pub fn generate_allow_attribute_macro_definition_standard(
    lint_name_input: TokenStream,
) -> TokenStream {
    generate_allow_attribute_macro_definition_from_iter(None, lint_name_input.into_iter())
}

/// Input: prefix::lint_name
#[proc_macro]
pub fn generate_allow_attribute_macro_definition_prefixed(
    lint_path_input: TokenStream,
) -> TokenStream {
    let mut lint_path_input = lint_path_input.into_iter();

    // The expected lint_path_name_input is NOT the same as if generated with:
    //
    // ("std::unused").parse::<TokenStream>().unwrap();
    //
    // because the above .parse() generates several TokenTrees, but the input we get for this macro
    // (from well formed invocations) generates only one top-level TokenTree (with exactly one
    // Group).

    let token_tree = lint_path_input.next();
    let token_tree =
        token_tree.unwrap_or_else(|| panic!("Expecting lint path, but received an empty input."));
    let group = match token_tree {
        TokenTree::Group(group) => group,
        _ => panic!(
            "Expecting a TokenTree::Group, but received {:?}.",
            token_tree
        ),
    };
    let group = group.stream();
    let mut group = group.into_iter();

    let prefix = group.next().unwrap_or_else(|| {
        panic!("Expecting a lint prefix (Identifier), but reached the end of the input.")
    });
    let prefix = match prefix {
        TokenTree::Ident(prefix) => prefix,
        _ => panic!(
            "Expecting a TokenTree::Ident(prefix), but received {:?}.",
            prefix
        ),
    };

    (0..2).for_each(|_| {
        let punct = group.next().unwrap_or_else(|| {
            panic!("Expecting a colon (Punct ':'...), but reached the end of the input.")
        });
        let punct = match punct {
            TokenTree::Punct(punct) => punct,
            _ => panic!(
                "Expecting a TokenTree::Punct(Punct(':'...)), but received {:?}.",
                punct
            ),
        };
        assert_eq!(
            punct.as_char(),
            ':',
            "Expecting a colon, but received different Punct for '{}'.",
            punct.as_char()
        );
    });

    generate_allow_attribute_macro_definition_from_iter(Some(prefix), group.into_iter())
}
