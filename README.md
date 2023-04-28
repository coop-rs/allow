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

- `use allow::anonymous_parameters as allow_anonymous_params_legacy;`
- `use allow::rustdoc_missing_doc_code_examples as allow_rustdoc_examples_linked;`
- `use allow::rustdoc_missing_doc_code_examples as allow_rustdoc_examples_none_trivial;`
- `use allow::clippy_await_holding_lock as allow_clippy_await_holding_lock;`

Then apply `#[allow_anonymous_params_legacy]`, `#[allow_rustdoc_examples_linked]`,
`#[allow_rustdoc_examples_none_trivial]`, `#allow_clippy_await_holding_lock` (or as you name them)
before your struct/enum/function/variable..., indicating your intention.

Side benefit: Rust would validate the (aliased) names, hence no typos. So you can `grep` or search
for them at anytime. Your team  could have a prelude-like module exporting the aliases.

## Scope

### In scope

- Rust versions 1.45, 1.49.0, 1.52.1, 1.58.1, 1.61.0, 1.69.0 and maybe some, but seemingly **not**
  all, versions in between. See ["Out of scope"](#out-of-scope) below.
- `stable` and `nightly` (but we may need your help with maintenance).
- All in-built `rustc` lints ("standard" with no prefix); `clippy::` & `rustdoc::` lints.
- Clippy: `allow` version `0.1.0` has all Clippy lints supported by Rust 1.45`. The author is adding
  newer lints (and specifying version ranges for lints that have been deprecated/removed later).

### Limited scope

- Supporting Rust below `1.63` (down to `1.45`). If we do use
[`ui_test`](https://github.com/oli-obk/ui_test) crate for testing, we can fully test only from Rust
1.63. Alternatively, could you help implement the negative build tests with
[`trybuild`](https://github.com/dtolnay/trybuild) crate?
  
  Lints (macros) **do** get validated for those lower Rust versions, too. And the tests that require
  Rust `1.63+` cover functionality that is used for lower Rust versions, too. Meaning: This is well
  tested.
- Immediate support for all possible Rust versions since `1.45`. Adding lints (macros), marking them
  as obsolete and removing them based on Rust version is not automated. Mistakes happen. Help report
  them and fix them.

### Out of scope

- Rust versions 1.63, 1.65.0, 1.66.1, 1.67.0, 1.67.1, 1.68.0, 1.68.2 (at least so for the mainstream
  x64 Linux target: `x86_64-unknown-linux-gnu`). Why? Because they don't support some `rustc`
  (standard) lints, even though those lints exist in both some earlier **and** later versions.
  However, some major versions in-between may work. And some older versions, **are** supported. See
  above.
- Lint groups (like `#[allow(unused)]`). Indeed, they do have their place (for example: fast
  prototyping). But they are contrary to the purpose of this crate: To differentiate between the use
  cases of ignoring the same lint.
- Crate level-only ("inner") attributes. They don't work with `#[allow(...)]`, but only with
  `#![allow(...)]` and only at crate level. That means (in general) much fewer repetitions than
  `#[allow(...)]` sprinkled around the code (granular).of these - and even if you do, 

  You can give thumbs up to [rust-lang/rust #54726](https://github.com/rust-lang/rust/issues/54726).
  Suppose it is implemented. However, top level attributes would most likely have to come before any
  `use` aliases, so we wouldn't be able to alias macros generating `#![allow(...)]` anyway,
  unfortunately - no semantic benefit. (We could alias them in other modules or crates, and call
  them without any `use` imports - with a fully qualified path, like
  `#![our_allow_crate::lints::non_ascii_idents_legacy]` or
  `#![crate::lints::non_ascii_idents_third_party]`. To be seen.)
- `Beta` version of `rustc` specifics. `Beta` version incubation is only for 6 weeks. Or, would you
  help maintain this?
- Rust older than `1.45`. If there is high demand, we could potentially support down to 1.31 (needed
  by [`rustversion`](https://crates.io/crates/rustversion) crate.) But then we'd have an ugly and
  more complicated proc macro.
  
  If you would benefit from `allow`, it's most likely when the lints you are suppressing are wide
  spread. Hence, if you choose to refactor the code, couldn't you as well upgrade it to newer Rust?
- Parameterized lints (very few - maybe only in `clippy::`?). If you'd like that, you'll have some
  low level proc macro work.
- Custom lints (such as [with
Dylint](https://blog.trailofbits.com/2021/11/09/write-rust-lints-without-forking-clippy/)). Possible
in principle - will you commit to maintain it?

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
powerhouses, which parse & quote the code that a macro can inject. However, their power comes with a
cost: build time.

But, not so for these macros. `allow` does not parse the new (generated) code into a `TokenStream`
(before it's injected where you use it). Instead, it composes it (through the proc_macro API).

The tests do have many more dependencies (if we continue to use `ui_test` - as `trybuild` may be
much faster). So don't judge its speed by `cargo test`, but by `cargo build`. (Also, some tests
don't run on Windows - see See [CONTRIBUTING.md](CONTRIBUTING.md#testing).)

## Crates, crates.io and GIT

This project consists of three crates. Two of them will soon be (TODO update) on crates.io: `allow`
and `allow_internal`. The third one, `allow_tests`, is not on crates.io, and it is for testing only.
(TODO If we continue with `ui_test`, consider a 4th crate, for testing only, so we check for Rust
below 1.63 more.)

They are all under the same [GIT repo](https://github.com/coop-rs/allow), which simplifies
maintenance.

## Help, please

- Compilation **failure** tests, preferably with [ui_test](https://github.com/oli-obk/ui_test), or
  with [trybuild](https://github.com/dtolnay/trybuild) (but NOT with `compilertest-rs`). Validate
  that incorrect lint names make the macros fail. See [ui_test
  #57](https://github.com/oli-obk/ui_test/issues/57) or [trybuild
  #235](https://github.com/dtolnay/trybuild/issues/235).
- Set up CI with GitHub Actions.

## Reporting Issues

See [coop-rs/allow > issues](https://github.com/coop-rs/allow/issues).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Allow project is distributed under the terms of both the MIT license and the Apache License (Version
2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for
details.
