# Macro invocations

The following is a macro (and related non-macro function) invocation chain that generates attribute
macros for prefixed (`clippy::` and `rustdoc::`) lints. Standard (prefixless, `rustc`) invocation
chain is similar.

```txt
macro_rules! allow_prefixed::prefixed_lint!
- proc_macro ::allow_internal::check_that_prefixed_lint_exists!($lint_prefix, $lint_name);
- proc_macro ::allow_internal::generate_allow_attribute_macro_prefixed!(...);
  - fn ::allow_internal::generate_allow_attribute_macro_from_iter(...)
    - macro_rules! (allow_prefixed::) generate_allow_attribute_macro_internal_prefixed!
      - mac_rul! (allow_prefixed::) generate_allow_attribute_macro_internal_with_given_docs_prefixed
        - #[doc = $doc]
          #[proc_macro_attribute]
          pub fn $new_macro_name(..){..}
```
