// We use both
// -`rustversion` both in build.rs and in lib.rs for conditional compilation, and
// - `cargo_toolchain` for detecting floating toolchain (stable/beta/nightly) when used with `rustup`. See also https://github.com/dtolnay/rustversion/issues/39.
//
// Thanks for https://github.com/dtolnay/rustversion/issues/8
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

// Whether some built-in attributes (primarily `#[doc]`) can invoke macros. Since
// https://releases.rs/docs/1.54.0/#language.
#[rustversion::since(1.54)]
fn emit_attributes_can_invoke_macros() {
    println!("cargo:rustc-cfg=attributes_can_invoke_macros");
}
#[rustversion::not(since(1.54))]
fn emit_attributes_can_invoke_macros() {}

fn emit_floating_toolchain() {
    let toolchain = cargo_toolchain::get_active_toolchain();
    if let Ok(toolchain) = toolchain {
        if ["stable", "beta", "nightly"].contains(&toolchain.as_str()) {
            println!("cargo:rustc-cfg=floating_toolchain");
        }
    }
    //@TODO print the error, if invoked with some flag or feature.
    //
    // Otherwise: The toolchain is either at a fixed version/date, or unknown. (TODO if we implement
    // the flag/feature: Since the user didn't pass a flag/feature), we are permissive and we let
    // the user's code build, even if they use macros not available for the newer versions.
}

fn main() {
    emit_unstable_feature();
    emit_floating_toolchain();
    emit_has_rustdoc_lints();
    emit_can_check_doc_attributes();
    emit_attributes_can_invoke_macros();
}
