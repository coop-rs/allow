macro_rules! standard_lint {
    // the `const _` is to check that the lint name is valid. It gets checked with `cargo check`.
    ($lint_name:ident) => {
        #[allow($lint_name)]
        const _: () = ();
        ::allow_internal::generate_allow_attribute_macro_definition_standard!($lint_name);
    };
}
macro_rules! prefixed_lint {
    // the `const _` is to check that the lint name is valid. But, it does NOT get checked with
    // `cargo check`! Use `cargo clippy` or `cargo rustdoc`, respectively, instead.
    ($lint_path:path) => {
        #[allow($lint_path)]
        const _: () = ();
        ::allow_internal::generate_allow_attribute_macro_definition_prefixed!($lint_path);
    };
}

macro_rules! standard_lint_versioned {
    // We can't match major.minor.patch in macro_rules. So far all lints started at patch version
    // being 0, so we omit it as a parameter.
    ($major_minor:tt, $lint_name:ident) => {
        #[rustversion::since($major_minor.0)]
        standard_lint!($lint_name);
    }; // @TODO initial version - deprecated (or removed?) version
}

macro_rules! standard_lint_nightly {
    ($lint_name:ident) => {
        #[rustversion::nightly]
        standard_lint!($lint_name);
    };
}

macro_rules! prefixed_lint_versioned {
    // Again,  omitting patch version it as a parameter.
    ($major_minor:tt, $lint_path:path) => {
        #[rustversion::since($major_minor.0)]
        prefixed_lint!($lint_path);
    }; // @TODO initial version - deprecated (or removed?) version
}

macro_rules! prefixed_lint_nightly {
    ($lint_path:path) => {
        #[rustversion::nightly]
        prefixed_lint!($lint_path);
    }; // @TODO initial version - deprecated (or removed?) version
}
