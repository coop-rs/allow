#![feature(thread_local)]

use once_cell::unsync::OnceCell;
use proc_macro::{Literal, TokenStream, TokenTree};

#[thread_local]
// static ONE: TokenStream = TokenStream::from(...) failed, and rustc was suggesting Lazy::new(...).
// However, that would involve a Mutex.
//
// See https://docs.rs/once_cell/latest/once_cell/unsync/struct.OnceCell.html
static ONE: OnceCell<TokenStream> = OnceCell::new();

#[proc_macro_attribute]
pub fn unused(_given_attrs: TokenStream, _item: TokenStream) -> TokenStream {
    ONE.get_or_init(|| TokenStream::from(TokenTree::Literal(Literal::i32_unsuffixed(1))))
        .clone()
}
