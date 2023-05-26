//! NOT for public use. Only to be used by `allow_prefixed` crate.
#![doc(html_no_source)]
#![deny(missing_docs)]

use proc_macro::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use std::iter::FromIterator; // TODO remove if we upgrade Rust edition

mod auxiliary;
mod proc_builder;

/// Convert given lint path to a quoted string literal. The path must contain exactly one double
/// colon `::` separator, which will be replaced with an uderscore inthe result
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
    let literal = lint_path.chars().filter(|c| *c != ' ').collect::<String>();

    TokenStream::from(TokenTree::Literal(Literal::string(&literal)))
}

/// Generate the code that invokes `generate_allow_attribute_macro_internal` macro, and
/// as a result it defines an attribute macro for the given lint.
///
/// Param `pass_through` indicates whether the result attribute macro should just pass through its
/// input without injecting `#[allow(lint-name-here)]`. Used for removed/deprecated lints - for
/// backwards compatibility.
fn generate_allow_attribute_macro_from_iter(
    lint_prefix: Option<Ident>,
    mut lint_name_input: impl Iterator<Item = TokenTree>,
    pass_through: bool,
) -> TokenStream {
    let mut lint_name = lint_name_input.next().unwrap_or_else(|| {
        panic!("Expecting a lint name (Identifier), but reached the end of the input.")
    });
    // @TODO test in Rust 1.45.2, if the following is not needed then remove:
    // In Rust 1.45.2 and older, `lint_name` here is not `TokenTree::Ident(_)`, but a `Group`
    // containing `TokenTree::Ident(_)`.
    //
    // @TODO If we upgrade min. Rust version, test this and eliminate if not needed anymore.
    if let TokenTree::Group(group) = &lint_name {
        lint_name = group
            .stream()
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("Expecting an Ident in the group."));
    }

    if !matches!(&lint_name, TokenTree::Ident(_)) {
        panic!(
            "Expecting a TokenTree::Ident(lint_name), but received {:?}.",
            lint_name
        )
    }

    let mut lint_name_input = lint_name_input.peekable();
    assert!(
        lint_name_input.peek().is_none(),
        "Expecting no more tokens, but received: {:?}.",
        lint_name_input.collect::<Vec<_>>()
    );

    // Note: Do NOT prefix the generated Rust invocation (from `allow_prefixed` itself) in the
    // following with `crate::` like:
    // `crate::generate_allow_attribute_macro_internal_prefixed!(...);` That fails!
    let generate_internal = TokenTree::Ident(Ident::new(
        if lint_prefix.is_some() {
            "generate_allow_attribute_macro_internal_prefixed"
        } else {
            "generate_allow_attribute_macro_internal_standard"
        },
        Span::call_site(),
    ));
    let exclamation = proc_builder::get_punct_joint('!');

    let mut generate_internal_params = Vec::with_capacity(6); //@TODO capacity
                                                              // @TODO
                                                              // 1. change type of lint_prefix to Option<TokenTree>
                                                              // 2. clone() it and run a check that it contains exactly one Ident
                                                              // 3. change the 1st if {...} below to use that TokenTree.
                                                              // 4. extract the Ident in the 2nd if {...} below - use the TokenTree from the Option param
                                                              //    instead.
    if let Some(lint_prefix) = &lint_prefix {
        generate_internal_params.push(TokenTree::Ident(lint_prefix.clone()));
        generate_internal_params.push(proc_builder::get_punct_alone(','));
    }
    generate_internal_params.push(lint_name.clone());
    generate_internal_params.push(proc_builder::get_punct_alone(','));

    let mut generated_proc_macro_name = lint_name;
    if let Some(lint_prefix) = lint_prefix {
        //@TODO remove .clone()
        let mut lint_prefix = lint_prefix.to_string();
        lint_prefix.push('_');
        lint_prefix.extend(generated_proc_macro_name.to_string().chars());
        generated_proc_macro_name = TokenTree::Ident(Ident::new(&lint_prefix, Span::call_site()));
    }
    generate_internal_params.push(generated_proc_macro_name);
    generate_internal_params.push(proc_builder::get_punct_alone(','));
    generate_internal_params.push(TokenTree::Ident(Ident::new(
        if pass_through { "true" } else { "false" },
        Span::call_site(),
    )));

    let generate_internal_params_parens = TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::from_iter(generate_internal_params),
    ));
    let tokens = [
        generate_internal,
        exclamation,
        generate_internal_params_parens,
        proc_builder::get_punct_alone(';'),
    ];

    // TODO remove if we upgrade Rust min. version, or edition to 2021
    auxiliary::token_trees_to_stream(&tokens)
    //TokenStream::from_iter(tokens) // use if we upgrade Rust min. version, or edition to 2021
}

