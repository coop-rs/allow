// Simplified version of ../allow_prefixed/build.rs.

#[rustversion::nightly]
fn emit_unstable_feature() {
    println!("cargo:rustc-cfg=unstable_feature");
}
#[rustversion::not(nightly)]
fn emit_unstable_feature() {}

fn main() {
    emit_unstable_feature();
}
