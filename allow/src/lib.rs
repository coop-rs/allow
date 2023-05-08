//! Alias lints (to allow them = suppress their notices), label your intentions.
//!
//! Re-exported from `allow_prefixed` crate: prefixless (rustc/standard) lints are at the top level
//! and also grouped (duplicated) under `rustc::` module; `clippy` and `rustdoc` lints are grouped
//! under clippy:: and rustdoc:: modules.

#![forbid(unknown_lints)]
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
pub mod clippy;
pub mod rustc;
pub mod rustdoc;

// Users can choose to access prefixless lints through `rustc::`, or from the top level.
pub use rustc::*;
