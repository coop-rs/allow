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
}
