use proc_macro::{Literal, TokenStream, TokenTree};

thread_local! {
    static ONE: TokenStream = TokenStream::from(TokenTree::Literal(Literal::i32_unsuffixed(1)));
}

#[proc_macro_attribute]
pub fn unused(_given_attrs: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::from_iter([ONE.with(Clone::clone)])
}
