#![deny(unknown_lints)]
#![cfg_attr(
    unstable_feature,
    feature(
        c_unwind,
        strict_provenance,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint
    )
)]

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[macro_use]
mod wrapper_macros;

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

// @TODO test that e.g. non_existing_std_lint fails
// TODO compile test that the following fails
//standard_lint!(wrong_lint);

standard_lint!(absolute_paths_not_starting_with_crate);
standard_lint!(box_pointers);
// elided_lifetimes_in_paths is at crate level only
standard_lint!(explicit_outlives_requirements);
standard_lint!(ffi_unwind_calls);
standard_lint!(fuzzy_provenance_casts);
standard_lint!(keyword_idents);
standard_lint!(let_underscore_drop);
standard_lint!(lossy_provenance_casts);
standard_lint!(macro_use_extern_crate);
standard_lint!(meta_variable_misuse);
standard_lint!(missing_abi);
standard_lint!(missing_copy_implementations);
standard_lint!(missing_debug_implementations);
standard_lint!(missing_docs);
standard_lint!(multiple_supertrait_upcastable);
standard_lint!(must_not_suspend);
// non_ascii_idents is at crate level only
standard_lint!(non_exhaustive_omitted_patterns);
standard_lint!(noop_method_call);
standard_lint!(pointer_structural_match);
standard_lint!(rust_2021_incompatible_closure_captures);
standard_lint!(rust_2021_incompatible_or_patterns);
// rust_2021_prefixes_incompatible_syntax is at crate level only
standard_lint!(rust_2021_prelude_collisions);
standard_lint!(single_use_lifetimes);
standard_lint!(trivial_casts);
standard_lint!(trivial_numeric_casts);
standard_lint!(unreachable_pub);
standard_lint!(unsafe_code);
standard_lint!(unsafe_op_in_unsafe_fn);
// unstable_features is deprecated
// unused_crate_dependencies is at crate level only
standard_lint!(unused_extern_crates);
standard_lint!(unused_import_braces);
standard_lint!(unused_lifetimes);
standard_lint!(unused_macro_rules);
standard_lint!(unused_qualifications);
standard_lint!(unused_results);
standard_lint!(unused_tuple_struct_fields);
standard_lint!(variant_size_differences);

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html:
standard_lint!(ambiguous_glob_reexports);
standard_lint!(anonymous_parameters);
standard_lint!(array_into_iter);
standard_lint!(asm_sub_register);
standard_lint!(bad_asm_style);
standard_lint!(bare_trait_objects);
standard_lint!(break_with_label_and_loop);
standard_lint!(byte_slice_in_packed_struct_with_derive);
standard_lint!(clashing_extern_declarations);
standard_lint!(coherence_leak_check);
// confusable_idents is at crate level only
standard_lint!(const_evaluatable_unchecked);
standard_lint!(const_item_mutation);
standard_lint!(dead_code);
/*
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
standard_lint!();
*/

prefixed_lint!(clippy::assign_ops);
// TODO compile test that the following fails - BUT ONLY with `cargo clippy`
prefixed_lint!(clippy::WRONG_LINT);