/// Like [`generate_allow_attribute_macro_prefixed!`], but generate a macro for a given
/// standard (prefixless) `rustc` lint. The macro name itself will be the same as the lint name.
#[proc_macro]
pub fn generate_allow_attribute_macro_standard(lint_name_input: TokenStream) -> TokenStream {
    generate_allow_attribute_macro_from_iter(None, lint_name_input.into_iter(), false)
}

/// Input: prefix, lint_name. Output: Attribute macro code that (when applied) injects
/// `#[allow(lint_path_input)]`.
///
/// `lint_path_input` must contain exactly one separator: a comma.
///
/// The macro name will be based on the given prefix and lint name, concatenated with an underscore
/// in between.
#[proc_macro]
pub fn generate_allow_attribute_macro_prefixed(prefix_and_lint_name: TokenStream) -> TokenStream {
    let mut prefix_and_lint_name = prefix_and_lint_name.into_iter();

    // The expected lint_path_name_input is NOT the same as if generated with:
    //
    // ("std::unused").parse::<TokenStream>().unwrap();
    //
    // because the above .parse() generates several TokenTrees, but the input we get for this macro
    // (from well formed invocations) generates only one top-level TokenTree (with exactly one
    // Group).

    // @TODO re-use - see check_that_prefixed_lint_exists
    let prefix = prefix_and_lint_name.next().unwrap_or_else(|| {
        panic!("Expecting lint prefix and lint name, but reached an end of input.")
    });
    let prefix = match prefix {
        TokenTree::Ident(prefix) => prefix,
        _ => panic!(
            "Expecting a TokenTree::Ident (of a lint prefix), but received {:?}.",
            prefix
        ),
    };

    let comma = prefix_and_lint_name.next().unwrap_or_else(|| {
        panic!("Expecting a comma after the lint prefix, but received an empty input.")
    });
    if !matches!(&comma, TokenTree::Punct(p) if p.as_char()==',') {
        panic!("Expecting a comma, but received {:?}.", comma);
    }
    /*
    let name = prefix_and_lint_name.next().unwrap_or_else(|| panic!("Expecting lint name after the prefix and comma, but reached an end of input."));
    let name = match name {
        TokenTree::Ident(name) => name,
        _ => panic!(
            "Expecting a TokenTree::Ident (of a lint name), but received {:?}.",
            name
        ),
    };

    let mut prefix_and_lint_name = prefix_and_lint_name.peekable();
    assert!(
        prefix_and_lint_name.peek().is_none(),
        "Expecting no more tokens, but received: {:?}.",
        prefix_and_lint_name.collect::<Vec<_>>()
    );*/

    generate_allow_attribute_macro_from_iter(Some(prefix), prefix_and_lint_name, false)
}

