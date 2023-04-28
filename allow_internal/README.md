# allow_internal

See the parent [`allow`](https://crates.io/crates/allow) crate, or their shared [GIT
repo](https://github.com/coop-rs/allow).

`allow_internal` is internal - to be used by [allow](https://crates.io/crates/allow) crate only.

This could be simplified, or even eliminated (replaced + integrated into `allow` itself) if we used
[paste](https://crates.io/crates/paste) crate. But that would

- increase the built time, and
- complicate versioning.

Up to the future maintainers.
