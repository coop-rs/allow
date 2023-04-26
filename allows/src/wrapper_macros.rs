macro_rules! standard_lint {
    // the `const _` is to check that the lint name is valid. It gets checked with `cargo check`.
    ($lint_name:ident) => {
        #[deny(unknown_lints)]
        #[allow($lint_name)]
        const _: () = ();
        ::allows_internals::generate_allows_attribute_macro_definition_standard!($lint_name);
    };
}
macro_rules! prefixed_lint {
    // the `const _` is to check that the lint name is valid. But, it does NOT get checked with
    // `cargo check`! Use `cargo clippy` instead.
    ($lint_path:path) => {
        #[deny(unknown_lints)]
        #[allow($lint_path)]
        const _: () = ();
        ::allows_internals::generate_allows_attribute_macro_definition_prefixed!($lint_path);
    };
}
