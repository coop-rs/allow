//! `rustdoc` lint macros, re-exported from [`allow_prefixed`](../allow_prefixed) crate.

// MAINTAINERS: See rustc.rs

#[rustversion::since(1.52)]
#[rustfmt::skip]
pub use allow_prefixed::{
    rustdoc_broken_intra_doc_links as broken_intra_doc_links,
    //rustdoc_private_intra_doc_links as private_intra_doc_links,
    //rustdoc_missing_crate_level_docs as missing_crate_level_docs
};
#[rustversion::nightly]
#[rustfmt::skip]
pub use allow_prefixed::{
    //rustdoc_missing_doc_code_examples as missing_doc_code_examples,
};
#[rustversion::since(1.52)]
#[rustfmt::skip]
pub use allow_prefixed::{
    /*rustdoc_invalid_codeblock_attributes as invalid_codeblock_attributes,
    rustdoc_invalid_html_tags as invalid_html_tags,
    rustdoc_invalid_rust_codeblocks as invalid_rust_codeblocks,
    rustdoc_bare_urls as bare_urls,
    rustdoc_unescaped_backticks as unescaped_backticks,*/
};
