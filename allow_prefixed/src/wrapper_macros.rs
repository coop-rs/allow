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
macro_rules! check_that_default_is_underscore {
    ($_lint_name:ident, _, $($_:tt)+) => {};
}

/// Internal transformation for (after/from within) [`any`].
///
/// Unlike [`any`], all input patterns here treat `$nightly` as a bool literal.
///
/// Suggest you look at the source code of [`any`] first. Then read the source code of
/// `any_with_bools`, BUT from the bottom up (from the last input pattern to the first).
macro_rules! any_with_bools {
    // TODO the allow_internal:: proc macro will pass $not_anymore and $not_yet to allow_prefixed::
    // The following input variations are a "private" interface of this macro: Used from other match
    // branches of this macro only.
    (ALL_PARAMS, rustc, $($properties:tt)+) => {
        check_that_default_is_populated!($($properties)+);
        check_that_standard_lint_exists!($($properties)+);

        ::allow_internal::doc_and_attrib_macro_rustc!($($properties)+);
    };
    (ALL_PARAMS, rustdoc, $($properties:tt)+) => {
        check_that_default_is_underscore!($($properties)+);
        check_that_prefixed_lint_exists!(rustdoc, $($properties)+);
        //@TODO
    };
    (ALL_PARAMS, clippy, $($properties:tt)+) => {
        check_that_default_is_underscore!($($properties)+);
        check_that_prefixed_lint_exists!(clippy, $($properties)+);

        ::allow_internal::doc_and_attrib_macro_clippy!($($properties)+);
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt, $nightly:literal, $until_major_minor:tt, $not_yet:literal, $not_anymore:literal) => {
        any_with_bools!(ALL_PARAMS,
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly,
            $until_major_minor,
            $not_yet,
            $not_anymore);
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt, $nightly:literal, _, $not_yet:literal) => {
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly,
            "", // This is NOT an underscore, but an empty string, so that the proc macro can expect a literal.
            $not_yet,
            false // not deprecated/discontinued yet (but potentially not available yet, either)
        );
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt, $nightly:literal, $until_major_minor:tt, $not_yet:literal) => {
        #[rustversion::not(since($until_major_minor))]
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly,
            $until_major_minor,
            $not_yet,
            false // not deprecated/discontinued yet (but potentially not available yet, either)
        );
        #[rustversion::since($until_major_minor)]
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly,
            $until_major_minor,
            $not_yet,
            true // not available anymore
        );
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, _, $since_major_minor:tt, $nightly:literal, $until_major_minor:tt) => {
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            "", // This is NOT an underscore, but an empty string, so that the proc macro can expect a literal.
            $since_major_minor,
            $nightly,
            $until_major_minor
        );
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt, $nightly:literal, $until_major_minor:tt) => {
        #[rustversion::not(since($since_major_minor))]
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly,
            $until_major_minor,
            true // not available yet
        );
        #[rustversion::since($since_major_minor)]
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly,
            $until_major_minor,
            false // already available (but potentially already deprecated, too)
        );
    };
}

/// Dispatch to an appropriate `::allow_internal::doc_and_attrib_macro_***` proc macro to generate
/// documentation and the source of the desired proc macro that allows the given lint.
///
/// TODO CHECK: For supported "public" input see comments in source code of [`validate_any`].
///
/// Unlike [`any_with_bools`], here `$nightly` is NOT as a bool literal, but either `nightly`
/// (without quotes, like a language keyword), or empty (with a trailing comma left in).
///
/// Read the source code from the BOTTOM UP (from the last input pattern to the first). Then you may
/// want to look at [`any_with_bools`].
///
/// $until_major_minor is EXCLUSIVE ("open range"), so only any version LOWER than
/// $until_major_minor is considered.
///
/// Unfortunately, `rustversion` crate doesn't support version notations like `1.71.0-nightly`.
/// Hence we have a separate `nightly` flag. That also helps when troubleshooting, since `nightly`
/// version depends on when it was updated the last time...
macro_rules! any {
    // TODO CHECK: The following variant requires $until_major_minor NOT to be an underscore _.
    // Otherwise add two new variants, or use the other (shortcut) variants.
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt,
    _, $until_major_minor:tt) => {
        validate_any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            false,
            $until_major_minor
        );
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            false,
            $until_major_minor
        );
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt,
    nightly, $until_major_minor:tt) => {
        validate_any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            true,
            $until_major_minor
        );
        any_with_bools!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            true,
            $until_major_minor
        );
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt, $nightly_or_underscore:tt) => {
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            $nightly_or_underscore,
            _
        );
    };

    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt, $since_major_minor:tt) => {
        any!(
            $lint_prefix,
            $lint_name,
            $default,
            $deprecated_msg,
            $since_major_minor,
            _
        );
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt, $deprecated_msg:tt) => {
        any!($lint_prefix, $lint_name, $default, $deprecated_msg, 1.45, _);
    };
    ($lint_prefix:tt, $lint_name:tt, $default:tt) => {
        any!($lint_prefix, $lint_name, $default, _, 1.45,);
    };
    ($lint_prefix:tt, $lint_name:tt) => {
        any!($lint_prefix, $lint_name, _, _, 1.45,);
    };
}

