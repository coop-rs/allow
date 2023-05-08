//! Alias/label lints (to disable) with your intentions.
//!
//! Implementation of 'allow' crate, exported with no prefix (for prefixless lints), and with
//! `clippy_` and `rustoc_` prefixes.
// We can't have `#![forbid(unknown_lints)]` here, because it gets passed to `#[allow(...)]` in
// `standard_lint!(...)` as a part of `standard_lint!`'s internal check. That would then fail (under
// outer `#![forbid(unknown_lints)]`). We used to support that by having a special branch in
// `standard_lint!` macro for `unknown_lints` itself, but that could introduce a human mistake.
//
// Also, it would break consumer crates when some lints wouldn't exist anymore, or if there were a
// mistake in specifying Rust version ranges for specific lint macros in `allow_prefixed`.
//
// Instead of `#[forbid(unknown_lints)]` here, we have it in tests.
#![deny(unknown_lints)]
#![cfg_attr(has_rustdoc_lints, deny(rustdoc::missing_docs))]
#![cfg_attr(can_check_doc_attributes, deny(invalid_doc_attributes))]
#![deny(unused_doc_comments)]
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

#[macro_use]
mod wrapper_macros;

mod auxiliary;

mod proc_builder;

/* // @TODO
// When panic_old_on_floating_toolchain` feature is turned on, then our proc macros generate
// `#[deprecated = panic!("...")]` - NOT in the definition of the proc macro (before its `fn`), but
// in the code generated.
mod restrict_floating_toolchain_pof {
    macro_rules! generate_code_as_if_from_proc_macro {
        () => {
            //#[forbid(deprecated)] // This `forbid` has no effect on the consumer!
            //#[deprecated(note = "Deprecated or removed, AND on a floating toolchain, AND restrict_floating_toolchain.")]
            #[deprecated = panic!("PANI")]
            #[allow(dead_code)]
            fn f() {}
        };
    }
}*/

/// NOT for public use. See [generate_allow_attribute_macro_definition_internal].
/// 
/// $doc is used for rustdoc of the generated proc macro; it must be an `&str`-like literal or
/// expression - for example, a result of `stringify!`
#[allow(unused_macros)]
macro_rules! generate_allow_attribute_macro_definition_internal_without_docs {
    ( $lint_path:path, $new_macro_name:ident, $doc:expr ) => {
        #[doc = $doc]
        #[proc_macro_attribute]
        pub fn $new_macro_name(
            given_attrs: ::proc_macro::TokenStream,
            item: ::proc_macro::TokenStream,
        ) -> ::proc_macro::TokenStream {
            // Clippy lints that have configuration (few of them) don't accept the config values as
            // any attribute parameters. See
            // https://doc.rust-lang.org/nightly/clippy/configuration.html.
            assert!(
                given_attrs.is_empty(),
                "Do not pass any attribute parameters."
            );
            // TODO replace with the below if we upgrade Rust min. version, or edition to 2021
            let tokens = [
                $crate::proc_builder::get_hash(),
                $crate::proc_builder::brackets_allow_lint(::allow_internal::path_to_str_literal!(
                    $lint_path
                )),
                item,
            ];
            auxiliary::token_streams_to_stream(&tokens)
            /*::proc_macro::TokenStream::from_iter([
                $crate::proc_builder::get_hash(),
                $crate::proc_builder::brackets_allow_lint(::allow_internal::path_to_str_literal!($lint_path)),
                item,
            ])*/
        }
    };
}

