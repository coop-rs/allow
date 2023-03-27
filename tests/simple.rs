#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
fn _unused() -> i32 {
    #[allows::unused]
    0
}
