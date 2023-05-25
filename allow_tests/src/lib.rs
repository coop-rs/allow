#![forbid(unknown_lints)]

use allow_prefixed::unused_variables as allow_unused_variables;
use allow_prefixed::rustdoc_broken_intra_doc_links as broken_intra_doc_links_legacy;

//use allow_prefixed::clippy_await_holding_lock as await_holding_lock_i_know_better;

//#[warn(renamed_and_removed_lints)]
#[test]
fn test_unused() {
    unused();
}

/// BEWARE: If you forget the exclamation mark, like:
///
/// `#[deny(unused)]`
///
/// instead of
///
/// `#![deny(unused)]`
///
/// then:
/// 1. such a #[deny(...)] applies only to the first item (function), and
/// 2. even worse: Even if you do add an appropriate `#[allow::...]` in front of that first item,
///    that `#[allow::...]` will NOT apply - because it will be "overridden" by the previous
///    (mistaken) `#[deny(...)]`.
/// 3. The rest of the code will not get that lint checked (of course).

// NO need to mark functions as `#[test]`, since all we check is compilation. But we do invoke it
// from a (separate) `#[test]` function, for peace of mind.

//#[allow(clippy::oh_dear)]
pub fn unused() {
    //#[allow(clippy::oh_dear2)]
    fn f() {
        std::hint::black_box(());
    }
    f();
    _unused();
}

//#[allow_prefixed::unused_braces]
//#[allow::clippy_await_holding_lock]
//#[await_holding_lock_i_know_better]
#[allow_prefixed::rustdoc_broken_intra_doc_links]
#[allow::rustdoc::broken_intra_doc_links]
#[broken_intra_doc_links_legacy]
#[allow_unused_variables]
//#[allow_prefixed::sing_le]
fn _unused() {}
