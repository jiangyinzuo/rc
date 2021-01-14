use syn::{DeriveInput, DataEnum, Expr};
use quote::*;
use proc_macro2::Ident;

const DISABLED: &str = "disabled";

fn add_kv(key: &mut Vec<String>, value: &mut Vec<Ident>, expr: &Expr, ident: Ident) {
    let lit = syn::parse2::<syn::LitStr>(expr.to_token_stream()).unwrap();
    key.push(lit.value());
    value.push(ident);
}

pub fn add_from_str(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        vis,
    } = ast;

    let mut key: Vec<String> = vec![];
    let mut value: Vec<Ident> = vec![];

    if let syn::Data::Enum(DataEnum { variants, .. }) = data {
        for v in variants {
            if v.attrs.is_empty() {
                key.push(v.ident.to_string().to_lowercase());
                value.push(format_ident!("{}", v.ident));
            } else {
                for attr in v.attrs {
                    let path = attr.path.get_ident().unwrap();
                    if path != DISABLED {
                        let tks = attr.tokens;
                        if let Ok(res) = syn::parse2::<syn::ExprTuple>(tks.clone()) {
                            for expr in res.elems.iter() {
                                add_kv(&mut key, &mut value, expr, v.ident.clone());
                            }
                        } else if let Ok(res) = syn::parse2::<syn::ExprParen>(tks) {
                            let expr = res.expr.as_ref();
                            add_kv(&mut key, &mut value, expr, v.ident.clone());
                        }
                    }
                }
            }
        }
    }
    quote! (
        impl std::str::FromStr for #ident {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #key => Ok(Self::#value), )*
                    _ => Err(()),
                }
            }
        }
    )
}

#[test]
fn add_from_str_test() {
    use quote::quote;

    let tokens = quote! {
        enum Color<'a> {
            #[disabled]
            Red(&'a str),
            Yellow,
            #[value("+")]
            Orange
        }
    };
    let tokens = add_from_str(syn::parse2(tokens).unwrap());
    println!("{}", tokens);
}
