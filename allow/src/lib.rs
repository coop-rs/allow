// We could have `#![forbid(unknown_lints)]` here, be we don't want to. Otherwise it could break
// consumer crates if some lints don't exist anymore (and if `allow` crate itself is not updated
// yet).
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

mod auxiliary;

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
    //let allow_parens_lint_path = TokenStream::from_iter([get_allow(), parens_lint_path]);

    TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Bracket,
        allow_parens_lint_path,
    )))
}

/// NOT for public use. "Used" only by
/// [`allow_internal::generate_allow_attribute_macro_definition_standard`] and
/// [`allow_internal::generate_allow_attribute_macro_definition_prefixed`] macros. Those macros
/// don't invoke this, but instead they generate code that invokes it.
///
/// This generates a definition of a `proc` attribute macro to allow the given lint. The proc macro
/// will have the same name as the given `lint_path`, except that any package-like separators (pairs
/// of colons) :: are replaced with an underscore _.
///
/// Param `lint_path` must NOT contain any whitespace, and it can contain max. one pair of colons
/// `::` (for `clippy::` or `rustdoc::` lints).
#[allow(unused_macros)]
macro_rules! generate_allow_attribute_macro_definition_internal {
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
            // TODO remove if we upgrade Rust min. version, or edition to 2021
            let tokens = [
                $crate::get_hash(),
                $crate::brackets_allow_lint(::allow_internal::path_to_str_literal!($lint_path)),
                item,
            ];
            auxiliary::token_streams_to_stream(&tokens)
            /*::proc_macro::TokenStream::from_iter([
                $crate::get_hash(),
                $crate::brackets_allow_lint(::allow_internal::path_to_str_literal!($lint_path)),
                item,
            ])*/
        }
    };
}

// @TODO test that e.g. non_existing_std_lint fails TODO compile test that the following fails
// standard_lint!(wrong_lint);

// MAINTENANCE NOTE When you edit/add comments below, if you have two (or more) successive comments
// about different lints, either insert a blank line between those comments, or a line with an empty
// `//` comment. That allows us to reformat all comments in VS Code withCtrl+A Alt+Q using
// https://marketplace.visualstudio.com/items?itemName=stkb.rewrap.

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/allowed-by-default.html - in the
// same order:

// absolute_paths_not_starting_with_crate was in edition 2015 only (and we required 2018+).
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

#[rustversion::since(1.52)]
standard_lint!(unsafe_op_in_unsafe_fn); // According to https://github.com/rust-lang/rust/pull/79208 it was stabilized in 1.52.0

// unstable_features is deprecated unused_crate_dependencies is at crate level only
standard_lint!(unused_extern_crates);
standard_lint!(unused_import_braces);
standard_lint!(unused_lifetimes);
standard_lint!(unused_macro_rules);
standard_lint!(unused_qualifications);
standard_lint!(unused_results);
standard_lint!(unused_tuple_struct_fields);
standard_lint!(variant_size_differences);

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html - in the same
// order:
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
// warnings is a group
standard_lint!(where_clauses_object_safety);
standard_lint!(while_true);

// Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/deny-by-default.html - in the same
// order:
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

// Based on https://doc.rust-lang.org/nightly/rustdoc/lints.html - in the same order:

// According to https://releases.rs/docs/1.52.0/#rustdoc rustdoc:: lints exist since 1.52:
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::broken_intra_doc_links);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::private_intra_doc_links);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::missing_docs);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::missing_crate_level_docs);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::missing_doc_code_examples);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::private_doc_tests);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::invalid_codeblock_attributes);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::invalid_html_tags);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::invalid_rust_codeblocks);
#[rustversion::since(1.52)]
prefixed_lint!(rustdoc::bare_urls);

