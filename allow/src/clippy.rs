//! `clippy` lint macros, re-exported from [`allow_prefixed`](../allow_prefixed) crate.

// MAINTAINERS:
//
// 1. See rustc.rs
// 2. Do not keep/seek notes on lints renamed or removed before 1.45. For that see ../../allow_prefixed/src/lib.rs.

// Based on https://rust-lang.github.io/rust-clippy/index.html for 1.45 to master:
//
// Any  lint marked as `rustversion::since(1.44.1)` may have existed earlier, too.

use paste::paste;

macro_rules! export_since {
    ( $version:literal, $($unprefixed:ident),* ) => {
        paste! {
            #[rustversion::since($version)]
            pub use allow_prefixed::{
                $(
                    [<clippy_ $unprefixed>] as $unprefixed,
                )*
            };
        }
    };
}

macro_rules! export {
    ( $($unprefixed:ident),* ) => {
        export_since!( 1.45, $($unprefixed),* );
    }
}

export!(absurd_extreme_comparisons);
export_since!(1.64, alloc_instead_of_core);
export_since!(1.69, allow_attributes);
export_since!(1.61, allow_attributes_without_reason);
export_since!(1.68, almost_complete_range);
export!(almost_swapped, approx_constant);
export_since!(1.64, arithmetic_side_effects);
export!(as_conversions);
export_since!(1.66, as_ptr_cast_mut);
export_since!(1.63, as_underscore);
export!(assertions_on_constants);
export_since!(1.64, assertions_on_result_states);
export!(assign_op_pattern);
export_since!(1.48, async_yields_async);
export_since!(1.62, await_holding_invalid_type);
export!(await_holding_lock);
export_since!(1.49, await_holding_refcell_ref);
export!(bad_bit_mask, bind_instead_of_map);
export_since!(1.47, blanket_clippy_restriction_lints);
export!(blocks_in_if_conditions);
export_since!(1.53, bool_assert_comparison);
export!(bool_comparison);
export_since!(1.65, bool_to_int_with_if);
export_since!(1.60, borrow_as_ptr);
export_since!(1.63, borrow_deref_ref);
export!(borrow_interior_mutable_const, borrowed_box);
export_since!(1.57, box_collection);
export_since!(1.66, box_default);
export!(boxed_local);
export_since!(1.53, branches_sharing_code);
export!(builtin_type_shadow);
export_since!(1.62, bytes_count_to_len);
export_since!(1.52, bytes_nth);
export!(cargo_common_metadata);
export_since!(1.51, case_sensitive_file_extension_comparisons);
export_since!(1.62, cast_abs_to_unsigned);
export_since!(1.61, cast_enum_constructor, cast_enum_truncation);
export!(cast_lossless);
export_since!(1.66, cast_nan_to_int);
export!(
    cast_possible_truncation,
    cast_possible_wrap,
    cast_precision_loss,
    cast_ptr_alignment,
    cast_ref_to_mut,
    cast_sign_loss
);
export_since!(1.61, cast_slice_different_sizes);
export_since!(1.65, cast_slice_from_raw_parts);
export!(
    char_lit_as_u8,
    chars_last_cmp,
    chars_next_cmp,
    checked_conversions
);
export_since!(1.69, clear_with_drain);
export!(
    clone_double_ref,
    clone_on_copy,
    clone_on_ref_ptr,
    cloned_instead_of_copied
);
export!(cmp_nan);
export!(cmp_null, cmp_owned, cognitive_complexity);
export_since!(1.51, collapsible_else_if);
export!(collapsible_if);
export_since!(1.50, collapsible_match);
export_since!(1.65, collapsible_str_replace);
export_since!(1.69, collection_is_never_read);
export!(comparison_chain);
export_since!(1.49, comparison_to_empty);
export!(copy_iterator);
export_since!(1.62, crate_in_macro_def);
export_since!(1.48, create_dir);
export!(
    crosspointer_transmute,
    dbg_macro,
    debug_assert_with_mut_call,
    decimal_literal_representation,
    declare_interior_mutable_const
);
export_since!(1.64, default_instead_of_iter_empty);
export_since!(1.52, default_numeric_fallback);
export!(default_trait_access);
export_since!(1.60, default_union_representation);
export!(deprecated_cfg_attr, deprecated_semver, deref_addrof);
export_since!(1.61, deref_by_slicing);
export_since!(1.57, derivable_impls);
export_since!(1.47, derive_ord_xor_partial_ord);
export_since!(1.63, derive_partial_eq_without_eq);
export!(derived_hash_with_manual_eq);
export_since!(1.66, disallowed_macros);
export_since!(1.49, disallowed_methods);
export!(disallowed_names);
export_since!(1.55, disallowed_script_idents);
export_since!(1.55, disallowed_types);
export!(diverging_sub_expression);
export_since!(1.63, doc_link_with_quotes);
export!(
    doc_markdown,
    double_comparisons,
    double_must_use,
    double_neg,
    double_parens,
    drop_copy
);
export_since!(1.62, drop_non_drop);
export!(drop_ref);
export_since!(1.63, duplicate_mod);
export!(
    duplicate_underscore_argument,
    duration_subsec,
    else_if_without_else
);
export_since!(1.62, empty_drop);
export!(empty_enum, empty_line_after_outer_attr, empty_loop);
export_since!(1.62, empty_structs_with_brackets);
export!(
    enum_clike_unportable_variant,
    enum_glob_use,
    enum_variant_names,
    eq_op
);
export_since!(1.57, equatable_if_let);
export!(erasing_op);
export_since!(1.62, err_expect);
export!(excessive_precision);
export_since!(1.51, exhaustive_enums, exhaustive_structs);
export!(exit, expect_fun_call, expect_used, expl_impl_clone_on_copy);
export_since!(1.64, explicit_auto_deref);
export!(
    explicit_counter_loop,
    explicit_deref_methods,
    explicit_into_iter_loop,
    explicit_iter_loop,
    explicit_write
);
export_since!(1.55, extend_with_drain);
export!(extra_unused_lifetimes);
export_since!(1.69, extra_unused_type_parameters);
export!(fallible_impl_from);
export_since!(1.49, field_reassign_with_default);
export!(filetype_is_file);
export_since!(1.52, filter_map_identity);
export!(filter_map_next, filter_next, flat_map_identity);
export_since!(1.53, flat_map_option);
export!(float_arithmetic, float_cmp, float_cmp_const);
export_since!(1.48, float_equality_without_abs);
export!(fn_address_comparisons);
export_since!(1.68, fn_null_check);
export!(fn_params_excessive_bools, fn_to_numeric_cast);
export_since!(1.58, fn_to_numeric_cast_any);
export!(fn_to_numeric_cast_with_truncation, for_kv_map, forget_copy);
export_since!(1.62, forget_non_drop);
export!(forget_ref);
export_since!(1.58, format_in_format_args);
export_since!(1.62, format_push_string);
export_since!(1.49, from_iter_instead_of_collect);
export_since!(1.51, from_over_into);
export_since!(1.67, from_raw_with_void_ptr);
export_since!(1.52, from_str_radix_10);
export!(future_not_send);
export_since!(1.63, get_first);
export!(
    get_last_with_len,
    get_unwrap,
    identity_op,
    if_let_mutex,
    if_not_else,
    if_same_then_else
);
export_since!(1.53, if_then_some_else_none);
export!(ifs_same_cond);
export_since!(1.69, impl_trait_in_params);
export_since!(1.52, implicit_clone);
export!(implicit_hasher, implicit_return);
export_since!(1.66, implicit_saturating_add);
export!(
    implicit_saturating_sub,
    imprecise_flops,
    inconsistent_digit_grouping
);
export_since!(1.52, inconsistent_struct_constructor);
export_since!(1.59, index_refutable_slice);
export!(
    indexing_slicing,
    ineffective_bit_mask,
    inefficient_to_string,
    infallible_destructuring_match,
    infinite_iter,
    inherent_to_string,
    inherent_to_string_shadow_display
);
export_since!(1.59, init_numbered_fields);
export!(inline_always);
export_since!(1.49, inline_asm_x86_att_syntax, inline_asm_x86_intel_syntax);
export!(inline_fn_without_body);
export_since!(1.51, inspect_for_each);
export!(
    int_plus_one,
    integer_arithmetic,
    integer_division,
    into_iter_on_ref
);
export_since!(1.53, invalid_null_ptr_usage);
export!(invalid_regex, invalid_upcast_comparisons);
export_since!(1.64, invalid_utf8_in_unchecked);
export_since!(1.49, invisible_characters);
export_since!(1.62, is_digit_ascii_radix);
export!(items_after_statements);
export_since!(1.70, items_after_test_module);
export!(iter_cloned_collect);
export_since!(1.52, iter_count);
export_since!(1.66, iter_kv_map);
export!(iter_next_loop);
export_since!(1.46, iter_next_slice);
export_since!(1.57, iter_not_returning_iterator);
export!(iter_nth, iter_nth_zero);
export_since!(1.65, iter_on_empty_collections);
export_since!(1.65, iter_on_single_items);
export_since!(1.60, iter_overeager_cloned);
export!(iter_skip_next);
export_since!(1.61, iter_with_drain);
export!(
    iterator_step_by_zero,
    just_underscores_and_digits,
    large_const_arrays,
    large_digit_groups,
    large_enum_variant
);
export_since!(1.68, large_futures);
export_since!(1.62, large_include_file);
export!(large_stack_arrays);
export_since!(1.49, large_types_passed_by_value);
export!(len_without_is_empty, len_zero, let_and_return);
export_since!(1.67, let_underscore_future);
export!(let_underscore_lock, let_underscore_must_use);
export_since!(1.69, let_underscore_untyped);
export!(let_unit_value);
export_since!(1.69, let_with_type_underscore);
export_since!(1.70, lines_filter_map_ok);
export!(
    linkedlist,
    lossy_float_literal,
    macro_use_imports,
    main_recursion
);
export_since!(1.57, manual_assert);
export!(manual_async_fn);
export_since!(1.60, manual_bits);
export_since!(1.66, manual_clamp);
export_since!(1.66, manual_filter);
export_since!(1.51, manual_filter_map);
export_since!(1.64, manual_find);
export_since!(1.51, manual_find_map);
export_since!(1.52, manual_flatten);
export_since!(1.65, manual_instant_elapsed);
export_since!(1.67, manual_is_ascii_check, manual_let_else);
export_since!(1.70, manual_main_separator_str);
export_since!(1.52, manual_map);
export!(manual_memcpy, manual_non_exhaustive);
export_since!(1.49, manual_ok_or, manual_range_contains);
export_since!(1.64, manual_rem_euclid, manual_retain);
export!(manual_saturating_arithmetic);
export_since!(1.70, manual_slice_size_calculation);
export_since!(1.57, manual_split_once);
export_since!(1.54, manual_str_repeat);
export_since!(1.65, manual_string_new);
export_since!(1.48, manual_strip);
export!(manual_swap);
export_since!(1.49, manual_unwrap_or);
export_since!(1.70, manual_while_let_some);
export!(many_single_char_names, map_clone);
export_since!(1.49, map_collect_result_unit);
export!(map_entry);
export_since!(1.48, map_err_ignore);
export!(map_flatten);
export_since!(1.47, map_identity);
export!(map_unwrap_or, match_as_ref, match_bool);
export_since!(1.47, match_like_matches_macro);
export!(match_on_vec_items, match_overlapping_arm, match_ref_pats);
export_since!(1.57, match_result_ok);
export!(match_same_arms, match_single_binding);
export_since!(1.58, match_str_case_mismatch);
export!(
    match_wild_err_arm,
    match_wildcard_for_single_variants,
    maybe_infinite_iter,
    mem_forget,
    mem_replace_option_with_none,
    mem_replace_with_default,
    mem_replace_with_uninit,
    min_max,
    mismatched_target_os
);
export_since!(1.63, mismatching_type_param_order);
export_since!(1.67, misnamed_getters);
export!(misrefactored_assign_op);
export_since!(1.69, missing_assert_message);
export!(missing_const_for_fn, missing_docs_in_private_items);
export_since!(1.55, missing_enforced_import_renames);
export!(missing_errors_doc, missing_inline_in_public_items);
export_since!(1.51, missing_panics_doc);
export!(missing_safety_doc);
export_since!(1.61, missing_spin_loop);
export_since!(1.66, missing_trait_methods);
export!(
    mistyped_literal_suffixes,
    mixed_case_hex_literals,
    mixed_read_write_in_expression
);
export_since!(1.57, mod_module_files);
export!(
    module_inception,
    module_name_repetitions,
    modulo_arithmetic,
    modulo_one
);
export_since!(1.65, multi_assignments);
export!(multiple_crate_versions, multiple_inherent_impl);
export_since!(1.69, multiple_unsafe_ops_per_block);
export!(must_use_candidate, must_use_unit, mut_from_ref, mut_mut);
export_since!(1.49, mut_mutex_lock);
export!(
    mut_range_bound,
    mutable_key_type,
    mutex_atomic,
    mutex_integer,
    naive_bytecount
);
export_since!(1.47, needless_arbitrary_self_type);
export_since!(1.54, needless_bitwise_bool);
export!(needless_bool);
export_since!(1.69, needless__bool_assign);
export!(
    needless_borrow,
    needless_borrowed_reference,
    needless_collect,
    needless_continue,
    needless_doctest_main
);
export_since!(1.53, needless_for_each);
export_since!(1.59, needless_late_init);
export!(needless_lifetimes);
export_since!(1.61, needless_match);
export_since!(1.57, needless_option_as_deref);
export_since!(1.62, needless_option_take);
export_since!(1.63, needless_parens_on_range_literals);
export!(needless_pass_by_value);
export_since!(1.51, needless_question_mark);
export!(needless_range_loop, needless_return);
export_since!(1.59, needless_splitn);
export!(needless_update, neg_cmp_op_on_partial_ord, neg_multiply);
export_since!(1.57, negative_feature_names);
export!(never_loop, new_ret_no_self, new_without_default, no_effect);
export_since!(1.63, no_effect_replace);
export_since!(1.58, no_effect_underscore_binding);
export_since!(1.69, no_mangle_with_rust_abi);
export!(non_ascii_literal);
export_since!(1.53, non_octal_unix_permissions);
export_since!(1.57, non_send_fields_in_send_ty);
export!(nonminimal_bool, nonsensical_open_options);
export_since!(1.55, nonstandard_macro_braces);
export!(not_unsafe_ptr_arg_deref);
export_since!(1.64, obfuscated_if_else);
export_since!(1.59, octal_escapes);
export!(ok_expect);
export_since!(1.61, only_used_in_recursion);
export!(op_ref, option_as_ref_deref, option_env_unwrap);
export_since!(1.53, option_filter_map);
export_since!(1.47, option_if_let_else);
export!(
    option_map_or_none,
    option_map_unit_fn,
    option_option,
    or_fun_call
);
export_since!(1.61, or_then_unwrap);
export!(
    out_of_bounds_indexing,
    overflow_check_conditional,
    overly_complex_bool_expr,
    panic
);
export_since!(1.48, panic_in_result_fn);
export!(panicking_unwrap);
export_since!(1.66, partial_pub_fields);
export!(partialeq_ne_impl);
export_since!(1.65, partialeq_to_none);
export!(path_buf_push_overwrite);
export_since!(1.47, pattern_type_mismatch);
export_since!(1.68, permissions_set_readonly_false);
export!(possible_missing_comma, precedence);
export_since!(1.61, print_in_format_impl);
export!(print_literal);
export_since!(1.50, print_stderr);
export!(print_stdout, print_with_newline, println_empty_string);
export!(ptr_arg);
export_since!(1.51, ptr_as_ptr);
export_since!(1.49, ptr_eq);
export!(ptr_offset_with_cast);
export_since!(1.62, pub_use);
export!(question_mark);
export_since!(1.69, question_mark_used);
export!(range_minus_one, range_plus_one, range_zip_with_len);
export_since!(1.48, rc_buffer);
export_since!(1.63, rc_clone_in_vec_init);
export_since!(1.55, rc_mutex);
export_since!(1.63, read_zero_byte_vec);
export_since!(1.48, recursive_format_impl);
export!(redundant_allocation);
export_since!(1.69, redundant_async_block);
export!(
    redundant_clone,
    redundant_closure,
    redundant_closure_call,
    redundant_closure_for_method_calls
);
export_since!(1.50, redundant_else);
export_since!(1.57, redundant_feature_names);
export!(
    redundant_field_names,
    redundant_pattern,
    redundant_pattern_matching,
    redundant_pub_crate
);
export_since!(1.51, redundant_slicing);
export!(redundant_static_lifetimes);
export_since!(1.54, ref_binding_to_reference);
export_since!(1.49, ref_option_ref);
export_since!(1.47, repeat_once);
export!(rest_pat_in_fully_bound_structs);
export_since!(1.65, result_large_err);
export!(result_map_or_into_option, result_map_unit_fn);
export_since!(1.49, result_unit_err);
export_since!(1.59, return_self_not_must_use);
export!(reversed_empty_ranges, same_functions_in_if_condition);
export_since!(1.47, same_item_push);
export_since!(1.57, same_name_method);
export!(search_is_some);
export_since!(1.67, seek_from_current, seek_to_start_instead_of_rewind);
export_since!(1.48, self_assignment);
export_since!(1.55, self_named_constructors);
export_since!(1.57, self_named_module_files);
export_since!(1.52, semicolon_if_nothing_returned);
export_since!(1.68, semicolon_inside_block, semicolon_outside_block);
export_since!(1.58, separated_literal_suffix);
export!(
    serde_api_misuse,
    shadow_reuse,
    shadow_same,
    shadow_unrelated
);
export!(short_circuit_statement, should_implement_trait);
export_since!(1.60, significant_drop_in_scrutinee);
export_since!(1.69, significant_drop_tightening);
export!(similar_names);
export_since!(1.49, single_char_add_str);
export_since!(1.60, single_char_lifetime_names);
export!(single_char_pattern, single_component_path_imports);
export_since!(1.49, single_element_loop);
export!(single_match, single_match_else);
export_since!(1.50, size_of_in_element_count);
export_since!(1.68, size_of_ref);
export!(skip_while_next, slow_vector_initialization);
export_since!(1.47, stable_sort_primitive);
export_since!(1.64, std_instead_of_alloc, std_instead_of_core);
export!(
    str_to_string,
    string_add,
    string_add_assign,
    string_extend_chars
);
export_since!(1.50, string_from_utf8_as_bytes);
export!(string_lit_as_bytes);
export_since!(1.58, string_slice);
export!(string_to_string);
export_since!(1.55, strlen_on_c_strings);
export!(
    struct_excessive_bools,
    suboptimal_flops,
    suspicious_arithmetic_impl,
    suspicious_assignment_formatting
);
export_since!(1.69, suspicious_command_arg_space);
export_since!(1.70, suspicious_doc_comments);
export!(
    suspicious_else_formatting,
    suspicious_map,
    suspicious_op_assign_impl
);
export_since!(1.50, suspicious_operation_groupings);
export_since!(1.54, suspicious_splitn);
export_since!(1.65, suspicious_to_owned);
export!(suspicious_unary_op_formatting);
export_since!(1.67, suspicious_xor_used_as_pow);
export_since!(1.63, swap_ptr_to_ref);
export!(tabs_in_doc_comments, temporary_assignment);
export_since!(1.70, tests_outside_test_module);
export!(to_digit_is_some);
export_since!(1.58, to_string_in_format_args);
export!(todo, too_many_arguments, too_many_lines, toplevel_ref_arg);
export_since!(1.58, trailing_empty_array);
export_since!(1.47, trait_duplication_in_bounds);
export!(
    transmute_bytes_to_str,
    transmute_float_to_int,
    transmute_int_to_bool,
    transmute_int_to_char,
    transmute_int_to_float
);
export_since!(1.69, transmute_int_to_non_zero);
export_since!(1.68, transmute_null_to_fn);
export_since!(1.58, transmute_num_to_bytes);
export!(transmute_ptr_to_ptr, transmute_ptr_to_ref);
export_since!(1.60, transmute_undefined_repr);
export_since!(1.47, transmutes_expressible_as_ptr_casts);
export!(transmuting_null);
export_since!(1.62, trim_split_whitespace);
export!(
    trivial_regex,
    trivially_copy_pass_by_ref,
    try_err,
    type_complexity,
    type_repetition_in_bounds
);
export_since!(1.67, unchecked_duration_subtraction);
export_since!(1.58, undocumented_unsafe_blocks);
export_since!(1.49, undropped_manually_drops);
export!(unicode_not_nfc, unimplemented, uninit_assumed_init);
export_since!(1.58, uninit_vec);
export_since!(1.66, uninlined_format_args);
export!(unit_arg, unit_cmp);
export_since!(1.58, unit_hash);
export_since!(1.47, unit_return_expecting_ord);
export_since!(1.70, unnecessary_box_returns);
export!(unnecessary_cast, unnecessary_filter_map);
export_since!(1.61, unnecessary_find_map);
export!(unnecessary_fold);
export_since!(1.61, unnecessary_join);
export_since!(1.48, unnecessary_lazy_evaluations);
export!(unnecessary_mut_passed, unnecessary_operation);
export_since!(1.62, unnecessary_owned_empty_stringss);
export_since!(1.67, unnecessary_safety_comment, unnecessary_safety_doc);
export_since!(1.53, unnecessary_self_imports);
export_since!(1.46, unnecessary_sort_by);
export_since!(1.70, unnecessary_struct_initialization);
export_since!(1.59, unnecessary_to_owned);
export!(unnecessary_unwrap);
export_since!(1.50, unnecessary_wraps);
export!(unneeded_field_pattern, unneeded_wildcard_pattern);
export_since!(1.46, unnested_or_patterns);
export!(
    unreachable,
    unreadable_literal,
    unsafe_derive_deserialize,
    unsafe_removed_from_name,
    unseparated_literal_suffix,
    unsound_collection_transmute
);
export_since!(1.54, unused_async);
export_since!(1.66, unused_format_specs);
export!(unused_io_amount);
export_since!(1.65, unused_peekable);
export_since!(1.63, unused_rounding);
export!(unused_self, unused_unit);
export_since!(1.49, unusual_byte_groupings);
export_since!(1.48, unwrap_in_result);
export_since!(1.56, unwrap_or_else_default);
export!(unwrap_used);
export_since!(1.51, upper_case_acronyms);
export!(
    use_debug,
    use_self,
    used_underscore_binding,
    useless_asref,
    useless_attribute,
    useless_conversion,
    useless_format,
    useless_let_if_seq,
    useless_transmute,
    useless_vec,
    vec_box
);
export_since!(1.51, vec_init_then_push);
export_since!(1.46, vec_resize_to_zero);
export!(
    verbose_bit_mask,
    verbose_file_reads,
    vtable_address_comparisons,
    while_immutable_condition,
    while_let_loop,
    while_let_on_iterator,
    wildcard_dependencies,
    wildcard_enum_match_arm,
    wildcard_imports,
    wildcard_in_or_patterns,
    write_literal,
    write_with_newline,
    writeln_empty_string,
    wrong_self_convention,
    wrong_transmute,
    zero_divided_by_zero,
    zero_prefixed_literal,
    zero_ptr
);
export_since!(1.50, zero_sized_map_values);
// zero_width_space renamed in 1.49 to invisible_characters
export!(zst_offset);