/// NOT for public use. "Used" only by
/// [`allow_internal::generate_allow_attribute_macro_definition_standard`] and
/// [`allow_internal::generate_allow_attribute_macro_definition_prefixed`] macros. Those macros
/// don't invoke this, but instead they generate code that invokes it.
///
/// This generates a definition of a `proc` attribute macro to allow (suppress a warning for) the
/// given lint. The proc macro will have the same name as the given `lint_path`, except that any
/// package-like separators (pairs of colons) :: are replaced with an underscore _.
///
/// Param `lint_path` must NOT contain any whitespace, and it can contain max. one pair of colons
/// `::` (for `clippy::` or `rustdoc::` lints).
#[cfg(attributes_can_invoke_macros)]
macro_rules! generate_allow_attribute_macro_definition_internal {
    ( $lint_path:path, $new_macro_name:ident ) => {
        generate_allow_attribute_macro_definition_internal_without_docs!($lint_path, $new_macro_name, stringify!(Alias to #allow($lint_path).));
    };
}
#[cfg(not(attributes_can_invoke_macros))]
macro_rules! generate_allow_attribute_macro_definition_internal {
    ( $lint_path:path, $new_macro_name:ident ) => {
        generate_allow_attribute_macro_definition_internal_without_docs!(
            $lint_path,
            $new_macro_name,
            // If you change the following doc, update the copy in ../../README.md.
            "Alias to `#[allow(...)]` a lint with a similar name as imported from allow or allow_prefixed."
        );
    };
}

// @TODO test that e.g. non_existing_std_lint fails TODO compile test that the following fails
// standard_lint!(wrong_lint);

// MAINTENANCE NOTES
//
// 1. When you edit/add comments below, if you have two (or more) successive comments about
//    different lints, either insert a blank line between those comments, or a line with an empty
//    `//` comment. That allows us to reformat all comments in VS Code withCtrl+A Alt+Q using
//    https://marketplace.visualstudio.com/items?itemName=stkb.rewrap.

// 1. Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/allowed-by-default.html

// absolute_paths_not_starting_with_crate was in edition 2015 only (and we require 2018+).
//#[rustversion::since(1.45.0)] standard_lint!(box_pointers);
standard_lint!(box_pointers);
// elided_lifetimes_in_paths - at crate level only
standard_lint!(explicit_outlives_requirements);
standard_lint_nightly!(ffi_unwind_calls);
standard_lint_nightly!(fuzzy_provenance_casts);
standard_lint!(keyword_idents);
standard_lint!(let_underscore_drop);
standard_lint_nightly!(lossy_provenance_casts);
standard_lint!(macro_use_extern_crate);
standard_lint!(meta_variable_misuse);
standard_lint!(missing_abi);
standard_lint!(missing_copy_implementations);
standard_lint!(missing_debug_implementations);
standard_lint!(missing_docs);
standard_lint_nightly!(multiple_supertrait_upcastable);
standard_lint_nightly!(must_not_suspend);
// non_ascii_idents - at crate level only
standard_lint_nightly!(non_exhaustive_omitted_patterns);
standard_lint!(noop_method_call);
standard_lint!(pointer_structural_match);
standard_lint!(rust_2021_incompatible_closure_captures);
standard_lint!(rust_2021_incompatible_or_patterns);
// rust_2021_prefixes_incompatible_syntax - at crate level only
standard_lint!(rust_2021_prelude_collisions);
standard_lint!(single_use_lifetimes);
standard_lint!(trivial_casts);
standard_lint!(trivial_numeric_casts);
standard_lint!(unreachable_pub);
standard_lint!(unsafe_code);

standard_lint_versioned!(1.52, unsafe_op_in_unsafe_fn); // According to https://github.com/rust-lang/rust/pull/79208 it was stabilized in 1.52.0

// unstable_features - deprecated
//
// unused_crate_dependencies - at crate level only
standard_lint!(unused_extern_crates);
standard_lint!(unused_import_braces);
standard_lint!(unused_lifetimes);
standard_lint!(unused_macro_rules);
standard_lint!(unused_qualifications);
standard_lint!(unused_results);
standard_lint!(unused_tuple_struct_fields);
standard_lint!(variant_size_differences);

// 2. Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html
standard_lint_nightly!(ambiguous_glob_reexports);
standard_lint!(anonymous_parameters);
standard_lint!(array_into_iter);
standard_lint!(asm_sub_register);
standard_lint!(bad_asm_style);
standard_lint!(bare_trait_objects);
standard_lint!(break_with_label_and_loop);
standard_lint!(byte_slice_in_packed_struct_with_derive);
standard_lint!(clashing_extern_declarations);
standard_lint!(coherence_leak_check);
// confusable_idents - at crate level only
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
standard_lint_nightly!(invalid_macro_export_arguments);
standard_lint!(invalid_value);
standard_lint!(irrefutable_let_patterns);
standard_lint!(large_assignments);
standard_lint!(late_bound_lifetime_arguments);
standard_lint!(legacy_derive_helpers);
standard_lint!(map_unit_fn);
// mixed_script_confusables - at crate level only
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
// uncommon_codepoints - at crate level only
standard_lint!(unconditional_recursion);
standard_lint_nightly!(undefined_naked_function_abi);
standard_lint_nightly!(unexpected_cfgs);
standard_lint_nightly!(unfulfilled_lint_expectations);
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

// 3. Based on https://doc.rust-lang.org/nightly/rustc/lints/listing/deny-by-default.html - in the
// same order:
standard_lint!(ambiguous_associated_items);
standard_lint!(arithmetic_overflow);
standard_lint!(bindings_with_variant_name);
standard_lint!(cenum_impl_drop_cast);
standard_lint!(conflicting_repr_hints);
standard_lint!(deprecated_cfg_attr_crate_type_name);
standard_lint!(enum_intrinsics_non_enums);
// ill_formed_attribute_input - at crate level only
standard_lint!(implied_bounds_entailment);
standard_lint!(incomplete_include);
standard_lint!(ineffective_unstable_trait_impl);
standard_lint_nightly!(invalid_alignment);
standard_lint!(invalid_atomic_ordering);
standard_lint!(invalid_type_param_default);
standard_lint!(let_underscore_lock);
// macro_expanded_macro_exports_accessed_by_absolute_paths - at crate level only
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
standard_lint_nightly!(test_unstable_lint);
standard_lint!(text_direction_codepoint_in_comment);
standard_lint!(text_direction_codepoint_in_literal);
standard_lint!(unconditional_panic);
// unknown_crate_types - at crate level only
standard_lint!(useless_deprecated);

// Based on https://doc.rust-lang.org/nightly/rustdoc/lints.html - in the same order:

// According to https://releases.rs/docs/1.52.0/#rustdoc rustdoc:: lints exist since 1.52:
prefixed_lint_versioned!(1.52, rustdoc::broken_intra_doc_links);
prefixed_lint_versioned!(1.52, rustdoc::private_intra_doc_links);
prefixed_lint_versioned!(1.52, rustdoc::missing_docs);
prefixed_lint_versioned!(1.52, rustdoc::missing_crate_level_docs);
prefixed_lint_versioned!(1.52, rustdoc::missing_doc_code_examples);
prefixed_lint_versioned!(1.52, rustdoc::private_doc_tests);
prefixed_lint_versioned!(1.52, rustdoc::invalid_codeblock_attributes);
prefixed_lint_versioned!(1.52, rustdoc::invalid_html_tags);
prefixed_lint_versioned!(1.52, rustdoc::invalid_rust_codeblocks);
prefixed_lint_versioned!(1.52, rustdoc::bare_urls);
prefixed_lint_versioned!(1.52, rustdoc::unescaped_backticks);

// Based on https://rust-lang.github.io/rust-clippy/index.html for 1.45 to master for nightly 1.71:
prefixed_lint!(clippy::absurd_extreme_comparisons);

prefixed_lint_versioned!(1.64, clippy::alloc_instead_of_core);
prefixed_lint_versioned!(1.69, clippy::allow_attributes);
prefixed_lint_versioned!(1.61, clippy::allow_attributes_without_reason);
prefixed_lint_versioned!(1.68, clippy::almost_complete_range);
prefixed_lint!(clippy::almost_swapped);
prefixed_lint!(clippy::approx_constant);
prefixed_lint_versioned!(1.64, clippy::arithmetic_side_effects);
prefixed_lint!(clippy::as_conversions);
prefixed_lint_versioned!(1.66, clippy::as_ptr_cast_mut);
prefixed_lint_versioned!(1.63, clippy::as_underscore);
prefixed_lint!(clippy::assertions_on_constants);
prefixed_lint_versioned!(1.64, clippy::assertions_on_result_states);
prefixed_lint!(clippy::assign_op_pattern);
// clippy::assign_ops is deprecated since at least 1.45
prefixed_lint_versioned!(1.48, clippy::async_yields_async);
prefixed_lint_versioned!(1.62, clippy::await_holding_invalid_type);
prefixed_lint!(clippy::await_holding_lock);
prefixed_lint_versioned!(1.49, clippy::await_holding_refcell_ref);
prefixed_lint!(clippy::bad_bit_mask);
prefixed_lint!(clippy::bind_instead_of_map);
// clippy::blacklisted_name has been renamed to `clippy::disallowed_names`
prefixed_lint_versioned!(1.47, clippy::blanket_clippy_restriction_lints);
prefixed_lint!(clippy::blocks_in_if_conditions);
prefixed_lint_versioned!(1.53, clippy::bool_assert_comparison);
prefixed_lint!(clippy::bool_comparison);
prefixed_lint_versioned!(1.65, clippy::bool_to_int_with_if);
prefixed_lint_versioned!(1.60, clippy::borrow_as_ptr);
prefixed_lint_versioned!(1.63, clippy::borrow_deref_ref);
prefixed_lint!(clippy::borrow_interior_mutable_const);
prefixed_lint!(clippy::borrowed_box);
prefixed_lint_versioned!(1.57, clippy::box_collection);
prefixed_lint_versioned!(1.66, clippy::box_default);
// clippy::box_vec has been renamed to `clippy::box_collection`
prefixed_lint!(clippy::boxed_local);
prefixed_lint_versioned!(1.53, clippy::branches_sharing_code);
prefixed_lint!(clippy::builtin_type_shadow);
prefixed_lint_versioned!(1.62, clippy::bytes_count_to_len);
prefixed_lint_versioned!(1.52, clippy::bytes_nth);
prefixed_lint!(clippy::cargo_common_metadata);
prefixed_lint_versioned!(1.51, clippy::case_sensitive_file_extension_comparisons);
prefixed_lint_versioned!(1.62, clippy::cast_abs_to_unsigned);
prefixed_lint_versioned!(1.61, clippy::cast_enum_constructor);
prefixed_lint_versioned!(1.61, clippy::cast_enum_truncation);
prefixed_lint!(clippy::cast_lossless);
prefixed_lint_versioned!(1.66, clippy::cast_nan_to_int);
prefixed_lint!(clippy::cast_possible_truncation);
prefixed_lint!(clippy::cast_possible_wrap);
prefixed_lint!(clippy::cast_precision_loss);
prefixed_lint!(clippy::cast_ptr_alignment);
prefixed_lint!(clippy::cast_ref_to_mut);
prefixed_lint!(clippy::cast_sign_loss);
prefixed_lint_versioned!(1.61, clippy::cast_slice_different_sizes);
prefixed_lint_versioned!(1.65, clippy::cast_slice_from_raw_parts);
prefixed_lint!(clippy::char_lit_as_u8);
prefixed_lint!(clippy::chars_last_cmp);
prefixed_lint!(clippy::chars_next_cmp);
prefixed_lint!(clippy::checked_conversions);
prefixed_lint_versioned!(1.69, clippy::clear_with_drain);
prefixed_lint!(clippy::clone_double_ref);
prefixed_lint!(clippy::clone_on_copy);
prefixed_lint!(clippy::clone_on_ref_ptr);
prefixed_lint!(clippy::cloned_instead_of_copied);
prefixed_lint!(clippy::cmp_nan);
prefixed_lint!(clippy::cmp_null);
prefixed_lint!(clippy::cmp_owned);
prefixed_lint!(clippy::cognitive_complexity);
prefixed_lint_versioned!(1.51, clippy::collapsible_else_if);
prefixed_lint!(clippy::collapsible_if);
prefixed_lint_versioned!(1.50, clippy::collapsible_match);
prefixed_lint_versioned!(1.65, clippy::collapsible_str_replace);
prefixed_lint_versioned!(1.69, clippy::collection_is_never_read);
prefixed_lint!(clippy::comparison_chain);
prefixed_lint_versioned!(1.49, clippy::comparison_to_empty);
prefixed_lint!(clippy::copy_iterator);
prefixed_lint_versioned!(1.62, clippy::crate_in_macro_def);
prefixed_lint_versioned!(1.48, clippy::create_dir);
prefixed_lint!(clippy::crosspointer_transmute);
prefixed_lint!(clippy::dbg_macro);
prefixed_lint!(clippy::debug_assert_with_mut_call);
prefixed_lint!(clippy::decimal_literal_representation);
prefixed_lint!(clippy::declare_interior_mutable_const);
prefixed_lint_versioned!(1.64, clippy::default_instead_of_iter_empty);
prefixed_lint_versioned!(1.52, clippy::default_numeric_fallback);
prefixed_lint!(clippy::default_trait_access);
prefixed_lint_versioned!(1.60, clippy::default_union_representation);
prefixed_lint!(clippy::deprecated_cfg_attr);
prefixed_lint!(clippy::deprecated_semver);
prefixed_lint!(clippy::deref_addrof);
prefixed_lint_versioned!(1.61, clippy::deref_by_slicing);
prefixed_lint_versioned!(1.57, clippy::derivable_impls);
// clippy::derive_hash_xor_eq has been renamed to `clippy::derived_hash_with_manual_eq`
prefixed_lint_versioned!(1.47, clippy::derive_ord_xor_partial_ord);
prefixed_lint_versioned!(1.63, clippy::derive_partial_eq_without_eq);
prefixed_lint!(clippy::derived_hash_with_manual_eq);
prefixed_lint_versioned!(1.66, clippy::disallowed_macros);
prefixed_lint_versioned!(1.49, clippy::disallowed_methods);
prefixed_lint!(clippy::disallowed_names);
prefixed_lint_versioned!(1.55, clippy::disallowed_script_idents);
prefixed_lint_versioned!(1.55, clippy::disallowed_types);
prefixed_lint!(clippy::diverging_sub_expression);
prefixed_lint_versioned!(1.63, clippy::doc_link_with_quotes);
prefixed_lint!(clippy::doc_markdown);
prefixed_lint!(clippy::double_comparisons);
prefixed_lint!(clippy::double_must_use);
prefixed_lint!(clippy::double_neg);
prefixed_lint!(clippy::double_parens);
// clippy::drop_bounds has been renamed to (prefixless) `drop_bounds`
prefixed_lint!(clippy::drop_copy);
prefixed_lint_versioned!(1.62, clippy::drop_non_drop);
prefixed_lint!(clippy::drop_ref);
prefixed_lint_versioned!(1.63, clippy::duplicate_mod);
prefixed_lint!(clippy::duplicate_underscore_argument);
prefixed_lint!(clippy::duration_subsec);
prefixed_lint!(clippy::else_if_without_else);
prefixed_lint_versioned!(1.62, clippy::empty_drop);
prefixed_lint!(clippy::empty_enum);
prefixed_lint!(clippy::empty_line_after_outer_attr);
prefixed_lint!(clippy::empty_loop);
prefixed_lint_versioned!(1.62, clippy::empty_structs_with_brackets);
prefixed_lint!(clippy::enum_clike_unportable_variant);
prefixed_lint!(clippy::enum_glob_use);
prefixed_lint!(clippy::enum_variant_names);
prefixed_lint!(clippy::eq_op);
prefixed_lint_versioned!(1.57, clippy::equatable_if_let);
prefixed_lint!(clippy::erasing_op);
prefixed_lint_versioned!(1.62, clippy::err_expect);
// clippy::eval_order_dependence has been renamed to `clippy::mixed_read_write_in_expression`
prefixed_lint!(clippy::excessive_precision);
prefixed_lint_versioned!(1.51, clippy::exhaustive_enums);
prefixed_lint_versioned!(1.51, clippy::exhaustive_structs);
prefixed_lint!(clippy::exit);
prefixed_lint!(clippy::expect_fun_call);
prefixed_lint!(clippy::expect_used);
prefixed_lint!(clippy::expl_impl_clone_on_copy);
prefixed_lint_versioned!(1.64, clippy::explicit_auto_deref);
prefixed_lint!(clippy::explicit_counter_loop);
prefixed_lint!(clippy::explicit_deref_methods);
prefixed_lint!(clippy::explicit_into_iter_loop);
prefixed_lint!(clippy::explicit_iter_loop);
prefixed_lint!(clippy::explicit_write);
// clippy::extend_from_slice is deprecated since at least 1.44.1
prefixed_lint_versioned!(1.55, clippy::extend_with_drain);
prefixed_lint!(clippy::extra_unused_lifetimes);
prefixed_lint_versioned!(1.69, clippy::extra_unused_type_parameters);
prefixed_lint!(clippy::fallible_impl_from);
prefixed_lint_versioned!(1.49, clippy::field_reassign_with_default);
prefixed_lint!(clippy::filetype_is_file);
// clippy::filter_map has been removed: this lint has been replaced by `manual_filter_map`, a more
// specific (and prefixless) lint.
prefixed_lint_versioned!(1.52, clippy::filter_map_identity);
prefixed_lint!(clippy::filter_map_next);
prefixed_lint!(clippy::filter_next);
// clippy::find_map has been removed: this lint has been replaced by `manual_find_map`, a more
// specific (and prefixless) lint.
prefixed_lint!(clippy::flat_map_identity);
prefixed_lint_versioned!(1.53, clippy::flat_map_option);
prefixed_lint!(clippy::float_arithmetic);
prefixed_lint!(clippy::float_cmp);
prefixed_lint!(clippy::float_cmp_const);
prefixed_lint_versioned!(1.48, clippy::float_equality_without_abs);
prefixed_lint!(clippy::fn_address_comparisons);
prefixed_lint_versioned!(1.68, clippy::fn_null_check);
prefixed_lint!(clippy::fn_params_excessive_bools);
prefixed_lint!(clippy::fn_to_numeric_cast);
prefixed_lint_versioned!(1.58, clippy::fn_to_numeric_cast_any);
prefixed_lint!(clippy::fn_to_numeric_cast_with_truncation);
prefixed_lint!(clippy::for_kv_map);
// for_loop_over_option is removed since 1.45
//
// for_loop_over_result is removed since 1.45
//
// clippy::for_loops_over_fallibles has been renamed to (prefixless) `for_loops_over_fallibles`
prefixed_lint!(clippy::forget_copy);
prefixed_lint_versioned!(1.62, clippy::forget_non_drop);
prefixed_lint!(clippy::forget_ref);
prefixed_lint_versioned!(1.58, clippy::format_in_format_args);
prefixed_lint_versioned!(1.62, clippy::format_push_string);
prefixed_lint_versioned!(1.49, clippy::from_iter_instead_of_collect);
prefixed_lint_versioned!(1.51, clippy::from_over_into);
prefixed_lint_versioned!(1.67, clippy::from_raw_with_void_ptr);
prefixed_lint_versioned!(1.52, clippy::from_str_radix_10);
prefixed_lint!(clippy::future_not_send);
prefixed_lint_versioned!(1.63, clippy::get_first);
prefixed_lint!(clippy::get_last_with_len);
prefixed_lint!(clippy::get_unwrap);
// identity_conversion is removed since 1.45
prefixed_lint!(clippy::identity_op);
prefixed_lint!(clippy::if_let_mutex);
// clippy::if_let_redundant_pattern_matching is deprecated since at least 1.45
//
// clippy::if_let_some_result has been renamed to `clippy::match_result_ok`
prefixed_lint!(clippy::if_not_else);
prefixed_lint!(clippy::if_same_then_else);
prefixed_lint_versioned!(1.53, clippy::if_then_some_else_none);
prefixed_lint!(clippy::ifs_same_cond);
prefixed_lint_versioned!(1.69, clippy::impl_trait_in_params);
prefixed_lint_versioned!(1.52, clippy::implicit_clone);
prefixed_lint!(clippy::implicit_hasher);
prefixed_lint!(clippy::implicit_return);
prefixed_lint_versioned!(1.66, clippy::implicit_saturating_add);
prefixed_lint!(clippy::implicit_saturating_sub);
prefixed_lint!(clippy::imprecise_flops);
prefixed_lint!(clippy::inconsistent_digit_grouping);
prefixed_lint_versioned!(1.52, clippy::inconsistent_struct_constructor);
prefixed_lint_versioned!(1.59, clippy::index_refutable_slice);
prefixed_lint!(clippy::indexing_slicing);
prefixed_lint!(clippy::ineffective_bit_mask);
prefixed_lint!(clippy::inefficient_to_string);
prefixed_lint!(clippy::infallible_destructuring_match);
prefixed_lint!(clippy::infinite_iter);
prefixed_lint!(clippy::inherent_to_string);
prefixed_lint!(clippy::inherent_to_string_shadow_display);
prefixed_lint_versioned!(1.59, clippy::init_numbered_fields);
prefixed_lint!(clippy::inline_always);
prefixed_lint_versioned!(1.49, clippy::inline_asm_x86_att_syntax);
prefixed_lint_versioned!(1.49, clippy::inline_asm_x86_intel_syntax);
prefixed_lint!(clippy::inline_fn_without_body);
prefixed_lint_versioned!(1.51, clippy::inspect_for_each);
prefixed_lint!(clippy::int_plus_one);
prefixed_lint!(clippy::integer_arithmetic);
prefixed_lint!(clippy::integer_division);
// clippy::into_iter_on_array is deprecated since at least 1.45
prefixed_lint!(clippy::into_iter_on_ref);
// clippy::invalid_atomic_ordering has been renamed to (prefixless) `invalid_atomic_ordering`
prefixed_lint_versioned!(1.53, clippy::invalid_null_ptr_usage);
// clippy::invalid_ref is deprecated since at least 1.44.1
prefixed_lint!(clippy::invalid_regex);
prefixed_lint!(clippy::invalid_upcast_comparisons);
prefixed_lint_versioned!(1.64, clippy::invalid_utf8_in_unchecked);
prefixed_lint_versioned!(1.49, clippy::invisible_characters);
prefixed_lint_versioned!(1.62, clippy::is_digit_ascii_radix);
prefixed_lint!(clippy::items_after_statements);
prefixed_lint_versioned!(1.70, clippy::items_after_test_module);
prefixed_lint!(clippy::iter_cloned_collect);
prefixed_lint_versioned!(1.52, clippy::iter_count);
prefixed_lint_versioned!(1.66, clippy::iter_kv_map);
prefixed_lint!(clippy::iter_next_loop);
prefixed_lint_versioned!(1.46, clippy::iter_next_slice);
prefixed_lint_versioned!(1.57, clippy::iter_not_returning_iterator);
prefixed_lint!(clippy::iter_nth);
prefixed_lint!(clippy::iter_nth_zero);
prefixed_lint_versioned!(1.65, clippy::iter_on_empty_collections);
prefixed_lint_versioned!(1.65, clippy::iter_on_single_items);
prefixed_lint_versioned!(1.60, clippy::iter_overeager_cloned);
prefixed_lint!(clippy::iter_skip_next);
prefixed_lint_versioned!(1.61, clippy::iter_with_drain);
prefixed_lint!(clippy::iterator_step_by_zero);
prefixed_lint!(clippy::just_underscores_and_digits);
prefixed_lint!(clippy::large_const_arrays);
prefixed_lint!(clippy::large_digit_groups);
prefixed_lint!(clippy::large_enum_variant);
prefixed_lint_versioned!(1.68, clippy::large_futures);
prefixed_lint_versioned!(1.62, clippy::large_include_file);
prefixed_lint!(clippy::large_stack_arrays);
prefixed_lint_versioned!(1.49, clippy::large_types_passed_by_value);
prefixed_lint!(clippy::len_without_is_empty);
prefixed_lint!(clippy::len_zero);
prefixed_lint!(clippy::let_and_return);
prefixed_lint_versioned!(1.67, clippy::let_underscore_future);
prefixed_lint!(clippy::let_underscore_lock);
prefixed_lint!(clippy::let_underscore_must_use);
prefixed_lint_versioned!(1.69, clippy::let_underscore_untyped);
prefixed_lint!(clippy::let_unit_value);
prefixed_lint_versioned!(1.69, clippy::let_with_type_underscore);
prefixed_lint_versioned!(1.70, clippy::lines_filter_map_ok);
prefixed_lint!(clippy::linkedlist);
// clippy::logic_bug has been renamed to `clippy::overly_complex_bool_expr`
prefixed_lint!(clippy::lossy_float_literal);
prefixed_lint!(clippy::macro_use_imports);
prefixed_lint!(clippy::main_recursion);
prefixed_lint_versioned!(1.57, clippy::manual_assert);
prefixed_lint!(clippy::manual_async_fn);
prefixed_lint_versioned!(1.60, clippy::manual_bits);
prefixed_lint_versioned!(1.66, clippy::manual_clamp);
prefixed_lint_versioned!(1.66, clippy::manual_filter);
prefixed_lint_versioned!(1.51, clippy::manual_filter_map);
prefixed_lint_versioned!(1.64, clippy::manual_find);
prefixed_lint_versioned!(1.51, clippy::manual_find_map);
prefixed_lint_versioned!(1.52, clippy::manual_flatten);
prefixed_lint_versioned!(1.65, clippy::manual_instant_elapsed);
prefixed_lint_versioned!(1.67, clippy::manual_is_ascii_check);
prefixed_lint_versioned!(1.67, clippy::manual_let_else);
prefixed_lint_versioned!(1.70, clippy::manual_main_separator_str);
prefixed_lint_versioned!(1.52, clippy::manual_map);
prefixed_lint!(clippy::manual_memcpy);
prefixed_lint!(clippy::manual_non_exhaustive);
prefixed_lint_versioned!(1.49, clippy::manual_ok_or);
prefixed_lint_versioned!(1.49, clippy::manual_range_contains);
prefixed_lint_versioned!(1.64, clippy::manual_rem_euclid);
prefixed_lint_versioned!(1.64, clippy::manual_retain);
prefixed_lint!(clippy::manual_saturating_arithmetic);
prefixed_lint_versioned!(1.70, clippy::manual_slice_size_calculation);
prefixed_lint_versioned!(1.57, clippy::manual_split_once);
prefixed_lint_versioned!(1.54, clippy::manual_str_repeat);
prefixed_lint_versioned!(1.65, clippy::manual_string_new);
prefixed_lint_versioned!(1.48, clippy::manual_strip);
prefixed_lint!(clippy::manual_swap);
prefixed_lint_versioned!(1.49, clippy::manual_unwrap_or);
prefixed_lint_versioned!(1.70, clippy::manual_while_let_some);
prefixed_lint!(clippy::many_single_char_names);
prefixed_lint!(clippy::map_clone);
prefixed_lint_versioned!(1.49, clippy::map_collect_result_unit);
prefixed_lint!(clippy::map_entry);
prefixed_lint_versioned!(1.48, clippy::map_err_ignore);
prefixed_lint!(clippy::map_flatten);
prefixed_lint_versioned!(1.47, clippy::map_identity);
prefixed_lint!(clippy::map_unwrap_or);
prefixed_lint!(clippy::match_as_ref);
prefixed_lint!(clippy::match_bool);
prefixed_lint_versioned!(1.47, clippy::match_like_matches_macro);
prefixed_lint!(clippy::match_on_vec_items);
prefixed_lint!(clippy::match_overlapping_arm);
prefixed_lint!(clippy::match_ref_pats);
prefixed_lint_versioned!(1.57, clippy::match_result_ok);
prefixed_lint!(clippy::match_same_arms);
prefixed_lint!(clippy::match_single_binding);
prefixed_lint_versioned!(1.58, clippy::match_str_case_mismatch);
prefixed_lint!(clippy::match_wild_err_arm);
prefixed_lint!(clippy::match_wildcard_for_single_variants);
prefixed_lint!(clippy::maybe_infinite_iter);
// clippy::mem_discriminant_non_enum has been renamed to (prefixless) `enum_intrinsics_non_enums`
prefixed_lint!(clippy::mem_forget);
prefixed_lint!(clippy::mem_replace_option_with_none);
prefixed_lint!(clippy::mem_replace_with_default);
prefixed_lint!(clippy::mem_replace_with_uninit);
prefixed_lint!(clippy::min_max);
// clippy::misaligned_transmute is deprecated since at least 1.44.1
prefixed_lint!(clippy::mismatched_target_os);
prefixed_lint_versioned!(1.63, clippy::mismatching_type_param_order);
prefixed_lint_versioned!(1.67, clippy::misnamed_getters);
prefixed_lint!(clippy::misrefactored_assign_op);
prefixed_lint_versioned!(1.69, clippy::missing_assert_message);
prefixed_lint!(clippy::missing_const_for_fn);
prefixed_lint!(clippy::missing_docs_in_private_items);
prefixed_lint_versioned!(1.55, clippy::missing_enforced_import_renames);
prefixed_lint!(clippy::missing_errors_doc);
prefixed_lint!(clippy::missing_inline_in_public_items);
prefixed_lint_versioned!(1.51, clippy::missing_panics_doc);
prefixed_lint!(clippy::missing_safety_doc);
prefixed_lint_versioned!(1.61, clippy::missing_spin_loop);
prefixed_lint_versioned!(1.66, clippy::missing_trait_methods);
prefixed_lint!(clippy::mistyped_literal_suffixes);
prefixed_lint!(clippy::mixed_case_hex_literals);
prefixed_lint!(clippy::mixed_read_write_in_expression);
prefixed_lint_versioned!(1.57, clippy::mod_module_files);
prefixed_lint!(clippy::module_inception);
prefixed_lint!(clippy::module_name_repetitions);
prefixed_lint!(clippy::modulo_arithmetic);
prefixed_lint!(clippy::modulo_one);
prefixed_lint_versioned!(1.65, clippy::multi_assignments);
prefixed_lint!(clippy::multiple_crate_versions);
prefixed_lint!(clippy::multiple_inherent_impl);
prefixed_lint_versioned!(1.69, clippy::multiple_unsafe_ops_per_block);
prefixed_lint!(clippy::must_use_candidate);
prefixed_lint!(clippy::must_use_unit);
prefixed_lint!(clippy::mut_from_ref);
prefixed_lint!(clippy::mut_mut);
prefixed_lint_versioned!(1.49, clippy::mut_mutex_lock);
prefixed_lint!(clippy::mut_range_bound);
prefixed_lint!(clippy::mutable_key_type);
prefixed_lint!(clippy::mutex_atomic);
prefixed_lint!(clippy::mutex_integer);
prefixed_lint!(clippy::naive_bytecount);
prefixed_lint_versioned!(1.47, clippy::needless_arbitrary_self_type);
prefixed_lint_versioned!(1.54, clippy::needless_bitwise_bool);
prefixed_lint!(clippy::needless_bool);
prefixed_lint_versioned!(1.69, clippy::needless__bool_assign);
prefixed_lint!(clippy::needless_borrow);
prefixed_lint!(clippy::needless_borrowed_reference);
prefixed_lint!(clippy::needless_collect);
prefixed_lint!(clippy::needless_continue);
prefixed_lint!(clippy::needless_doctest_main);
prefixed_lint_versioned!(1.53, clippy::needless_for_each);
prefixed_lint_versioned!(1.59, clippy::needless_late_init);
prefixed_lint!(clippy::needless_lifetimes);
prefixed_lint_versioned!(1.61, clippy::needless_match);
prefixed_lint_versioned!(1.57, clippy::needless_option_as_deref);
prefixed_lint_versioned!(1.62, clippy::needless_option_take);
prefixed_lint_versioned!(1.63, clippy::needless_parens_on_range_literals);
prefixed_lint!(clippy::needless_pass_by_value);
prefixed_lint_versioned!(1.51, clippy::needless_question_mark);
prefixed_lint!(clippy::needless_range_loop);
prefixed_lint!(clippy::needless_return);
prefixed_lint_versioned!(1.59, clippy::needless_splitn);
prefixed_lint!(clippy::needless_update);
prefixed_lint!(clippy::neg_cmp_op_on_partial_ord);
prefixed_lint!(clippy::neg_multiply);
prefixed_lint_versioned!(1.57, clippy::negative_feature_names);
prefixed_lint!(clippy::never_loop);
prefixed_lint!(clippy::new_ret_no_self);
prefixed_lint!(clippy::new_without_default);
prefixed_lint!(clippy::no_effect);
prefixed_lint_versioned!(1.63, clippy::no_effect_replace);
prefixed_lint_versioned!(1.58, clippy::no_effect_underscore_binding);
prefixed_lint_versioned!(1.69, clippy::no_mangle_with_rust_abi);
prefixed_lint!(clippy::non_ascii_literal);
prefixed_lint_versioned!(1.53, clippy::non_octal_unix_permissions);
prefixed_lint_versioned!(1.57, clippy::non_send_fields_in_send_ty);
prefixed_lint!(clippy::nonminimal_bool);
prefixed_lint!(clippy::nonsensical_open_options);
prefixed_lint_versioned!(1.55, clippy::nonstandard_macro_braces);
prefixed_lint!(clippy::not_unsafe_ptr_arg_deref);
prefixed_lint_versioned!(1.64, clippy::obfuscated_if_else);
prefixed_lint_versioned!(1.59, clippy::octal_escapes);
prefixed_lint!(clippy::ok_expect);
prefixed_lint_versioned!(1.61, clippy::only_used_in_recursion);
prefixed_lint!(clippy::op_ref);
// option_and_then_some is renamed to `bind_instead_of_map`
prefixed_lint!(clippy::option_as_ref_deref);
prefixed_lint!(clippy::option_env_unwrap);
// option_expect_used is removed (renamed to `expect_used`)
prefixed_lint_versioned!(1.53, clippy::option_filter_map);
prefixed_lint_versioned!(1.47, clippy::option_if_let_else);
prefixed_lint!(clippy::option_map_or_none);
prefixed_lint!(clippy::option_map_unit_fn);
// option_map_unwrap_or is renamed to `map_unwrap_or`
//
// option_mapw_unwrap_or_else is removed (since 1.45.0?)
prefixed_lint!(clippy::option_option);
// option_unwrap_used is renamed to `unwrap_used`
prefixed_lint!(clippy::or_fun_call);
prefixed_lint_versioned!(1.61, clippy::or_then_unwrap);
prefixed_lint!(clippy::out_of_bounds_indexing);
prefixed_lint!(clippy::overflow_check_conditional);
prefixed_lint!(clippy::overly_complex_bool_expr);
prefixed_lint!(clippy::panic);
prefixed_lint_versioned!(1.48, clippy::panic_in_result_fn);
// clippy::panic_params has been renamed to (prefixless)
prefixed_lint!(clippy::panicking_unwrap);
prefixed_lint_versioned!(1.66, clippy::partial_pub_fields);
prefixed_lint!(clippy::partialeq_ne_impl);
prefixed_lint_versioned!(1.65, clippy::partialeq_to_none);
prefixed_lint!(clippy::path_buf_push_overwrite);

prefixed_lint_versioned!(1.47, clippy::pattern_type_mismatch);
prefixed_lint_versioned!(1.68, clippy::permissions_set_readonly_false);
prefixed_lint!(clippy::possible_missing_comma);
prefixed_lint!(clippy::precedence);
prefixed_lint_versioned!(1.61, clippy::print_in_format_impl);
prefixed_lint!(clippy::print_literal);
prefixed_lint_versioned!(1.50, clippy::print_stderr);
prefixed_lint!(clippy::print_stdout);
prefixed_lint!(clippy::print_with_newline);
prefixed_lint!(clippy::println_empty_string);
prefixed_lint!(clippy::ptr_arg);
prefixed_lint_versioned!(1.51, clippy::ptr_as_ptr);
prefixed_lint_versioned!(1.49, clippy::ptr_eq);
prefixed_lint!(clippy::ptr_offset_with_cast);
// clippy::pub_enum_variant_names has been removed. set the `avoid-breaking-exported-api` config
// option to `false` to enable the `enum_variant_names` lint for public items. (Probably a
// prefixless lint; `allow_prefixed` and `allow` crates don't support attribute parameters.)
prefixed_lint_versioned!(1.62, clippy::pub_use);
prefixed_lint!(clippy::question_mark);
prefixed_lint_versioned!(1.69, clippy::question_mark_used);
prefixed_lint!(clippy::range_minus_one);
prefixed_lint!(clippy::range_plus_one);
// clippy::range_step_by_zero is deprecated since at least 1.44.1
prefixed_lint!(clippy::range_zip_with_len);
prefixed_lint_versioned!(1.48, clippy::rc_buffer);
prefixed_lint_versioned!(1.63, clippy::rc_clone_in_vec_init);
prefixed_lint_versioned!(1.55, clippy::rc_mutex);
prefixed_lint_versioned!(1.63, clippy::read_zero_byte_vec);
prefixed_lint_versioned!(1.48, clippy::recursive_format_impl);
prefixed_lint!(clippy::redundant_allocation);
prefixed_lint_versioned!(1.69, clippy::redundant_async_block);
prefixed_lint!(clippy::redundant_clone);
prefixed_lint!(clippy::redundant_closure);
prefixed_lint!(clippy::redundant_closure_call);
prefixed_lint!(clippy::redundant_closure_for_method_calls);
prefixed_lint_versioned!(1.50, clippy::redundant_else);
prefixed_lint_versioned!(1.57, clippy::redundant_feature_names);
prefixed_lint!(clippy::redundant_field_names);
prefixed_lint!(clippy::redundant_pattern);
prefixed_lint!(clippy::redundant_pattern_matching);
prefixed_lint!(clippy::redundant_pub_crate);
prefixed_lint_versioned!(1.51, clippy::redundant_slicing);
prefixed_lint!(clippy::redundant_static_lifetimes);
prefixed_lint_versioned!(1.54, clippy::ref_binding_to_reference);
// clippy::ref_in_deref has been renamed to clippy::needless_borrow
prefixed_lint_versioned!(1.49, clippy::ref_option_ref);
// clippy::regex_macro has been removed
prefixed_lint_versioned!(1.47, clippy::repeat_once);
// clippy::replace_consts is deprecated since 1.45
prefixed_lint!(clippy::rest_pat_in_fully_bound_structs);
// result_expect_used has been renamed to `expect_used`
prefixed_lint_versioned!(1.65, clippy::result_large_err);
prefixed_lint!(clippy::result_map_or_into_option);
prefixed_lint!(clippy::result_map_unit_fn);
// result_map_unwrap_or_else is renamed to `map_unwrap_or`
prefixed_lint_versioned!(1.49, clippy::result_unit_err);
// result_unwrap_used is renamed to `unwrap_used`
prefixed_lint_versioned!(1.59, clippy::return_self_not_must_use);
// reverse_range_loop is removed (since 1.45?)
prefixed_lint!(clippy::reversed_empty_ranges);
prefixed_lint!(clippy::same_functions_in_if_condition);
prefixed_lint_versioned!(1.47, clippy::same_item_push);
prefixed_lint_versioned!(1.57, clippy::same_name_method);
prefixed_lint!(clippy::search_is_some);
prefixed_lint_versioned!(1.67, clippy::seek_from_current);
prefixed_lint_versioned!(1.67, clippy::seek_to_start_instead_of_rewind);
prefixed_lint_versioned!(1.48, clippy::self_assignment);
prefixed_lint_versioned!(1.55, clippy::self_named_constructors);
prefixed_lint_versioned!(1.57, clippy::self_named_module_files);
prefixed_lint_versioned!(1.52, clippy::semicolon_if_nothing_returned);
prefixed_lint_versioned!(1.68, clippy::semicolon_inside_block);
prefixed_lint_versioned!(1.68, clippy::semicolon_outside_block);
prefixed_lint_versioned!(1.58, clippy::separated_literal_suffix);
prefixed_lint!(clippy::serde_api_misuse);
prefixed_lint!(clippy::shadow_reuse);
prefixed_lint!(clippy::shadow_same);
prefixed_lint!(clippy::shadow_unrelated);
prefixed_lint!(clippy::short_circuit_statement);
// clippy::should_assert_eq is deprecated since at least 1.44.1
prefixed_lint!(clippy::should_implement_trait);
prefixed_lint_versioned!(1.60, clippy::significant_drop_in_scrutinee);
prefixed_lint_versioned!(1.69, clippy::significant_drop_tightening);
prefixed_lint!(clippy::similar_names);
prefixed_lint_versioned!(1.49, clippy::single_char_add_str);
prefixed_lint_versioned!(1.60, clippy::single_char_lifetime_names);
prefixed_lint!(clippy::single_char_pattern);
prefixed_lint!(clippy::single_component_path_imports);
prefixed_lint_versioned!(1.49, clippy::single_element_loop);
prefixed_lint!(clippy::single_match);
prefixed_lint!(clippy::single_match_else);
prefixed_lint_versioned!(1.50, clippy::size_of_in_element_count);
prefixed_lint_versioned!(1.68, clippy::size_of_ref);
prefixed_lint!(clippy::skip_while_next);
prefixed_lint!(clippy::slow_vector_initialization);
prefixed_lint_versioned!(1.47, clippy::stable_sort_primitive);
prefixed_lint_versioned!(1.64, clippy::std_instead_of_alloc);
prefixed_lint_versioned!(1.64, clippy::std_instead_of_core);
prefixed_lint!(clippy::str_to_string);
prefixed_lint!(clippy::string_add);
prefixed_lint!(clippy::string_add_assign);
prefixed_lint!(clippy::string_extend_chars);
prefixed_lint_versioned!(1.50, clippy::string_from_utf8_as_bytes);
prefixed_lint!(clippy::string_lit_as_bytes);
prefixed_lint_versioned!(1.58, clippy::string_slice);
prefixed_lint!(clippy::string_to_string);
prefixed_lint_versioned!(1.55, clippy::strlen_on_c_strings);
prefixed_lint!(clippy::struct_excessive_bools);
prefixed_lint!(clippy::suboptimal_flops);
prefixed_lint!(clippy::suspicious_arithmetic_impl);
prefixed_lint!(clippy::suspicious_assignment_formatting);
prefixed_lint_versioned!(1.69, clippy::suspicious_command_arg_space);
prefixed_lint_versioned!(1.70, clippy::suspicious_doc_comments);
prefixed_lint!(clippy::suspicious_else_formatting);
prefixed_lint!(clippy::suspicious_map);
prefixed_lint!(clippy::suspicious_op_assign_impl);
prefixed_lint_versioned!(1.50, clippy::suspicious_operation_groupings);
prefixed_lint_versioned!(1.54, clippy::suspicious_splitn);
prefixed_lint_versioned!(1.65, clippy::suspicious_to_owned);
prefixed_lint!(clippy::suspicious_unary_op_formatting);
prefixed_lint_versioned!(1.67, clippy::suspicious_xor_used_as_pow);
prefixed_lint_versioned!(1.63, clippy::swap_ptr_to_ref);
prefixed_lint!(clippy::tabs_in_doc_comments);
prefixed_lint!(clippy::temporary_assignment);
// clippy::temporary_cstring_as_ptr is renamed to (prefixless) temporary_cstring_as_ptr
prefixed_lint_versioned!(1.70, clippy::tests_outside_test_module);
prefixed_lint!(clippy::to_digit_is_some);
prefixed_lint_versioned!(1.58, clippy::to_string_in_format_args);
prefixed_lint!(clippy::todo);
prefixed_lint!(clippy::too_many_arguments);
prefixed_lint!(clippy::too_many_lines);
prefixed_lint!(clippy::toplevel_ref_arg);
prefixed_lint_versioned!(1.58, clippy::trailing_empty_array);
prefixed_lint_versioned!(1.47, clippy::trait_duplication_in_bounds);
prefixed_lint!(clippy::transmute_bytes_to_str);
prefixed_lint!(clippy::transmute_float_to_int);
prefixed_lint!(clippy::transmute_int_to_bool);
prefixed_lint!(clippy::transmute_int_to_char);
prefixed_lint!(clippy::transmute_int_to_float);
prefixed_lint_versioned!(1.69, clippy::transmute_int_to_non_zero);
prefixed_lint_versioned!(1.68, clippy::transmute_null_to_fn);
prefixed_lint_versioned!(1.58, clippy::transmute_num_to_bytes);
prefixed_lint!(clippy::transmute_ptr_to_ptr);
prefixed_lint!(clippy::transmute_ptr_to_ref);
prefixed_lint_versioned!(1.60, clippy::transmute_undefined_repr);
prefixed_lint_versioned!(1.47, clippy::transmutes_expressible_as_ptr_casts);
prefixed_lint!(clippy::transmuting_null);
prefixed_lint_versioned!(1.62, clippy::trim_split_whitespace);
prefixed_lint!(clippy::trivial_regex);
prefixed_lint!(clippy::trivially_copy_pass_by_ref);
prefixed_lint!(clippy::try_err);
prefixed_lint!(clippy::type_complexity);
prefixed_lint!(clippy::type_repetition_in_bounds);
prefixed_lint_versioned!(1.67, clippy::unchecked_duration_subtraction);
prefixed_lint_versioned!(1.58, clippy::undocumented_unsafe_blocks);
prefixed_lint_versioned!(1.49, clippy::undropped_manually_drops);
prefixed_lint!(clippy::unicode_not_nfc);
prefixed_lint!(clippy::unimplemented);
prefixed_lint!(clippy::uninit_assumed_init);
prefixed_lint_versioned!(1.58, clippy::uninit_vec);
prefixed_lint_versioned!(1.66, clippy::uninlined_format_args);
prefixed_lint!(clippy::unit_arg);
prefixed_lint!(clippy::unit_cmp);
prefixed_lint_versioned!(1.58, clippy::unit_hash);
prefixed_lint_versioned!(1.47, clippy::unit_return_expecting_ord);
prefixed_lint_versioned!(1.70, clippy::unnecessary_box_returns);
// clippy::unknown_clippy_lints is renamed to (prefixless rustc lint) unknown_lints
prefixed_lint!(clippy::unnecessary_cast);
prefixed_lint!(clippy::unnecessary_filter_map);
prefixed_lint_versioned!(1.61, clippy::unnecessary_find_map);
prefixed_lint!(clippy::unnecessary_fold);
prefixed_lint_versioned!(1.61, clippy::unnecessary_join);
prefixed_lint_versioned!(1.48, clippy::unnecessary_lazy_evaluations);
prefixed_lint!(clippy::unnecessary_mut_passed);
prefixed_lint!(clippy::unnecessary_operation);
prefixed_lint_versioned!(1.62, clippy::unnecessary_owned_empty_stringss);
prefixed_lint_versioned!(1.67, clippy::unnecessary_safety_comment);
prefixed_lint_versioned!(1.67, clippy::unnecessary_safety_doc);
prefixed_lint_versioned!(1.53, clippy::unnecessary_self_imports);
prefixed_lint_versioned!(1.46, clippy::unnecessary_sort_by);
prefixed_lint_versioned!(1.70, clippy::unnecessary_struct_initialization);
prefixed_lint_versioned!(1.59, clippy::unnecessary_to_owned);
prefixed_lint!(clippy::unnecessary_unwrap);
prefixed_lint_versioned!(1.50, clippy::unnecessary_wraps);
prefixed_lint!(clippy::unneeded_field_pattern);
prefixed_lint!(clippy::unneeded_wildcard_pattern);
prefixed_lint_versioned!(1.46, clippy::unnested_or_patterns);
prefixed_lint!(clippy::unreachable);
prefixed_lint!(clippy::unreadable_literal);
prefixed_lint!(clippy::unsafe_derive_deserialize);
prefixed_lint!(clippy::unsafe_removed_from_name);
// clippy::unsafe_vector_initialization is deprecated since at least 1.44.1
prefixed_lint!(clippy::unseparated_literal_suffix);
prefixed_lint!(clippy::unsound_collection_transmute);
// clippy::unstable_as_mut_slice is deprecated since at least 1.44.1
//
// clippy::unstable_as_slice is deprecated since at least 1.44.1
prefixed_lint_versioned!(1.54, clippy::unused_async);
// clippy::unused_collect is deprecated since at least 1.44.1
prefixed_lint_versioned!(1.66, clippy::unused_format_specs);
prefixed_lint!(clippy::unused_io_amount);
// clippy::unused_label is deprecated since at least 1.44.1
prefixed_lint_versioned!(1.65, clippy::unused_peekable);
prefixed_lint_versioned!(1.63, clippy::unused_rounding);
prefixed_lint!(clippy::unused_self);
prefixed_lint!(clippy::unused_unit);
prefixed_lint_versioned!(1.49, clippy::unusual_byte_groupings);
prefixed_lint_versioned!(1.48, clippy::unwrap_in_result);
prefixed_lint_versioned!(1.56, clippy::unwrap_or_else_default);
prefixed_lint!(clippy::unwrap_used);
prefixed_lint_versioned!(1.51, clippy::upper_case_acronyms);
prefixed_lint!(clippy::use_debug);
prefixed_lint!(clippy::use_self);
prefixed_lint!(clippy::used_underscore_binding);
prefixed_lint!(clippy::useless_asref);
prefixed_lint!(clippy::useless_attribute);
prefixed_lint!(clippy::useless_conversion);
prefixed_lint!(clippy::useless_format);
prefixed_lint!(clippy::useless_let_if_seq);
prefixed_lint!(clippy::useless_transmute);
prefixed_lint!(clippy::useless_vec);
prefixed_lint!(clippy::vec_box);
prefixed_lint_versioned!(1.51, clippy::vec_init_then_push);
prefixed_lint_versioned!(1.46, clippy::vec_resize_to_zero);
prefixed_lint!(clippy::verbose_bit_mask);
prefixed_lint!(clippy::verbose_file_reads);
prefixed_lint!(clippy::vtable_address_comparisons);
prefixed_lint!(clippy::while_immutable_condition);
prefixed_lint!(clippy::while_let_loop);
prefixed_lint!(clippy::while_let_on_iterator);
prefixed_lint!(clippy::wildcard_dependencies);
prefixed_lint!(clippy::wildcard_enum_match_arm);
prefixed_lint!(clippy::wildcard_imports);
prefixed_lint!(clippy::wildcard_in_or_patterns);
prefixed_lint!(clippy::write_literal);
prefixed_lint!(clippy::write_with_newline);
prefixed_lint!(clippy::writeln_empty_string);
// clippy::wrong_pub_self_convention has been removed: set the `avoid-breaking-exported-api` config
// option to `false` to enable the `wrong_self_convention` lint for public items. (Probably a
// prefixless lint; `allow_prefixed` and `allow` crates don't support attribute parameters.)
prefixed_lint!(clippy::wrong_self_convention);
prefixed_lint!(clippy::wrong_transmute);
prefixed_lint!(clippy::zero_divided_by_zero);
prefixed_lint!(clippy::zero_prefixed_literal);
prefixed_lint!(clippy::zero_ptr);
prefixed_lint_versioned!(1.50, clippy::zero_sized_map_values);
// clippy::zero_width_space renamed in 1.49 to clippy::invisible_characters
prefixed_lint!(clippy::zst_offset);

// TODO compile test that the following fails - BUT ONLY with `cargo clippy`
// prefixed_lint!(clippy::WRONG_LINT);
