# Purpose

Aliases for `#[allow(...)]` local lint permissions/suppressions. You can have multiple aliases, each
with its name - semantic.

## The problem

Let's say you `#[allow(...)]` the same standard `rustc` (prefixless) lint, or `clippy::` or
`rustdoc::` lint, at many places in your code. Even though it's the same lint, you may have
different reasons for allowing it. However, `#[allow(...)]` doesn't carry your intention.

Unfortunately, we can't alias lints with `use` either. (Why? Because lint names are not (proc)
macros, but special compiler symbols.) Of course, you could add a comment, but that's haphazard.

This crate defines one attribute macro per each lint (other than crate-level only lints, more
below). Those macros don't take any attribute parameters. They inject `#[allow(lint-name-here)]` in
front of your code.

You can import same macros as many times under as many names you need, for example:

- `use lint_allows::rustdoc_missing_doc_code_examples as examples_linked;`
- `use lint_allows::rustdoc_missing_doc_code_examples as examples_none_trivial;`

Then apply #[examples_linked], #[examples_none_trivial], or as you name them, indicating your
intention. (Side not: Handling `rustdoc::` and `clippy::` is not implemented yet. Only standard
lints for now.)

Side benefit: Rust would validate the (aliased) names, hence no typos - so you can `grep` or search
for them at anytime. Your team  could have a prelude-like module exporting the aliases.

## Scope

### In scope

- All in-built Rust lints (standard `rustc` lints with no prefix - done; `clippy::` & `rustdoc::` -
  TODO see ["Help, please"](#help-please)).
- Both `stable` and `nightly` versions of Rust.

### Out of scope

- Lint groups (like `#[allow(unused)]`). That's contrary to the purpose of this crate: To
  differentiate between the use cases of the same lint.
- Crate level-only ("inner") attributes. They don't work with `#[allow(...)]`, but only with
  `#![allow(...)]` and only at crate level. Chances are you don't have many repetitions of these.
  You can give thumbs up to [rust-lang/rust #54726](https://github.com/rust-lang/rust/issues/54726),
  but even once that feature is implemented, top level attributes have to come before an `use`
  aliases, so you ouldn't alias calls to macros generating `#![allow(...)]` anyway - so no semantic
  benefit.
- `Beta` version of `rustc` specifics ( it's only for 6 weeks). Or, would you help maintain this?
- Custom lints (such as [with
Dylint](https://blog.trailofbits.com/2021/11/09/write-rust-lints-without-forking-clippy/)).

## Related issues - give thumbs up, please

- [`thread_local!` failing for proc macros](https://github.com/rust-lang/rust/issues/66003)
- [`#[thread_local]` (attribute) stabilization](https://github.com/rust-lang/rust/issues/29594)
- [Partial stabilization of once_cell](https://github.com/rust-lang/rust/pull/105587)
- [standard lazy types](https://github.com/rust-lang/rfcs/pull/2788)
- [`concat_idents`](https://github.com/rust-lang/rust/issues/29599)

## Efficient proc macros

This does use procedural macros (specifically: [attribute
macros](https://doc.rust-lang.org/nightly/book/ch19-06-macros.html#attribute-like-macros)). Many:
One per lint.

Yes, proc macros often slow compilation down. Proc macros usually use
[syn](https://crates.io/crates/syn) and [quote](https://crates.io/crates/syn) crates. Those are
powerhouses, which parse & quote the code that a macro can inject. However, their power comes at
cost: build time.

But, not so for these macros. This does not parse the new (generated) code into a TokenStream
(before it's injected where you use it). Instead, it composes it (through the proc_macro API).

(The tests do have many more dependencies. So don't judge its speed by `cargo test`, but by `cargo
build`.)

## Help, please

- Compilation **failure** tests, preferrably with [ui_test](https://github.com/oli-obk/ui_test), or
  with [trybuild](https://github.com/dtolnay/trybuild) - but NOT with `compilertest-rs`). Validate
  that incorrect lint names make the macros fail. See [ui_test
  #57](https://github.com/oli-obk/ui_test/issues/57) or [trybuild
  #235](https://github.com/dtolnay/trybuild/issues/235).
- set up CI with GitHub Actions

## Reporting Issues

See [coop-rs/allows > issues](https://github.com/coop-rs/allows/issues).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Testing

Testing is not possible on Windows (to be determined). To minimize human mistakes, tests need to be
built on a filesystem that supports symlinks. Of course, the actual crates themselves are
platform-independent.

## License

Allows is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for
details.