/// Generate code like: `#[allow(prefix::lint_name)] const _: () = ();`. Use it together with
/// `#[deny(unknown_lints)]` to check for any incorrect prefixed lints.
///
/// Param `prefix_and_lint_name_without_double_colon` contains a (one level) lint prefix (that is,
/// either `clippy` or `rustdoc`), and a lint name, separated by a comma. (No double colon `::` -
/// that will be generated).
///
/// When calling this from a `macro_rules!`, you want to capture the prefix and lint name as `tt`
/// (and NOT as `ident`) metavariable. Otherwise use it with `defile` crate.
///
/// This does NOT get checked with `cargo check`! Use `cargo clippy` or `cargo rustdoc`,
/// respectively, instead.
#[proc_macro]
pub fn check_that_prefixed_lint_exists(
    prefix_and_lint_name_without_double_colon: TokenStream,
) -> TokenStream {
    // The `const _` is to check that the lint prefix & path is valid (thanks to
    // `#![deny(unknown_lints)]` in `lib.rs` or `allow_prefixed` crate.
    //
    // For a similar, but simplified version, see also `macro_rules! standard_lint` in
    // `allow_prefixed` crate.
    let mut prefix_and_lint_name_without_double_colon =
        prefix_and_lint_name_without_double_colon.into_iter();

    let prefix = prefix_and_lint_name_without_double_colon
        .next()
        .unwrap_or_else(|| {
            panic!("Expecting a lint prefix (Identifier), but reached the end of the input.")
        });
    let prefix = if let TokenTree::Ident(prefix) = prefix {
        prefix.to_string()
    } else {
        panic!(
            "Expecting a TokenTree::Ident(prefix), but received {:?}.",
            prefix
        );
    };

    let comma = prefix_and_lint_name_without_double_colon
        .next()
        .unwrap_or_else(|| {
            panic!("Expecting a lint prefix (Identifier), but reached the end of the input.")
        });
    if !matches!(&comma, TokenTree::Punct(p) if p.as_char()==',') {
        panic!(
            "Expecting a comma (a TokenTree::Punct), but received {:?}.",
            comma
        );
    }

    let name = prefix_and_lint_name_without_double_colon
        .next()
        .unwrap_or_else(|| {
            panic!("Expecting a lint name (Identifier), but reached the end of the input.")
        });
    // `span` must NOT be `Span::call_site()`. See https://github.com/rust-lang/rust/issues/109881.
    let (name, span) = if let TokenTree::Ident(name) = name {
        (name.to_string(), name.span())
    } else {
        panic!(
            "Expecting a TokenTree::Ident(lint_name), but received {:?}.",
            name
        );
    };

    let mut token_streams = Vec::with_capacity(6); //@TODO capacity

    token_streams.push(proc_builder::get_hash());
    token_streams.push(proc_builder::brackets_allow_lint_parts(
        &prefix, &name, span,
    ));
    token_streams.push(TokenStream::from(proc_builder::get_ident_tree("const")));
    token_streams.push(TokenStream::from(proc_builder::get_ident_tree("_")));
    token_streams.push(TokenStream::from(proc_builder::get_colon_alone()));

    token_streams.push(TokenStream::from(proc_builder::get_parens(
        TokenStream::new(),
    )));
    token_streams.push(TokenStream::from(proc_builder::get_punct_alone('=')));
    token_streams.push(TokenStream::from(proc_builder::get_parens(
        TokenStream::new(),
    )));
    token_streams.push(TokenStream::from(proc_builder::get_punct_alone(';')));
    auxiliary::token_streams_to_stream(&token_streams)
}

macro_rules! empty_proc_macro_gen {
    ($macro_name:tt, $subdoc_literal:tt) => {
        // Generated macro $macro_name and its documentation
        //#[doc = stringify!(Generated macro: $macro_name based on ...)]
        //#[doc = "generated proc_mac with a #[doc = ...]-based documentation. This documentation DOES show up in rust-analyzer."]
        #[doc = "Alias to #[allow(clippy::"]
        //#[doc = $macro_name] // -- this would be missing enclosing quotes "..."
        /// to $macro_name
        #[doc = ")]"]
        #[proc_macro]
        pub fn $macro_name(_input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            ::proc_macro::TokenStream::new()
        }
    };
}

empty_proc_macro_gen!(generated_proc_macro, "prefix:lint");

// Can't use stringify! here:
//
// empty_proc_macro_generator!(generated_proc_macro, stringify!(prefix:lint));
//
// See, and give thums up to, please:
// - https://github.com/rust-lang/rust-analyzer/issues/8092
// - https://github.com/rust-lang/rust-analyzer/issues/14772

/// Helper.
/// TODO if we parse: Requires `use proc_macro::{TokenStream ETC.}` at the caller scope.
#[proc_macro]
pub fn generate_proc_mac_with_doc_attrib(_input: TokenStream) -> TokenStream {
    "#[proc_macro]#[doc = \"Documented by a `#[doc = \\\"...\\\" ]` attribute.\" ]
    pub fn generated_proc_mac_with_doc_attrib(
        _input: ::proc_macro::TokenStream,
    ) -> ::proc_macro::TokenStream {
        ::proc_macro::TokenStream::new()
    }
    "
    .parse()
    .unwrap()
}
