//! NOT for public use. Only to be used by `allow_prefixed` crate.
#![doc(html_no_source)]
#![deny(missing_docs)]

use proc_macro::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use std::{fmt::Display, iter::FromIterator, str::FromStr}; // TODO remove if we upgrade Rust edition

mod auxiliary;
mod proc_builder;

/// Generate the code that invokes `generate_allow_attribute_macro_internal` macro, and
/// as a result it defines an attribute macro for the given lint.
///
/// @TODO Move this doc to the macro: Param `pass_through` indicates whether the result attribute macro should just pass through its
/// input without injecting `#[allow(lint-name-here)]`. Used for removed/deprecated lints - for
/// backwards compatibility.
fn pass_through_deprecated_attrib_macro(
    lint_prefix: Option<&str>,
    properties: AllowMacroProperties,
    doc: &str,
) -> TokenStream {
    // @TODO  cfg: no/nightly, silent/scream _past_dummies, fixed_toolchains_conservative on
    // floating toolchain
    /* // TODO move below & implement:
    if !properties.until_major_minor.is_empty() {
        // emit #[deprecated = "..."]
    }*/
    /*
    let mut lint_name = lint_name_and_qualities.next().unwrap_or_else(|| {
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

    let mut lint_name_input = lint_name_and_qualities.peekable();
    assert!(
        lint_name_input.peek().is_none(),
        "Expecting no more tokens, but received: {:?}.",
        lint_name_input.collect::<Vec<_>>()
    );
    */
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

    let mut generate_internal_params = Vec::with_capacity(9);

    if let Some(lint_prefix) = &lint_prefix {
        // `lint_prefix` will be checked later. [TokenTree::clone] is documented to be cheap.
        generate_internal_params.push(TokenTree::Ident(Ident::new(lint_prefix, Span::call_site())));
        generate_internal_params.push(proc_builder::get_punct_alone(','));
    }
    // We could have passed the lint name TokenTree from upstream, but it's cheap to re-create:
    let lint_name_token = TokenTree::Ident(Ident::new(&properties.lint_name, Span::call_site()));
    generate_internal_params.push(lint_name_token.clone());
    generate_internal_params.push(proc_builder::get_punct_alone(','));

    let mut new_proc_macro_name_token = lint_name_token;
    if let Some(lint_prefix) = lint_prefix {
        let mut new_proc_macro_name_with_prefix = lint_prefix.to_owned();
        new_proc_macro_name_with_prefix.push('_');
        new_proc_macro_name_with_prefix.push_str(&properties.lint_name);
        new_proc_macro_name_token = TokenTree::Ident(Ident::new(
            &new_proc_macro_name_with_prefix,
            Span::call_site(),
        ));
    }
    generate_internal_params.push(new_proc_macro_name_token);
    generate_internal_params.push(proc_builder::get_punct_alone(','));

    generate_internal_params.push(TokenTree::Ident(Ident::new(
        if false/*TODO pass_through*/ { "true" } else { "false" },
        Span::call_site(),
    )));
    generate_internal_params.push(proc_builder::get_punct_alone(','));
    generate_internal_params.push(TokenTree::Literal(Literal::string(doc)));

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
pub fn generate_allow_attribute_macro_standard(lint_name_and_the_rest: TokenStream) -> TokenStream {
    TokenStream::new()
    //TODO
    //
    //pass_through_deprecated_attrib_macro(None, lint_name_and_the_rest.into_iter(), false)
}

/// Input: `prefix, lint_name`. Separated NOT by colons `::`, but by an exactly one comma.
///
/// Output: Attribute macro code that (when applied) injects `#[allow(lint_path_input)]`.
///
/// The macro name will be based on the given prefix and lint name, concatenated with an underscore
/// in between.
// @TODO pass_through - or have a separate macro for it?
#[proc_macro]
pub fn generate_allow_attribute_macro_prefixed(input: TokenStream) -> TokenStream {
    /*
    let mut input = input.into_iter();

    // TODO check if still applicable:
    //
    // The expected [prefix_and_lint_name] is NOT the same as if generated with (for example):
    //
    // ("clippy::all").parse::<TokenStream>().unwrap();
    //
    // because the above .parse() generates several TokenTrees, but the input we get for this macro
    // (from well formed invocations) generates only one top-level TokenTree (with exactly one
    // Group).
    let prefix = parse_value(&mut input, true, "lint_prefix");
    if let TokenTree::Ident(prefix) = prefix {
        pass_through_deprecated_attrib_macro(prefix.to_string(), input, false)
    } else {
        panic!(
            "Expected a lint prefix (an identifier), but received {}.",
            prefix
        )
    }*/
    input
}

/// Default lint applicability. Enum wordings are based on URLs like
/// https://doc.rust-lang.org/nightly/rustc/lints/listing/allowed-by-default.html.
enum LintDefault {
    Allowed,
    Warn,
    Deny,
}
impl LintDefault {
    fn to_str(&self) -> &str {
        match self {
            Self::Allowed => "allowed",
            Self::Warn => "warn",
            Self::Deny => "deny",
        }
    }
}
impl FromStr for LintDefault {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allowed" => Ok(Self::Allowed),
            "warn" => Ok(Self::Warn),
            "deny" => Ok(Self::Deny),
            other => Err(other.to_owned()),
        }
    }
}
impl Display for LintDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

