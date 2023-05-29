# Macro invocations

The following is a macro (and related non-macro function) invocation chain that generates attribute
macros for prefixed (`clippy::` and `rustdoc::`) lints. Standard (prefixless, `rustc`) invocation
chain is similar.

```txt
NEW:
macro_rules! allow_prefixed::clippy!
- macro_rules! allow_prefixed::prefixed!
  - proc_macro ::allow_internal::check_that_prefixed_lint_exists!($lint_prefix, $lint_name);
  - generate
    - `deprecated`,
    - `not_anymore`,
    - `not_yet` - all as boolean LITERALS
    -> ONLY within proc macro: `pass_through` = not_anymore || not_yet.

- proc_macro
  ::allow_internal::doc_and_attrib_macro_clippy OR
  ::allow_internal::doc_and_attrib_macro_rustc OR
  ::allow_internal::doc_and_attrib_macro_rustdoc

  injected to the token input of (the above) macro_rules allow_prefixed::prefixed.
    - generate URL & `doc`
      - inject the rest of the given TokenStream (since OR since, until OR nightly)

- proc_macro ::allow_internal::generate_allow_attribute_macro_prefixed!(...);
  - fn ::allow_internal::generate_allow_attribute_macro_from_iter(...)

    - macro_rules! (allow_prefixed::) generate_allow_attribute_macro_internal_prefixed!
      - mac_rul! (allow_prefixed::) generate_allow_attribute_macro_internal_with_given_docs_prefixed
        - #[doc = $doc]
          #[proc_macro_attribute]
          pub fn $new_macro_name(..){..}


macro_rules! allow_prefixed::
- rustdoc($name, $since)
- rustdoc($name, nightly)

- rustc_allowed($name, $since)
- rustc_allowed($name, $since, $until)
- rustc_allowed($name, nightly)
- rustc_warn(...)
- rustc_deny(...)

- clippy($name, $since)
- clippy($name, $since, $until)
- clippy($name, nightly)

- ($prefix, $name, $since)         -> AllowAttribMacroQualities::new_since($since)
  #[rustversion::since(1.54)]

- ($prefix, $name, $since, $until) -> AllowAttribMacroQualities::new_since_until($since, $until)
  #[rustversion::since(1.54)]
  #[rustversion::not(since(1.60))]

OLD:
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