macro_rules! any_clippy {
    ($lint_name:tt) => {
        any!(clippy, $lint_name);
    };
    ($lint_name:tt, $($properties:tt)+) => {
        any!(clippy, $lint_name, _, _, $($properties:tt)+);
    }
}

macro_rules! validate_major_minor {
    ($major_minor:literal) => {
        const _: f32 = $major_minor;
    };
}

/// Validate the input (as it's expected by [`any`]. It excludes validation that is done by [`any`]
/// itself.
macro_rules! validate_any {
    ($_lint_prefix:tt, $_lint_name:tt, $_default:tt, $_deprecated_msg:tt, $since_major_minor:tt, $_nightly_or_underscore:tt, _) => {
        validate_major_minor!($since_major_minor);
    };
    ($_lint_prefix:tt, $_lint_name:tt, $_default:tt, $_deprecated_msg:tt, $since_major_minor:tt, $_nightly_or_underscore:tt, $until_major_minor:tt) => {
        validate_major_minor!($since_major_minor);
        validate_major_minor!($until_major_minor);
        const _: () = {
            assert!($since_major_minor <= $until_major_minor);
        };
    };
}

macro_rules! rustdoc {
    ($lint_name:tt) => {
        //prefixed!("rustdoc", $lint_name);
    };
}
rustdoc!(bufo);

macro_rules! standard_lint {
    ($_:tt) => {};
}
macro_rules! standard_lint_allowed {
    // The `const _` is to check that the lint name is valid (thanks to `#![deny(unknown_lints)]` in
    // `lib.rs`). It gets checked with `cargo check`.
    ($lint_name:tt) => {
        #[allow($lint_name)]
        const _: () = ();
        any!(rustc, $lint_name, allowed, _, 1.45);
    };
}

macro_rules! standard_lint_allowed_from {
    ($lint_name:tt, $since_major_minor:tt) => {
        #[allow($lint_name)]
        const _: () = ();
        any!(rustc, $lint_name, allowed, _, $since_major_minor);
    };
}

macro_rules! standard_lint_allowed_from_to {
    ($lint_name:tt, $since_major_minor:tt, $until_major_minor:tt) => {
        #[allow($lint_name)]
        const _: () = ();
        any!(
            rustc,
            $lint_name,
            allowed,
            _,
            $since_major_minor,
            _,
            $until_major_minor
        );
    };
}

macro_rules! standard_lint_allowed_nightly {
    ($lint_name:tt) => {
        #[allow($lint_name)]
        const _: () = ();
        any!(rustc, $lint_name, allowed, _, 1.45, nightly);
    };
}

macro_rules! standard_lint_versioned {
    // We can't match major.minor.patch in macro_rules. So far all lints started at patch version
    // being 0, so we omit it as a parameter.
    ($major_minor:tt, $lint_name:tt) => {
        #[rustversion::since($major_minor)]
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
        #[rustversion::since($major_minor)]
        prefixed_lint!($lint_prefix, $lint_name);
    }; // @TODO initial version - deprecated (or removed?) version
}

macro_rules! prefixed_lint_nightly {
    ($lint_prefix:tt, $lint_name:tt) => {
        #[rustversion::nightly]
        prefixed_lint!($lint_prefix, $lint_name);
    }; // @TODO initial version - deprecated (or removed?) version
}