/// Properties of our target generated attribute macros (to be used, and potentially aliased, by
/// users), except for the prefix. The field names reflect the "full" parameters (right of
/// `ALL_PARAMS`) accepted by macro by example [`allow_prefixed::any`].
struct AllowMacroProperties {
    lint_name_token_tree: TokenTree,
    lint_name: String,
    default: Option<LintDefault>, // for rustc (standard) only
    deprecated_msg: String,
    since_major_minor: String,
    until_major_minor: String,
    nightly: bool,
    not_yet: bool,
    not_anymore: bool,
}

impl AllowMacroProperties {
    fn nightly(&self) -> bool {
        self.since_major_minor == "nightly"
    }
}

fn parse_value(
    iter: &mut impl Iterator<Item = TokenTree>,
    expect_comma_afterwards: bool,
    description: &str,
) -> TokenTree {
    let value = iter.next();
    let value = value.unwrap_or_else(|| {
        panic!("Expecting {}, but reached an end of input.", description);
    });
    if expect_comma_afterwards {
        let comma = iter.next().unwrap_or_else(|| {
            panic!(
                "Expecting a comma after {}, but reached an end of input.",
                description
            )
        });
        if !matches!(&comma, TokenTree::Punct(p) if p.as_char()==',') {
            panic!("Expecting a comma, but received {:?}.", comma);
        }
    }
    value
}

fn parse_literal(
    iter: &mut impl Iterator<Item = TokenTree>,
    expect_comma_afterwards: bool,
    description: &str,
) -> String {
    let value = parse_value(iter, expect_comma_afterwards, description);
    if let TokenTree::Literal(literal) = &value {
        literal.to_string()
    } else {
        panic!(
            "Expecting {} as a literal, but received {:#?} instead.",
            description, value
        )
    }
}

/// Why a tuple of [String] and [TokenTree], instead of just [String]? When used for `lint_name`
/// (the first token to `any` macro in `allow_prefixed`), in addition to the string (ident name) we
/// want the original [TokenTree], so that we can reuse it.
fn parse_ident(
    iter: &mut impl Iterator<Item = TokenTree>,
    expect_comma_afterwards: bool,
    description: &str,
) -> (String, TokenTree) {
    let token_tree = parse_value(iter, expect_comma_afterwards, description);
    if let TokenTree::Ident(ident) = &token_tree {
        (ident.to_string(), token_tree)
    } else {
        panic!(
            "Expecting {} as an Ident, but received {:#?} instead.",
            description, token_tree
        )
    }
}

fn assert_no_more_tokens(token_tree_iter: &mut impl Iterator<Item = TokenTree>) {
    let mut token_trees = token_tree_iter.peekable();
    assert!(
        token_trees.peek().is_none(),
        "Expecting no more tokens, but received: {:?}.",
        token_trees.collect::<Vec<_>>()
    );
}

/// Return `token_tree` if it's a non-group token. Otherwise, assert that it's a group with no
/// delimiter, containing exactly one token (sub)tree, and return that (sub)tree.
fn token_unwrap_undelimited_group_if_any(token_tree: TokenTree) -> TokenTree {
    if let TokenTree::Group(group) = token_tree {
        let mut iter = group.stream().into_iter();
        assert_eq!(
            group.delimiter(),
            Delimiter::None,
            "Received a group. Expecting the delimiter to be \"None\", but it was: {:#?}.",
            group.delimiter()
        );

        if let Some(token_tree) = iter.next() {
            assert_no_more_tokens(&mut iter);
            token_tree
        } else {
            panic!("Received a group with a correct delimiter (\"None\"). Expecting exactly one item, but the group was empty.")
        }
    } else {
        token_tree
    }
}

fn parse_literal_bool(
    iter: &mut impl Iterator<Item = TokenTree>,
    expect_comma_afterwards: bool,
    description: &str,
) -> bool {
    let value = parse_value(iter, expect_comma_afterwards, description);
    let value = token_unwrap_undelimited_group_if_any(value);

    if let TokenTree::Ident(ident) = value {
        let ident = ident.to_string();
        if ident == "true" {
            true
        } else if ident == "false" {
            false
        } else {
            panic!(
                "Expecting {} as a bool literal, but received {}.",
                description, ident
            )
        }
    } else {
        panic!(
            "Expecting {} as a bool literal (Ident), but received {:#?} instead.",
            description, value
        )
    }
}

