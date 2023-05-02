// Simplified version of ../allow_prefixed/build.rs.

#[rustversion::nightly]
fn emit_unstable_feature() {
    println!("cargo:rustc-cfg=unstable_feature");
}
#[rustversion::not(nightly)]
fn emit_unstable_feature() {}

#[rustversion::since(1.52)]
fn emit_has_rustdoc_lints() {
    println!("cargo:rustc-cfg=has_rustdoc_lints");
}
#[rustversion::not(since(1.52))]
fn emit_has_rustdoc_lints() {}

// Whether this Rust version supports `#![deny(invalid_doc_attributes)]` and similar. The exact
// earliest version is not mentioned at https://releases.rs, but it seems to be 1.54.
#[rustversion::since(1.54)]
fn emit_can_check_doc_attributes() {
    println!("cargo:rustc-cfg=can_check_doc_attributes");
}
#[rustversion::not(since(1.54))]
fn emit_can_check_doc_attributes() {}

fn main() {
    emit_unstable_feature();
    emit_has_rustdoc_lints();
    emit_can_check_doc_attributes();
}
