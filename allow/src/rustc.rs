//! Prefixless lint macros (for `rustc`/standard lints), re-exported from
//! [`allow_prefixed`](../allow_prefixed) crate.

// MAINTAINERS:
// - Based on <../allow_prefixed/src/lib.rs>
// - Three big parts, based on the default notification level: allowed, warning and denying.
// - Keep all parts alphabetically sorted.
// - One entry per line.

// 1. Lints with default level `allow`. See ../../allow_prefixed/src/lib.rs.

#[rustfmt::skip]
/// @TODO Does this rustdoc apply/show up anywhere?
pub use allow_prefixed::{
    // absolute_paths_not_starting_with_crate was in edition 2015 only (and we require 2018+).
    box_pointers,
    // elided_lifetimes_in_paths - at crate level only
    explicit_outlives_requirements,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    ffi_unwind_calls,
    fuzzy_provenance_casts,
};
#[rustfmt::skip]
pub use allow_prefixed::{
    keyword_idents,
    let_underscore_drop,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    lossy_provenance_casts,
};
#[rustfmt::skip]
pub use allow_prefixed::{
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    multiple_supertrait_upcastable,
    must_not_suspend,
    // non_ascii_idents - at crate level only
    non_exhaustive_omitted_patterns
};
#[rustfmt::skip]
pub use allow_prefixed::{
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    // rust_2021_prefixes_incompatible_syntax - at crate level only
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
};
#[rustversion::since(1.52)]
#[rustfmt::skip]
pub use allow_prefixed::{
    unsafe_op_in_unsafe_fn
};
#[rustfmt::skip]
pub use allow_prefixed::{
    // unstable_features - deprecated
    //
    // unused_crate_dependencies - at crate level only
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    unused_tuple_struct_fields,
    variant_size_differences,
};

// 2. Lints with default level `warn`. See ../../allow_prefixed/src/lib.rs.

#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    ambiguous_glob_reexports,
};
#[rustfmt::skip]
pub use allow_prefixed::{
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bare_trait_objects,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    clashing_extern_declarations,
    coherence_leak_check,
    // confusable_idents - at crate level only
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    for_loops_over_fallibles,
    forbidden_lint_groups,
    function_item_references,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_doc_attributes,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    invalid_macro_export_arguments, //@TODO not on 1.71.0-nightly?!
};
#[rustfmt::skip]
pub use allow_prefixed::{
    invalid_value,
    irrefutable_let_patterns,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    map_unit_fn,
    // mixed_script_confusables - at crate level only
    named_arguments_used_positionally,
    no_mangle_generic_items,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    opaque_hidden_inferred_bound,
    overlapping_range_endpoints,
    path_statements,
    private_in_public,
    redundant_semicolons,
    renamed_and_removed_lints,
    repr_transparent_external_private_fields,
    semicolon_in_expressions_from_macros,
    special_module_name,
    stable_features,
    suspicious_auto_trait_impls,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    suspicious_double_ref_op,
};
#[rustfmt::skip]
pub use allow_prefixed::{
    temporary_cstring_as_ptr,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    // uncommon_codepoints - at crate level only
    unconditional_recursion,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    undefined_naked_function_abi,
    unexpected_cfgs,
    unfulfilled_lint_expectations
};
#[rustfmt::skip]
pub use allow_prefixed::{
    ungated_async_fn_track_caller,
    uninhabited_static,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_labels,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_variables,
    // warnings is a group
    where_clauses_object_safety,
    while_true
};

// 3. Lints with default level `deny`. See ../../allow_prefixed/src/lib.rs.

#[rustfmt::skip]
pub use allow_prefixed::{
    ambiguous_associated_items,
    arithmetic_overflow,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    conflicting_repr_hints,
    deprecated_cfg_attr_crate_type_name,
    enum_intrinsics_non_enums,
    // ill_formed_attribute_input - at crate level only
    implied_bounds_entailment,
    incomplete_include,
    ineffective_unstable_trait_impl,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    invalid_alignment,//@TODO not on 1.71.0-nightly?!
};
#[rustfmt::skip]
pub use allow_prefixed::{
    invalid_atomic_ordering,
    invalid_type_param_default,
    let_underscore_lock,
    // macro_expanded_macro_exports_accessed_by_absolute_paths - at crate level only
    missing_fragment_specifier,
    mutable_transmutes,
    named_asm_labels,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    patterns_in_fns_without_body,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    soft_unstable,
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    test_unstable_lint
};
#[rustfmt::skip]
pub use allow_prefixed::{
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    unconditional_panic,
    // unknown_crate_types - at crate level only
    useless_deprecated,
};
