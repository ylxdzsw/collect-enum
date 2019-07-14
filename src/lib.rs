extern crate proc_macro;
use proc_macro::*;

fn ident(x: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(x, Span::call_site()))
}

#[proc_macro_attribute]
pub fn collect_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Vec<_> = args.into_iter().map(|x| {
        if let TokenTree::Ident(i) = x {
            i
        } else {
            panic!("fuck")
        }
    }).collect();

    let mut def: Vec<TokenTree> = vec![];
    def.extend("#[derive(Debug)]".parse::<TokenStream>().unwrap());
    def.push(TokenTree::Ident(Ident::new("enum", Span::call_site())));
    def.push(TokenTree::Ident(args[0].clone()));

    let mut fields: Vec<TokenTree> = vec![];
    fields.push(TokenTree::Ident(Ident::new("Fuck", Span::call_site())));

    def.push(TokenTree::Group(Group::new(Delimiter::Brace, fields.into_iter().collect())));

    def.extend(input);
    def.into_iter().collect()
}

