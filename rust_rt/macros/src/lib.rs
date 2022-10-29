#![allow(warnings)]

extern crate proc_macro;
use proc_macro2::TokenStream;

#[proc_macro]
pub fn java(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ts2 = TokenStream::from(item);
    let mut iterator = ts2.into_iter().peekable();

    use proc_macro2::TokenTree::*;

    loop {
        match iterator.next() {
            Some(Punct(punct)) => {
                let punct = punct.as_char();
            }
            Some(Group(group)) => {
                let group_output = group.stream();
                match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => {}
                    proc_macro2::Delimiter::Brace => {}
                    proc_macro2::Delimiter::Bracket => {}
                    proc_macro2::Delimiter::None => {}
                }
            }
            Some(Ident(ident)) => {}
            Some(Literal(literal)) => {}
            None => break,
        }
    }

    "fn answer() -> u32 { 42 }".parse().unwrap()
}
