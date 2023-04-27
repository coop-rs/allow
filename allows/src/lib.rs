#![deny(unknown_lints)]
#![cfg_attr(
    unstable_feature, // "unstable_feature" comes from ../build.rs
    feature(
        c_unwind, // https://github.com/rust-lang/rust/issues/74990
        lint_reasons, // https://github.com/rust-lang/rust/issues/54503
        multiple_supertrait_upcastable, // https://doc.rust-lang.org/beta/unstable-book/language-features/multiple-supertrait-upcastable.html
        must_not_suspend, // https://github.com/rust-lang/rust/issues/83310
        non_exhaustive_omitted_patterns_lint, // https://github.com/rust-lang/rust/issues/89554
        strict_provenance, // https://github.com/rust-lang/rust/issues/95228
        test_unstable_lint // https://doc.rust-lang.org/nightly/unstable-book/language-features/test-unstable-lint.html
    )
)]

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[macro_use]
mod wrapper_macros;

/// [`TokenStream`] consisting of one hash character: `#`. It serves as the leading character of the
/// injected code (just left of the injected "[allow(...)]").
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

// @TODO test that e.g. non_existing_std_lint fails TODO compile test that the following fails
// standard_lint!(wrong_lint);

// "top level" inner attribute can't be generated from macros. Hence, we can't handle crate level
// lints. Please give thumbs up to https://github.com/rust-lang/rust/issues/54726.

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/allowed-by-default.html
standard_lint!(absolute_paths_not_starting_with_crate);
standard_lint!(box_pointers);
// elided_lifetimes_in_paths is at crate level only
standard_lint!(explicit_outlives_requirements);
#[rustversion::nightly]
standard_lint!(ffi_unwind_calls);
#[rustversion::nightly]
standard_lint!(fuzzy_provenance_casts);
standard_lint!(keyword_idents);
standard_lint!(let_underscore_drop);
#[rustversion::nightly]
standard_lint!(lossy_provenance_casts);
standard_lint!(macro_use_extern_crate);
standard_lint!(meta_variable_misuse);
standard_lint!(missing_abi);
standard_lint!(missing_copy_implementations);
standard_lint!(missing_debug_implementations);
standard_lint!(missing_docs);
#[rustversion::nightly]
standard_lint!(multiple_supertrait_upcastable);
#[rustversion::nightly]
standard_lint!(must_not_suspend);
// non_ascii_idents is at crate level only
#[rustversion::nightly]
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
// unstable_features is deprecated unused_crate_dependencies is at crate level only
standard_lint!(unused_extern_crates);
standard_lint!(unused_import_braces);
standard_lint!(unused_lifetimes);
standard_lint!(unused_macro_rules);
standard_lint!(unused_qualifications);
standard_lint!(unused_results);
standard_lint!(unused_tuple_struct_fields);
standard_lint!(variant_size_differences);

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html:
#[rustversion::nightly]
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
standard_lint!(deprecated);
standard_lint!(deprecated_where_clause_location);
standard_lint!(deref_into_dyn_supertrait);
standard_lint!(deref_nullptr);
standard_lint!(drop_bounds);
standard_lint!(duplicate_macro_attributes);
standard_lint!(dyn_drop);
standard_lint!(ellipsis_inclusive_range_patterns);
standard_lint!(exported_private_dependencies);
standard_lint!(for_loops_over_fallibles);
standard_lint!(forbidden_lint_groups);
standard_lint!(function_item_references);
standard_lint!(illegal_floating_point_literal_pattern);
standard_lint!(improper_ctypes);
standard_lint!(improper_ctypes_definitions);
standard_lint!(incomplete_features);
standard_lint!(indirect_structural_match);
standard_lint!(inline_no_sanitize);
standard_lint!(invalid_doc_attributes);
// invalid_macro_export_arguments - see https://github.com/rust-lang/rust/issues/110911
standard_lint!(invalid_value);
standard_lint!(irrefutable_let_patterns);
standard_lint!(large_assignments);
standard_lint!(late_bound_lifetime_arguments);
standard_lint!(legacy_derive_helpers);
standard_lint!(map_unit_fn);
// mixed_script_confusables is for crate level only
standard_lint!(named_arguments_used_positionally);
standard_lint!(no_mangle_generic_items);
standard_lint!(non_camel_case_types);
standard_lint!(non_fmt_panics);
standard_lint!(non_shorthand_field_patterns);
standard_lint!(non_snake_case);
standard_lint!(non_upper_case_globals);
standard_lint!(nontrivial_structural_match);
standard_lint!(opaque_hidden_inferred_bound);
standard_lint!(overlapping_range_endpoints);
standard_lint!(path_statements);
standard_lint!(private_in_public);
standard_lint!(redundant_semicolons);
standard_lint!(renamed_and_removed_lints);
standard_lint!(repr_transparent_external_private_fields);
standard_lint!(semicolon_in_expressions_from_macros);
standard_lint!(special_module_name);
standard_lint!(stable_features);
standard_lint!(suspicious_auto_trait_impls);
standard_lint!(temporary_cstring_as_ptr);
standard_lint!(trivial_bounds);
standard_lint!(type_alias_bounds);
standard_lint!(tyvar_behind_raw_pointer);
// uncommon_codepoints is for crate level only
standard_lint!(unconditional_recursion);
// standard_lint!(undefined_naked_function_abi); - https://github.com/rust-lang/rust/issues/110911
#[rustversion::nightly]
standard_lint!(unexpected_cfgs);
#[rustversion::nightly]
standard_lint!(unfulfilled_lint_expectations);
standard_lint!(ungated_async_fn_track_caller);
standard_lint!(uninhabited_static);
standard_lint!(unknown_lints);
standard_lint!(unnameable_test_items);
standard_lint!(unreachable_code);
standard_lint!(unreachable_patterns);
standard_lint!(unstable_name_collisions);
standard_lint!(unstable_syntax_pre_expansion);
standard_lint!(unsupported_calling_conventions);
standard_lint!(unused_allocation);
standard_lint!(unused_assignments);
standard_lint!(unused_attributes);
standard_lint!(unused_braces);
standard_lint!(unused_comparisons);
standard_lint!(unused_doc_comments);
standard_lint!(unused_features);
standard_lint!(unused_imports);
standard_lint!(unused_labels);
standard_lint!(unused_macros);
standard_lint!(unused_must_use);
standard_lint!(unused_mut);
standard_lint!(unused_parens);
standard_lint!(unused_unsafe);
standard_lint!(unused_variables);
standard_lint!(warnings);
standard_lint!(where_clauses_object_safety);
standard_lint!(while_true);

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/deny-by-default.html
standard_lint!(ambiguous_associated_items);
standard_lint!(arithmetic_overflow);
standard_lint!(bindings_with_variant_name);
standard_lint!(cenum_impl_drop_cast);
standard_lint!(conflicting_repr_hints);
standard_lint!(deprecated_cfg_attr_crate_type_name);
standard_lint!(enum_intrinsics_non_enums);
// ill_formed_attribute_input is at crate level only
standard_lint!(implied_bounds_entailment);
standard_lint!(incomplete_include);
standard_lint!(ineffective_unstable_trait_impl);
// standard_lint!(invalid_alignment); https://github.com/rust-lang/rust/issues/110911
standard_lint!(invalid_atomic_ordering);
standard_lint!(invalid_type_param_default);
standard_lint!(let_underscore_lock);
// macro_expanded_macro_exports_accessed_by_absolute_paths  is at crate level only
standard_lint!(missing_fragment_specifier);
standard_lint!(mutable_transmutes);
standard_lint!(named_asm_labels);
standard_lint!(no_mangle_const_items);
standard_lint!(order_dependent_trait_objects);
standard_lint!(overflowing_literals);
standard_lint!(patterns_in_fns_without_body);
standard_lint!(proc_macro_back_compat);
standard_lint!(proc_macro_derive_resolution_fallback);
standard_lint!(pub_use_of_private_extern_crate);
standard_lint!(soft_unstable);
#[rustversion::nightly]
standard_lint!(test_unstable_lint);
standard_lint!(text_direction_codepoint_in_comment);
standard_lint!(text_direction_codepoint_in_literal);
standard_lint!(unconditional_panic);
// unknown_crate_types  is at crate level only
standard_lint!(useless_deprecated);

prefixed_lint!(clippy::assign_ops);
// TODO compile test that the following fails - BUT ONLY with `cargo clippy`
prefixed_lint!(clippy::WRONG_LINT);
