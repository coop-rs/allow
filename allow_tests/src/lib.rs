#![forbid(unknown_lints)]

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
///    that `#[allow::...]` will NOT apply - because it will be "overriden" by the previous
///    (mistaken) `#[deny(...)]`.
/// 3. The rest of the code will not get that lint checked (of course).

// NO need to mark functions as `#[test]`, since all we check is compilation. But we do invoke it
// from a (separate) `#[test]` function, for peace of mind.

//#[allow(clippy::oh_dear)]
//#[allow::clippy_clbu]
pub fn unused() {
    //#[allow::clippy_clbu]
    //#[allow(clippy::hohoho)]
    fn f() {
        std::hint::black_box(());
    }
    f();
    _unused();
}

// The following two together have triggered an ICE.
//#[allow::unused]
//#[allow::unused_braces]

// The following two together trigger an ICE.
//#[allow::array_into_iter]
//#[allow::bufo]
#[allow::clippy_assign_ops]

// #[allow::clippy_assign_ops]

//#[allow::clippy_almost_swapped] // <--- problem
//#[allow::clippy_assign_ops]
//#[allow::clippy_clbu]
fn _unused() {}

//#[allow::unused]
//fn unused2() {}
