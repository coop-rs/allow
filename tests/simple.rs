#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

#[test]
fn test_unused() {
    _unused();
}

fn _unused() -> i32 {
    #[allows::unused]
    0
}
