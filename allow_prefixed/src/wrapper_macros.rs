macro_rules! standard_lint {
    // The `const _` is to check that the lint name is valid (thanks to `#![deny(unknown_lints)]` in
    // `lib.rs`). It gets checked with `cargo check`.
    ($lint_name:tt) => {
        #[allow($lint_name)]
        const _: () = ();
        ::allow_internal::generate_allow_attribute_macro_standard!($lint_name);
    };
}
macro_rules! prefixed_lint {
    // The lint existence check below can't be generated from macro_rules!, because we need to
    // concatenate the lint prefix and the lint name. (We can't use `paste` crate, as that can
    // generate `ident` only, not `path`).
    //
    // We CAN'T use one parameter `$lint_path:path` instead of `$lint_prefix:tt, $lint_name:tt`. See
    // https://github.com/rust-lang/rust-analyzer/issues/14772.
    ($lint_prefix:tt, $lint_name:tt) => {
        ::allow_internal::check_that_prefixed_lint_exists!($lint_prefix, $lint_name);
        ::allow_internal::generate_allow_attribute_macro_prefixed!($lint_prefix, $lint_name);
    };
}

macro_rules! standard_lint_versioned {
    // We can't match major.minor.patch in macro_rules. So far all lints started at patch version
    // being 0, so we omit it as a parameter.
    ($major_minor:tt, $lint_name:tt) => {
        #[rustversion::since($major_minor.0)]
        standard_lint!($lint_name);
    }; // @TODO initial version - deprecated (or removed?) version
}

macro_rules! standard_lint_nightly {
    ($lint_name:tt) => {
        #[rustversion::nightly]
        standard_lint!($lint_name);
    };
}

macro_rules! prefixed_lint_versioned {
    // Again,  omitting patch version as a parameter.
    ($major_minor:tt, $lint_prefix:tt, $lint_name:tt) => {
        #[rustversion::since($major_minor.0)]
        prefixed_lint!($lint_prefix, $lint_name);
    }; // @TODO initial version - deprecated (or removed?) version
}

macro_rules! prefixed_lint_nightly {
    ($lint_prefix:tt, $lint_name:tt) => {
        #[rustversion::nightly]
        prefixed_lint!($lint_prefix, $lint_name);
    }; // @TODO initial version - deprecated (or removed?) version
}
