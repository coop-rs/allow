macro_rules! standard_lint {
    // Unlike the general `$lint_name`-based macro branch (the 2nd branch of this `macro_rules!`),
    // We can't generate `#![allow(unknown_lints)]` in the first branch, because we have
    // `#![forbid(unknown_lints)]` in our tests. Hence this special handling of `unknown_lints`:
    (unknown_lints) => {
        ::allow_internal::generate_allow_attribute_macro_definition_standard!(unknown_lints);
    };
    // the `const _` is to check that the lint name is valid. It gets checked with `cargo check`.
    ($lint_name:ident) => {
        #[allow($lint_name)]
        const _: () = ();
        ::allow_internal::generate_allow_attribute_macro_definition_standard!($lint_name);
    };
}
macro_rules! prefixed_lint {
    // the `const _` is to check that the lint name is valid. But, it does NOT get checked with
    // `cargo check`! Use `cargo clippy` instead.
    ($lint_path:path) => {
        #[allow($lint_path)]
        const _: () = ();
        ::allow_internal::generate_allow_attribute_macro_definition_prefixed!($lint_path);
    };
}
