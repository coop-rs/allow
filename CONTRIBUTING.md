## Testing

Testing is not possible on Windows (to be determined). To minimize human mistakes, tests need to be
built on a filesystem that supports symlinks (for
[`allow_tests/tests/internal_checks/incorrect_lint/src/wrapper_macros.rs`](allow_tests/tests/internal_checks/incorrect_lint/src/wrapper_macros.rs)).

## Rust and cargo versions

- Rust versions below 1.68 (like 1.67) that support `"protocol = sparse"` setting in
  `[registries.crates-io]` part of `~/cargo/config`. It seems that the older versions were
  incompatible with the stabilized format (1.68+). So if you enabled the `sparse` with 1.68+, but
  then you switch to 1.67 (or similar), you may get an error. Workaround: remove/comment out that
  `sparse` setting. See also [rust-lang/cargo
  #12058](https://github.com/rust-lang/cargo/issues/12058).

# License of contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed (as per
<README.md>), without any additional terms or conditions.