// Based on https://rust-lang.github.io/rust-clippy/index.html for 1.45 to master:
//
// Any clippy:: lint marked as `rustversion::since(1.44.1)` may have existed earlier, too.
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::absurd_extreme_comparisons);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::almost_swapped);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::approx_constant);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::as_conversions);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::assertions_on_constants);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::assign_op_pattern);
// clippy::assign_ops is deprecated since at least 1.45
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::await_holding_lock);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::bad_bit_mask);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::bind_instead_of_map);
// clippy::blacklisted_name has been renamed to `clippy::disallowed_names`
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::blocks_in_if_conditions);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::bool_comparison);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::borrow_interior_mutable_const);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::borrowed_box);
// clippy::box_vec has been renamed to `clippy::box_collection`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::boxed_local);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::builtin_type_shadow);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cargo_common_metadata);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_lossless);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_possible_truncation);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_possible_wrap);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_precision_loss);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_ptr_alignment);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_ref_to_mut);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cast_sign_loss);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::char_lit_as_u8);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::chars_last_cmp);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::chars_next_cmp);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::checked_conversions);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::clone_double_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::clone_on_copy);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::clone_on_ref_ptr);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cmp_nan);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cmp_null);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cmp_owned);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::cognitive_complexity);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::collapsible_if);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::comparison_chain);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::copy_iterator);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::crosspointer_transmute);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::dbg_macro);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::debug_assert_with_mut_call);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::decimal_literal_representation);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::declare_interior_mutable_const);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::default_trait_access);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::deprecated_cfg_attr);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::deprecated_semver);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::deref_addrof);
// clippy::derive_hash_xor_eq has been renamed to `clippy::derived_hash_with_manual_eq`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::diverging_sub_expression);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::doc_markdown);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::double_comparisons);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::double_must_use);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::double_neg);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::double_parens);
// clippy::drop_bounds has been renamed to (prefixless) `drop_bounds`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::drop_copy);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::drop_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::duplicate_underscore_argument);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::duration_subsec);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::else_if_without_else);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::empty_enum);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::empty_line_after_outer_attr);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::empty_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::enum_clike_unportable_variant);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::enum_glob_use);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::enum_variant_names);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::eq_op);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::erasing_op);
// clippy::eval_order_dependence has been renamed to `clippy::mixed_read_write_in_expression`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::excessive_precision);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::exit);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::expect_fun_call);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::expect_used);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::expl_impl_clone_on_copy);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::explicit_counter_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::explicit_deref_methods);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::explicit_into_iter_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::explicit_iter_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::explicit_write);
// clippy::extend_from_slice is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::extra_unused_lifetimes);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::fallible_impl_from);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::filetype_is_file);
// clippy::filter_map has been removed: this lint has been replaced by `manual_filter_map`, a more
// specific (and prefixless) lint.
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::filter_map_next);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::filter_next);
// clippy::find_map has been removed: this lint has been replaced by `manual_find_map`, a more
// specific (and prefixless) lint.
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::flat_map_identity);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::float_arithmetic);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::float_cmp);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::float_cmp_const);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::fn_address_comparisons);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::fn_params_excessive_bools);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::fn_to_numeric_cast);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::fn_to_numeric_cast_with_truncation);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::for_kv_map);
// for_loop_over_option is removed since 1.45
//
// for_loop_over_result is removed since 1.45
//
// clippy::for_loops_over_fallibles has been renamed to (prefixless) `for_loops_over_fallibles`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::forget_copy);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::forget_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::future_not_send);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::get_last_with_len);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::get_unwrap);
// identity_conversion is removed since 1.45
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::identity_op);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::if_let_mutex);
// clippy::if_let_redundant_pattern_matching is deprecated since at least 1.45
//
// clippy::if_let_some_result has been renamed to `clippy::match_result_ok`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::if_not_else);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::if_same_then_else);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::ifs_same_cond);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::implicit_hasher);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::implicit_return);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::implicit_saturating_sub);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::imprecise_flops);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::inconsistent_digit_grouping);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::indexing_slicing);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::ineffective_bit_mask);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::inefficient_to_string);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::infallible_destructuring_match);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::infinite_iter);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::inherent_to_string);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::inherent_to_string_shadow_display);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::inline_always);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::inline_fn_without_body);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::int_plus_one);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::integer_arithmetic);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::integer_division);
// clippy::into_iter_on_array is deprecated since at least 1.45
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::into_iter_on_ref);
// clippy::invalid_atomic_ordering has been renamed to (prefixless) `invalid_atomic_ordering`
//
// clippy::invalid_ref is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::invalid_regex);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::invalid_upcast_comparisons);
#[rustversion::since(1.49)]
prefixed_lint!(clippy::invisible_characters);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::items_after_statements);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::iter_cloned_collect);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::iter_next_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::iter_nth);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::iter_nth_zero);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::iter_skip_next);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::iterator_step_by_zero);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::just_underscores_and_digits);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::large_const_arrays);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::large_digit_groups);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::large_enum_variant);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::large_stack_arrays);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::len_without_is_empty);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::len_zero);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::let_and_return);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::let_underscore_lock);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::let_underscore_must_use);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::let_unit_value);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::linkedlist);
// clippy::logic_bug has been renamed to `clippy::overly_complex_bool_expr`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::lossy_float_literal);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::macro_use_imports);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::main_recursion);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::manual_async_fn);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::manual_memcpy);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::manual_non_exhaustive);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::manual_saturating_arithmetic);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::manual_swap);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::many_single_char_names);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::map_clone);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::map_entry);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::map_flatten);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::map_unwrap_or);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_as_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_bool);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::match_on_vec_items);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_overlapping_arm);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_ref_pats);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_same_arms);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_single_binding);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::match_wild_err_arm);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::match_wildcard_for_single_variants);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::maybe_infinite_iter);
// clippy::mem_discriminant_non_enum has been renamed to (prefixless) `enum_intrinsics_non_enums`
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mem_forget);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mem_replace_option_with_none);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mem_replace_with_default);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mem_replace_with_uninit);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::min_max);
// clippy::misaligned_transmute is deprecated since at least 1.44.1
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::mismatched_target_os);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::misrefactored_assign_op);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::missing_const_for_fn);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::missing_docs_in_private_items);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::missing_errors_doc);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::missing_inline_in_public_items);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::missing_safety_doc);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mistyped_literal_suffixes);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mixed_case_hex_literals);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::module_inception);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::module_name_repetitions);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::modulo_arithmetic);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::modulo_one);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::multiple_crate_versions);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::multiple_inherent_impl);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::must_use_candidate);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::must_use_unit);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mut_from_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mut_mut);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mut_range_bound);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mutable_key_type);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mutex_atomic);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::mutex_integer);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::naive_bytecount);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_bool);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_borrow);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_borrowed_reference);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_collect);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_continue);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_doctest_main);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_lifetimes);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_pass_by_value);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_range_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_return);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::needless_update);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::neg_cmp_op_on_partial_ord);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::neg_multiply);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::never_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::new_ret_no_self);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::new_without_default);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::no_effect);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::non_ascii_literal);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::nonminimal_bool);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::nonsensical_open_options);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::not_unsafe_ptr_arg_deref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::ok_expect);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::op_ref);
// option_and_then_some is removed since 1.45.0 TODO check
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::option_as_ref_deref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::option_env_unwrap);
// option_expect_used is removed since 1.45.0 TODO check
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::option_map_or_none);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::option_map_unit_fn);
// option_map_unwrap_or is removed since 1.45.0 TODO check
//
// option_mapw_unwrap_or_else is removed since 1.45.0 TODO check
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::option_option);
// option_unwrap_used is removed since 1.45.0 TODO check
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::or_fun_call);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::out_of_bounds_indexing);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::overflow_check_conditional);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::panic);
// clippy::panic_params has been renamed to (prefixless)
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::panicking_unwrap);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::partialeq_ne_impl);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::path_buf_push_overwrite);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::possible_missing_comma);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::precedence);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::print_literal);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::print_stdout);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::print_with_newline);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::println_empty_string);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::ptr_arg);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::ptr_offset_with_cast);
// clippy::pub_enum_variant_names has been removed. set the `avoid-breaking-exported-api` config
// option to `false` to enable the `enum_variant_names` lint for public items. (Probably a
// prefixless lint; `allow` crate doesn't support attribute parameters.)
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::question_mark);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::range_minus_one);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::range_plus_one);
// clippy::range_step_by_zero is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::range_zip_with_len);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_allocation);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_clone);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_closure);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_closure_call);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_closure_for_method_calls);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_field_names);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_pattern);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_pattern_matching);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_pub_crate);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::redundant_static_lifetimes);
// clippy::ref_in_deref has been renamed to clippy::needless_borrow
//
// clippy::regex_macro has been removed
//
// clippy::replace_consts is deprecated since 1.45
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::rest_pat_in_fully_bound_structs);
// result_expect_used has been removed since 1.45 TODO check
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::result_map_or_into_option);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::result_map_unit_fn);
// result_map_unwrap_or_else is removed since 1.45 TODO check
//
// result_unwrap_used is removed since 1.45 TODO check
//
// reverse_range_loop is removed since 1.45 TODO check
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::reversed_empty_ranges);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::same_functions_in_if_condition);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::search_is_some);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::serde_api_misuse);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::shadow_reuse);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::shadow_same);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::shadow_unrelated);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::short_circuit_statement);
// clippy::should_assert_eq is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::should_implement_trait);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::similar_names);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::single_char_pattern);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::single_component_path_imports);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::single_match);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::single_match_else);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::skip_while_next);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::slow_vector_initialization);
// clippy::str_to_string is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::string_add);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::string_add_assign);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::string_extend_chars);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::string_lit_as_bytes);
// clippy::string_to_string is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::struct_excessive_bools);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suboptimal_flops);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suspicious_arithmetic_impl);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suspicious_assignment_formatting);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suspicious_else_formatting);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suspicious_map);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suspicious_op_assign_impl);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::suspicious_unary_op_formatting);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::tabs_in_doc_comments);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::temporary_assignment);
#[rustversion::since(1.44.1)]
// clippy::temporary_cstring_as_ptr is renamed to (prefixless) temporary_cstring_as_ptr
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::to_digit_is_some);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::todo);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::too_many_arguments);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::too_many_lines);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::toplevel_ref_arg);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_bytes_to_str);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_float_to_int);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_int_to_bool);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_int_to_char);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_int_to_float);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_ptr_to_ptr);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmute_ptr_to_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::transmuting_null);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::trivial_regex);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::trivially_copy_pass_by_ref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::try_err);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::type_complexity);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::type_repetition_in_bounds);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unicode_not_nfc);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unimplemented);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::uninit_assumed_init);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unit_arg);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unit_cmp);
// clippy::unknown_clippy_lints is renamed to (prefixless rustc lint) unknown_lints
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unnecessary_cast);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unnecessary_filter_map);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unnecessary_fold);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unnecessary_mut_passed);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unnecessary_operation);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unnecessary_unwrap);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unneeded_field_pattern);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unneeded_wildcard_pattern);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unreachable);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unreadable_literal);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::unsafe_derive_deserialize);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unsafe_removed_from_name);
// clippy::unsafe_vector_initialization is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unseparated_literal_suffix);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unsound_collection_transmute);
// clippy::unstable_as_mut_slice is deprecated since at least 1.44.1
//
// clippy::unstable_as_slice is deprecated since at least 1.44.1
//
// clippy::unused_collect is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unused_io_amount);
// clippy::unused_label is deprecated since at least 1.44.1
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unused_self);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::unused_unit);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::unwrap_used);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::use_debug);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::use_self);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::used_underscore_binding);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::useless_asref);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::useless_attribute);
#[rustversion::since(1.45.0)]
prefixed_lint!(clippy::useless_conversion);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::useless_format);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::useless_let_if_seq);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::useless_transmute);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::useless_vec);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::vec_box);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::verbose_bit_mask);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::verbose_file_reads);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::vtable_address_comparisons);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::while_immutable_condition);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::while_let_loop);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::while_let_on_iterator);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::wildcard_dependencies);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::wildcard_enum_match_arm);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::wildcard_imports);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::wildcard_in_or_patterns);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::write_literal);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::write_with_newline);
// clippy::writeln_empty_string is removed: set the `avoid-breaking-exported-api` config option to
// `false` to enable the `wrong_self_convention` lint for public items. (Probably a prefixless lint;
// `allow` crate doesn't support attribute parameters.)
//
// clippy::wrong_pub_self_convention has been removed: set the `avoid-breaking-exported-api` config
// option to `false` to enable the `wrong_self_convention` lint for public items. (Probably a
// prefixless lint; `allow` crate doesn't support attribute parameters.)
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::wrong_self_convention);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::wrong_transmute);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::zero_divided_by_zero);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::zero_prefixed_literal);
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::zero_ptr);
// clippy::zero_width_space renamed in 1.49 to clippy::invisible_characters
#[rustversion::since(1.44.1)]
prefixed_lint!(clippy::zst_offset);

// TODO compile test that the following fails - BUT ONLY with `cargo clippy`
// prefixed_lint!(clippy::WRONG_LINT);