/// Parse all the properties from a comma-separated stream of values. The values and their order
/// reflect the "full" parameters (right of `ALL_PARAMS`) accepted by macro by example
/// [`allow_prefixed::any`].
fn parse_properties(
    token_trees: &mut impl Iterator<Item = TokenTree>,
    is_rustc: bool,
) -> AllowMacroProperties {
    let (lint_name, lint_name_token_tree) = parse_ident(token_trees, true, "lint name");

    let (default, _) = parse_ident(token_trees, true, "default");
    let default = if is_rustc {
        match default.parse::<LintDefault>() {
            Ok(default) => Some(default),
            Err(found) => panic!("Expecting a (rustc) lint default, but found: {}.", found),
        }
    } else {
        assert!(
            default == "_",
            "Expecting a (clippy|rustdoc) lint default to be an underscore _, but found: {}.",
            default
        );
        None
    };
    let deprecated_msg = parse_literal(
        token_trees,
        true,
        "deprecated (message, if other than default)",
    );
    let since_major_minor = parse_literal(token_trees, true, "since_major_minor");
    let nightly = parse_literal_bool(token_trees, true, "nightly");
    let until_major_minor = parse_literal(token_trees, true, "until_major_minor");
    let not_yet = parse_literal_bool(token_trees, true, "not_yet");
    let not_anymore = parse_literal_bool(token_trees, false, "not_anymore");

    let mut token_trees = token_trees.peekable();
    assert_no_more_tokens(&mut token_trees);
    AllowMacroProperties {
        lint_name_token_tree,
        lint_name,
        default,
        deprecated_msg,
        since_major_minor,
        until_major_minor,
        nightly,
        not_yet,
        not_anymore,
    }
}

/// Generate the documentation text and the whole target attribute macro to allow relevant
/// `clippy::` lint. The parameter `input` (stream) does NOT contain the lint prefix. It contains
/// all fields accepted by [`parse_properties`] (starting with the lint name). The same as the input
/// to macro_rules [`::allow_prefixed::any_with_nightly_as_bool`] after it accepts `ALL_PARAMS, clippy`.
#[proc_macro]
pub fn doc_and_attrib_macro_clippy(input: TokenStream) -> TokenStream {
    let properties = parse_properties(&mut input.clone().into_iter(), false);
    // emit [doc = "..."]
    // - rustc:
    //   https://doc.rust-lang.org/nightly/rustc/lints/listing/(allowed|warn|deny)-by-default.html
    //
    // - https://doc.rust-lang.org/nightly/rustdoc/lints.html#lint_name_here
    //
    // - clippy nightly -> "master":
    //   https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons
    //
    // - clippy versioned:
    //   https://rust-lang.github.io/rust-clippy/rust-1.65.0/index.html#alloc_instead_of_core

    // Based on https://rust-lang.github.io/rust-clippy/master/index.html > "Lint groups"
    // dropdown/filter > Deprecated, and
    // https://doc.rust-lang.org/nightly/rustc/lints/listing/index.html as of May 2023, we can treat
    // deprecated and removed/renamed lints as the same.
    //
    // Deprecate when:
    // - `#[cfg(floating_toolchain)]` and feature `fixed_toolchains_conservative` and
    //   `$since_major_minor` being "nightly"
    // - or: `not_anymore
    // - or: `not_yet && #[cfg(scream_future_dummies)]` - TODO consider
    // - or: properties.deprecated is not blank (not a blank string)

    let clippy_base = if properties.nightly() {
        "https://rust-lang.github.io/rust-clippy/master/index.html#".to_owned()
    } else {
        format!(
            "https://rust-lang.github.io/rust-clippy/rust-{}.0/index.html#",
            properties.since_major_minor
        )
    };
    let doc = format!(
        "Alias to `#[allow(clippy::{})]`. See {}/{}.",
        properties.lint_name, clippy_base, properties.lint_name
    );
    pass_through_deprecated_attrib_macro(Some("clippy"), properties, &doc)
}

/// Like [`doc_and_attrib_macro_clippy`], but for `rustc` ("standard", prefixless) lints.
#[proc_macro]
pub fn doc_and_attrib_macro_rustc(input: TokenStream) -> TokenStream {
    let properties = parse_properties(&mut input.clone().into_iter(), true);
    let rustc_base = "https://doc.rust-lang.org/nightly/rustc/lints/listing";

    let mut lint_name_with_hyphens = String::with_capacity(properties.lint_name.len());
    lint_name_with_hyphens.extend(
        properties
            .lint_name
            .chars()
            .map(|c| if c == '_' { '-' } else { c }),
    );

    assert!(properties.default.is_some(), "Allow macro definition for rustc (\"standard\", prefixless) lint {} require default applicability. And this should have been checked already.", properties.lint_name);
    let default = properties.default.as_ref().unwrap();

    let doc = format!(
        "Alias to `#[allow({})]`. See {}/{}-by-default.html#{}.",
        properties.lint_name, rustc_base, default, lint_name_with_hyphens
    );
    pass_through_deprecated_attrib_macro(Some("clippy"), properties, &doc)
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

    let mut token_streams = Vec::with_capacity(9);

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

//----------------
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
