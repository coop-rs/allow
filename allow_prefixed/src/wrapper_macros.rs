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

        //::allow_internal::generate_allow_attribute_macro_prefixed!($lint_prefix, $lint_name);
    };
}

/// The accepted token stream is the same as the `$properties` part in "ALL_PARAMS" branch of
/// [`any`]. So the expected input starts with NOT with the lint prefix, but with the lint name.
macro_rules! check_that_standard_lint_exists {
    ($lint_name:ident, $($_:tt)+) => {
        #[allow($lint_name)]
        const _: () = ();
    };
}

/// The accepted token stream is the same as the `$lint_name` + `$properties` part in "ALL_PARAMS"
/// branch of [`any`]. So the expected input starts with the lint prefix, then the lint name.
macro_rules! check_that_prefixed_lint_exists {
    ($lint_prefix:ident, $lint_name:ident, $($_:tt)+) => {
        ::allow_internal::check_that_prefixed_lint_exists!($lint_prefix, $lint_name);
    };
}

/// Input like of several other macros.
macro_rules! check_that_default_is_populated {
    ($_lint_name:ident, allowed, $($_:tt)+) => {};
    ($_lint_name:ident, warn, $($_:tt)+) => {};
    ($_lint_name:ident, deny, $($_:tt)+) => {};
}

/// Input like of several other macros.
macro_rules! check_that_default_is_blank {
    ($_lint_name:ident, "", $($_:tt)+) => {};
}

/// Dispatch to an appropriate `::allow_internal::doc_and_attrib_macro_***` proc macro to generate
/// documentation and the source of the desired proc macro that allows the given lint.
///
/// For supported "public" input see comments in source code of [`validate_any`].
macro_rules! any {
    // TODO the allow_internal:: proc macro will pass $not_anymore and $not_yet to allow_prefixed::
    // The following input variations are a "private" interface of this macro: Used from other match
    // branches of this macro only.
    (ALL_PARAMS, rustc, $($properties:tt)+) => {
        check_that_default_is_populated!($($properties)+);
        check_that_standard_lint_exists!($($properties)+);
        //@TODO
    };
    (ALL_PARAMS, rustdoc, $($properties:tt)+) => {
        check_that_default_is_blank!($($properties)+);
        check_that_prefixed_lint_exists!(rustdoc, $($properties)+);
        //@TODO
    };
    (ALL_PARAMS, clippy, $($properties:tt)+) => {
        check_that_default_is_blank!($($properties)+);
        check_that_prefixed_lint_exists!(clippy, $($properties)+);

        ::allow_internal::doc_and_attrib_macro_clippy!($($properties)+);
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $since_major_minor:tt, $until_major_minor:tt, $not_yet:literal, $not_anymore:literal) => {
        any!(ALL_PARAMS,
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor,
            $until_major_minor,
            $not_yet, $not_anymore);
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $since_major_minor:tt, "", $not_yet:literal) => {
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor,
            "",
            $not_yet,
            false // not deprecated/discontinued yet (but potentially not available yet, either)
        );
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $since_major_minor:tt, $until_major_minor:tt, $not_yet:literal) => {
        #[rustversion::not(since($until_major_minor.0))]
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor,
            $until_major_minor,
            $not_yet,
            false // not deprecated/discontinued yet (but potentially not available yet, either)
        );
        #[rustversion::since($until_major_minor.0)]
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor,
            $until_major_minor,
            $not_yet,
            true // not available anymore
        );
    };

    // The following input variations are "public" interface of this macro.
    //
    // The following variant requires $until_major_minor NOT to be an empty string. Otherwise use
    // the other (shortcut) variants.
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $since_major_minor:tt, $until_major_minor:tt) => {
        #[rustversion::not(since($since_major_minor.0))]
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor,
            $until_major_minor,
            true // not available yet
        );
        #[rustversion::since($since_major_minor.0)]
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor,
            $until_major_minor,
            false // already available (but potentially already deprecated, too)
        );
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, "nightly") => {
        any!($lint_prefix, $lint_name, $default, $deprecated, "nightly", "");
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $since_major_minor:tt) => {
        any!($lint_prefix, $lint_name, $default, $deprecated, $since_major_minor, "");
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt) => {
        any!($lint_prefix, $lint_name, $default, $deprecated, 1.45, "");
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt) => {
        any!($lint_prefix, $lint_name, $default, "", 1.45, "")
    };
    ($lint_prefix:tt, $lint_name:tt) => {
        any!($lint_prefix, $lint_name, "", "", 1.45, "")
    };
}

macro_rules! any_clippy {
    ($lint_name:tt) => {
        any!(clippy, $lint_name);
    };
    ($lint_name:tt, $($properties:tt)+) => {
        any!(clippy, $lint_name, "", "", $($properties:tt)+);
    }
}

/// Validate the input (as it's expected by TODO proc macro).
///
/// See comments in its source code.
macro_rules! validate_any {
    // $until_major_minor is EXCLUSIVE ("open range"), so only any version LOWER than
    // $until_major_minor is considered.
    //
    // Ensure that $since_major_minor is lower than $until_major_minor. This does NOT validate it.
    //
    //@TODO try (along with $deprecated): `:literal` wherever possible:
    //
    // ($lint_prefix:tt, $lint_name:tt, $since_major_minor:literal, $until_major_minor:literal)
    //
    // OR:
    //
    // ($lint_prefix:tt, $lint_name:tt, $since_major:literal.$since_minor:literal, $until_major:literal.$until_minor:literal)
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $since_major_minor:tt, $_until_major_minor:tt) => {
        validate_any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated,
            $since_major_minor
        );
    };
    //
    //@TODO try (along with $deprecated): `:literal` wherever possible:
    //
    //
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, "nightly") => {
        validate_any!($lint_prefix, $lint_name, $default, $deprecated);
    };
    //@TODO try (along with $deprecated):
    //
    // ($lint_prefix:tt, $lint_name:tt, $since_major_minor:literal)
    //
    // OR:
    //
    // ($lint_prefix:tt, $lint_name:tt, $since_major:literal.$since_minor:literal)
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt, $_since_major_minor:tt) => {
        validate_any!($lint_prefix, $lint_name, $default, $deprecated);
    };
    // `$deprecated` is an OPTIONAL message for deprecated lints. But: A lint is automatically
    // deprecated if Rust is past its `until_major_minor`, even if `$deprecated` is blank.
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated:tt) => {
        validate_any!($lint_prefix, $lint_name, $default);
    };

    (rustc, $_lint_name:ident, allowed) => {};
    (rustc, $_lint_name:ident, warn) => {};
    (rustc, $_lint_name:ident, deny) => {};
    (clippy, $_lint_name:ident, ) => {};
    (rustdoc, $_lint_name:ident, ) => {};

    // no `rustc` match with no $default!
    (clippy, $_lint_name:ident) => {};
    (rustdoc, $_lint_name:ident) => {};
}

macro_rules! rustdoc {
    ($lint_name:tt) => {
        //prefixed!("rustdoc", $lint_name);
    };
}
rustdoc!(bufo);

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
