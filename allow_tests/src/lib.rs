#![deny(unknown_lints)]
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
/// 2. even worse: Even if you do add an appropriate `#[allows::...]` in front of that first item,
///    that `#[allows::...]` will NOT apply - because it will be "overriden" by the previous
///    (mistaken) `#[deny(...)]`.
/// 3. The rest of the code will not get that lint checked (of course).

// NO need to mark functions as `#[test]`, since all we check is compilation. But we do invoke it
// from a (separate) `#[test]` function, for peace of mind.

//#[allow(clippy::oh_dear)]
//#[allows::clippy_clbu]
pub fn unused() {
    #![deny(unknown_lints)]
    //#[allows::clippy_clbu]
    //#[allow(clippy::hohoho)]
    fn f() {
        std::hint::black_box(());
    }
    f();
    _unused();
}

// The following two together have triggered an ICE.
//#[allows::unused]
//#[allows::unused_braces]

// The following two together trigger an ICE.
//#[allows::array_into_iter]
//#[allows::bufo]
#[allows::clippy_assign_ops]

// #[allows::clippy_assign_ops]

//#[allows::clippy_almost_swapped] // <--- problem
//#[allows::clippy_assign_ops]
//#[allows::clippy_clbu]
fn _unused() {}

//#[allows::unused]
//fn unused2() {}
