# allow_internal

Internal proc macros for crates [`allow_prefixed`](https://crates.io/crates/allow_prefixed) and
[`allow`](https://crates.io/crates/allow) only.

<!-- TODO REMOVE ONCE CLEAR:
It could be simplified, or even eliminated (replaced + integrated into `allow_prefixed` itself) if
we used [paste](https://crates.io/crates/paste) crate here (as we do in `allow` crate itself). It
would slow down the generated macros themselves - but by how much? (`allow` crate does use `paste`
crate, but it's not for a fixed number of times.) Up to the future maintainers if they want to
benchmark it.
-->