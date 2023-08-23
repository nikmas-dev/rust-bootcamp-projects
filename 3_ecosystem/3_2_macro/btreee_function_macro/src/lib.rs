extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, Token};

struct KeyValue {
    key: Expr,
    value: Expr,
}

impl Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=>]>()?;
        let value = input.parse()?;
        Ok(KeyValue { key, value })
    }
}

struct KeyValuePairs {
    pairs: Punctuated<KeyValue, Token![,]>,
}

impl Parse for KeyValuePairs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pairs = Punctuated::<KeyValue, Token![,]>::parse_terminated(input)?;
        Ok(KeyValuePairs { pairs })
    }
}

#[proc_macro]
pub fn btreemap(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as KeyValuePairs);

    let insertions = input.pairs.iter().map(|KeyValue { key, value }| {
        quote! {
            map.insert(#key, #value);
        }
    });

    let expanded = quote! {
        {
            let mut map = ::std::collections::BTreeMap::new();
            #(#insertions)*
            map
        }
    };

    expanded.into()
}
