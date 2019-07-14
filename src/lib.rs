extern crate proc_macro;
use proc_macro::*;
use std::iter::FromIterator;
use std::collections::{BTreeMap, BTreeSet};

fn ident(x: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(x, Span::call_site()))
}

#[derive(Debug)]
struct Collector {
    dict: BTreeMap<String, BTreeSet<String>>, // the collected enums and their variants
    current: Option<String>, // current enum waiting to get variant
    ncolon: usize // current enum will get a variant after two colons
}

impl Collector {
    fn new(names: impl Iterator<Item=String>) -> Self {
        Collector {
            dict: BTreeMap::from_iter(names.map(|name| (name, BTreeSet::new()))),
            current: None,
            ncolon: 0
        }
    }

    fn feed(&mut self, token: TokenTree) {
        // println!("{:?}", token);
        // println!("{:?}", self);

        match token {
            TokenTree::Ident(i) => {
                let name = i.to_string();

                if let Some(current) = &self.current {
                    if self.ncolon == 2 { // we got a variant
                        self.dict.get_mut(current).expect("bug").insert(name);
                        return self.reset()
                    }
                    self.reset()
                }

                if self.dict.contains_key(&name) {
                    self.current = Some(i.to_string());
                }
            },
            TokenTree::Punct(ref i) if i.as_char() == ':' && self.current.is_some() => {
                match self.ncolon {
                    0 if i.spacing() == Spacing::Joint => self.ncolon = 1,
                    1 if i.spacing() == Spacing::Alone => self.ncolon = 2,
                    _ => self.reset()
                }
            },
            TokenTree::Group(g) => {
                self.reset();

                for token in g.stream() {
                    self.feed(token)
                }
            },
            _ => self.reset()
        }
    }

    fn reset(&mut self) {
        self.current = None;
        self.ncolon = 0;
    }
}

#[proc_macro_attribute]
pub fn collect_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Vec<_> = args.into_iter().map(|x| {
        if let TokenTree::Ident(i) = x {
            i.to_string()
        } else {
            panic!("enum names should be identifiers")
        }
    }).collect();

    let mut collector = Collector::new(args.into_iter());
    for token in input.clone() {
        collector.feed(token)
    }

    let mut tokens = vec![];

    for (name, variants) in collector.dict {
        tokens.extend("#[derive(Debug, Eq, PartialEq)]".parse::<TokenStream>().unwrap());
        tokens.push(ident("pub"));
        tokens.push(ident("enum"));
        tokens.push(ident(&name));
        tokens.push(TokenTree::Group(Group::new(Delimiter::Brace, variants.iter().flat_map(|x| vec![ident(x), TokenTree::Punct(Punct::new(',', Spacing::Alone))].into_iter()).collect())));
    }

    tokens.extend(input);
    tokens.into_iter().collect()
}

