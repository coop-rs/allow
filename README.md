# Purpose

Help make

# Scope

## In scope

- All in-built Rust lints (standard, `clippy::`, `rustdoc::`).
- Both `stable` and `nightly` versions of Rust.

## Out of scope

- `Beta` version of `rustc` specifics (unless you help maintain it).
- Custom lints (such as [with
Dylint](https://blog.trailofbits.com/2021/11/09/write-rust-lints-without-forking-clippy/)).

# Related issues - give thumbs up, please

- [`thread_local!` failing for proc macros](https://github.com/rust-lang/rust/issues/66003)
- [`#[thread_local]` (attribute) stabilization](https://github.com/rust-lang/rust/issues/29594)
- [Partial stabilization of once_cell](https://github.com/rust-lang/rust/pull/105587)
- [standard lazy types](https://github.com/rust-lang/rfcs/pull/2788)


# Efficient proc macros

This does use (many) procedural macros (specifically: [attribute
macros](https://doc.rust-lang.org/nightly/book/ch19-06-macros.html#attribute-like-macros)). One per lint.

Yes, proc macros often slow compilation down. But not so for these macros.

How come? Most proc macro implementations use [syn](https://crates.io/crates/syn) and
[quote](https://crates.io/crates/syn) crates. Those are powerhouses, which parse & quote the code
that a macro can inject. However, their power comes at cost: build time.

However, `allows` does not parse the new (generated) code into a TokenStream (before it's injected
where you use it). Instead, it composes it (through the proc_macro API).

# Help, please

- add missing tests (quite many for one person to understand)
- set up CI with GitHub Actions
-  with Alpine Linux (or some other light weight way)
-  especially on targets that don't support #[thread_local]

## Reporting Issues

See <https://github.com/coop-rs/allows/issues>.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Allows is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for
details.
