
# Related issues - give thumbs up, please
- [`thread_local!`](with proc macros https://github.com/rust-lang/rust/issues/66003)
- [`#[thread_local]`](https://github.com/rust-lang/rust/issues/29594)
- [Partial stabilization of once_cell](https://github.com/rust-lang/rust/pull/105587)
- [standard lazy types](https://github.com/rust-lang/rfcs/pull/2788)


# Efficient proc macros

This does use (many) procedural macros (specifically: [attribute macros](https://doc.rust-lang.org/nightly/book/ch19-06-macros.html#attribute-like-macros)). Yes, proc macros often slow compilation down. But not so for these macros.

How come? Most proc macro implementations use [syn](https://crates.io/crates/syn). That's a powerhouse, which parses the code that a macro can inject. However, its power comes at cost.

 usually 
We do not parsing the new code into a TokenStream (before it's injected where you use it), we compose 

# Scope
## In scope

## Out of scope

# Help, please
- add missing tests (quite many for one person to understand)
- set up CI with GitHub Actions
-  with Alpine Linux (or some other light weight way)
-  especially on targets that don't support #[thread_local]


## Reprting Issues

See <https://github.com/coop-rs/allows/issues>.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Allows is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
