// Simplified version of ../allow/build.rs and ../allow_prefixed/build.rs.

#[rustversion::since(1.52)]
fn emit_has_rustdoc_lints() {
    println!("cargo:rustc-cfg=has_rustdoc_lints");
}

#[rustversion::not(since(1.52))]
fn emit_has_rustdoc_lints() {}

fn main() {
    emit_has_rustdoc_lints();
}
