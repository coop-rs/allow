# allow_internal

See the parent [`allow_prefixed`](https://crates.io/crates/allow_prefixed) crate, or their shared
[GIT repo](https://github.com/coop-rs/allow).

`allow_internal` is internal - to be used by
[`allow_prefixed`](https://crates.io/crates/allow_prefixed) and (indirectly) by
[`allow`](https://crates.io/crates/allow) crates only.

It could be simplified, or even eliminated (replaced + integrated into `allow_prefixed` itself) if
we used [paste](https://crates.io/crates/paste) crate here (as we do in `allow` crate itself). It
would slow down the generated macros themselves - but by how much? (`allow` crate does use `paste`
crate, but it's not for a fixed number of times.) Up to the future maintainers if they want to
benchmark it.
