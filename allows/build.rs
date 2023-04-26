// Thanks to https://github.com/dtolnay/rustversion/issues/8

#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=unstable_feature");
}

#[rustversion::not(nightly)]
fn main() {}
