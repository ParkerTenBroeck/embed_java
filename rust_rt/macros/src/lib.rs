#![allow(warnings)]

extern crate proc_macro;
use proc_macro2::{TokenStream, Span};

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


#[proc_macro]
pub fn embed_java(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ts2 = TokenStream::from(item);
    // let mut iterator = ts2.into_iter().peekable();

    let (class_name, code) = reconstruct(ts2);

    use proc_macro2::TokenTree::*;
    
    // include_bytes!()
    // panic!("{:?}", std::env::var_os("OUT_DIR"));
    let mut string = String::new();
    for (k,v) in std::env::vars_os(){
        string.push_str(k.to_str().unwrap());
        string.push_str(": ");
        string.push_str(v.to_str().unwrap());
        string.push('\n');
    }
    // let v = std::env::var_os("CARGO_MANIFEST_DIR").expect("bruh");
    // let p = v.to_str().expect("hellp");
    format!("panic!(\"{string}\")").parse().expect("asd")
    // "fn answer() -> u32 { 42 }".parse().unwrap()
}


fn reconstruct(stream: TokenStream) -> (String, String){
    let mut out = String::new();
    let mut class_name = String::new();

    let mut iter = stream.into_iter().peekable();

    while let Some(next) = iter.next(){
        match next{
            proc_macro2::TokenTree::Group(_) => todo!(),
            proc_macro2::TokenTree::Ident(_) => todo!(),
            proc_macro2::TokenTree::Punct(_) => todo!(),
            proc_macro2::TokenTree::Literal(_) => todo!(),
        }
    }

    ("".into(), "".into())
}